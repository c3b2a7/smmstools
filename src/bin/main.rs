use clap::{Parser, Subcommand, ValueHint};
use smmstools::client::Client;
use smmstools::Result;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"), version, author, about = "A lightweight SM.MS (https://sm.ms/) tools")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,

    /// API token of sm.ms, visit https://sm.ms/home/apitoken to get your token
    #[arg(short = 't', long, value_name = "SMMS_TOKEN", env = "SMMS_TOKEN")]
    token: String,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Get user profile
    Profile,

    /// Upload image(s) to sm.ms
    #[command(arg_required_else_help = true)]
    Upload {
        /// Path of image(s) to upload
        #[arg(value_name = "PATH", required = true, value_hint = ValueHint::FilePath)]
        image_paths: Vec<String>,
    },

    /// Delete image(s)
    #[command(arg_required_else_help = true)]
    Delete {
        /// Hash of image(s) to delete
        #[arg(value_name = "HASH", required = true)]
        hashes: Vec<String>,
    },

    /// Get upload history
    History {
        #[arg(long, default_value_t = 1)]
        page: usize,
    },
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let cli = Cli::parse();

    let uploader = Arc::new(Client::new(cli.token));
    match cli.cmd {
        Command::Profile => println!("{}", uploader.get_profile().await?),
        Command::Upload { image_paths } => upload_and_print_url(uploader, image_paths).await,
        Command::Delete { hashes } => delete_and_print_error(&uploader, hashes).await,
        Command::History { page } => println!(
            "{}",
            serde_json::to_string_pretty(&uploader.get_upload_history(page).await?)?
        ),
    }

    Ok(())
}

async fn upload_and_print_url(uploader: Arc<Client>, image_paths: Vec<String>) {
    let capacity = image_paths.len();
    let (tx, mut rx) = mpsc::channel(capacity);
    let mut ret = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        ret.push(None);
    }

    for (index, path) in image_paths.into_iter().enumerate() {
        let tx = tx.clone();
        let uploader = uploader.clone();
        tokio::spawn(async move {
            let upload_result = uploader.upload(path).await;
            tx.send((index, upload_result)).await
        });
    }

    drop(tx);
    let mut next_print_index = 0;
    while let Some((index, url)) = rx.recv().await {
        ret[index] = Some(url);
        while let Some(Some(upload_result)) = ret.get(next_print_index) {
            match upload_result {
                Ok(data) => {
                    println!("{}", data.url);
                }
                Err(err) => {
                    eprintln!("Error: {err}");
                }
            }
            next_print_index += 1;
        }
    }
}

async fn delete_and_print_error(uploader: &Arc<Client>, hashes: Vec<String>) {
    let (tx, mut rx) = mpsc::channel(hashes.len());
    for (index, hash) in hashes.into_iter().enumerate() {
        let tx = tx.clone();
        let uploader = uploader.clone();
        tokio::spawn(async move {
            let ret = uploader.delete(hash).await;
            tx.send((index, ret)).await
        });
    }
    drop(tx);
    while let Some((_, Err(err))) = rx.recv().await {
        eprintln!("Error: {err}")
    }
}
