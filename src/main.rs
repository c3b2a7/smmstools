use std::{env, process};
use std::error::Error;
use std::ops::Not;

use reqwest::{blocking, header};
use reqwest::blocking::multipart::Form;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SmmsV2 {
    success: bool,
    code: String,
    message: String,
    data: Option<Data>,
    // repeated images fallback
    images: Option<String>,
    // RequestId: String,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    file_id: u32,
    width: u32,
    height: u32,
    filename: String,
    storename: String,
    size: u64,
    path: String,
    hash: String,
    url: String,
    delete: String,
    page: String,
}

#[derive(Debug)]
pub struct Uploader<'a> {
    token: &'a str,
    paths: &'a [String],
}

impl<'a> Uploader<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Uploader<'a>, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let token = &args[1];
        let paths = &args[2..];
        Ok(Uploader { token, paths })
    }

    pub fn token(&self) -> &str {
        self.token
    }

    pub fn paths(&self) -> &[String] {
        self.paths
    }

    pub fn exec(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut _vec = Vec::new();
        let client = blocking::Client::new();
        for path in self.paths {
            let v2_response = client.post("https://sm.ms/api/v2/upload")
                .header(header::AUTHORIZATION, self.token)
                .multipart(Form::new().file("smfile", path)?)
                .send()?
                .json::<SmmsV2>()?;
            if v2_response.success.not() && v2_response.images.is_some() {
                _vec.push(v2_response.images.unwrap())
            } else if v2_response.success {
                _vec.push(v2_response.data.unwrap().url)
            }
        }
        Ok(_vec)
    }
}


fn main() {
    Uploader::new(&env::args().collect()).map_or_else(
        |err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }, |uploader| {
            match uploader.exec() {
                Ok(urls) => for x in urls {
                    println!("{}", x);
                },
                Err(err) => println!("{}", err),
            }
        },
    )
}
