use crate::client::api;
use crate::client::response::{Base, HistoryRecord, Profile, UploadResult};
use crate::{Error, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::multipart::{Form, Part};
use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;
use tokio::fs::File;

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new<T: AsRef<str>>(token: T) -> Client {
        Self {
            client: build_client_with_auth_header(token),
        }
    }

    pub async fn get_profile(&self) -> Result<Profile> {
        let response = self
            .client
            .post(api::PROFILE_API)
            .send()
            .await?
            .json::<Base<Profile>>()
            .await?;

        Self::success_or_else(response, |response| Error::NotSuccess {
            code: response.code,
            message: response.message,
        })
    }

    pub async fn upload<T: AsRef<Path>>(&self, path: T) -> Result<UploadResult> {
        let form = Form::new()
            .text("format", "json")
            .part("smfile", create_part_from_file(path).await?);
        let response = self
            .client
            .post(api::UPLOAD_API)
            .multipart(form)
            .send()
            .await?
            .json::<Base<UploadResult>>()
            .await?;

        Self::success_or_else(response, |base| match base.exists_image {
            Some(exists_at) => Error::ImageRepeated {
                request_id: base.request_id,
                exists_image_url: exists_at,
            },
            None => Error::NotSuccess {
                code: base.code,
                message: base.message,
            },
        })
    }

    pub async fn delete<T>(&self, hash: T) -> Result<()>
    where
        T: AsRef<str> + Display,
    {
        let response = self
            .client
            .get(format!("{}/{hash}", api::DELETE_API))
            .send()
            .await?
            .json::<Base<Vec<()>>>()
            .await?;

        Self::success_or_else(response, |base| Error::NotSuccess {
            code: base.code,
            message: base.message,
        })
        .map(|_| ())
    }

    pub async fn get_upload_history(&self, page: usize) -> Result<Vec<HistoryRecord>> {
        let mut params = HashMap::new();
        params.insert("page", page);

        let response = self
            .client
            .get(api::UPLOAD_HISTORY_API)
            .query(&params)
            .send()
            .await?
            .json::<Base<Vec<HistoryRecord>>>()
            .await?;

        Self::success_or_else(response, |base| Error::NotSuccess {
            code: base.code,
            message: base.message,
        })
    }

    fn success_or_else<T, E, F>(response: Base<T>, f: F) -> Result<T>
    where
        F: FnOnce(Base<T>) -> E,
        E: Into<anyhow::Error>,
    {
        if response.success {
            response.data.ok_or(Error::SuccessWithNoData.into())
        } else {
            Err(f(response).into())
        }
    }
}

fn build_client_with_auth_header<T: AsRef<str>>(token: T) -> reqwest::Client {
    let token = token.as_ref();

    let mut headers = HeaderMap::with_capacity(2);
    let mut auth_value = HeaderValue::from_str(token).expect("add auth header");
    auth_value.set_sensitive(true);
    headers.insert(AUTHORIZATION, auth_value);

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("build client")
}

async fn create_part_from_file<T: AsRef<Path>>(path: T) -> Result<Part> {
    let path = path.as_ref();
    let file_name = path
        .file_name()
        .map(|filename| filename.to_string_lossy().into_owned());
    let ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    let mime = mime_guess::from_ext(ext).first_or_octet_stream();

    let file = File::open(path).await?;
    let part = Part::stream(file).mime_str(mime.as_ref())?;

    Ok(if let Some(file_name) = file_name {
        part.file_name(file_name)
    } else {
        part
    })
}

#[cfg(test)]
mod tests {
    use super::Client;
    use std::io;

    #[tokio::test]
    async fn io_error() {
        let uploader = Client::new("token");
        let result = uploader.upload("not-exists.png").await;
        assert!(result.is_err());

        let error = result.err().expect("error");
        if let Some(error) = error.downcast_ref::<io::Error>() {
            assert_eq!(error.kind(), io::ErrorKind::NotFound);
        } else {
            panic!("Expected io::Error, Got: {:#?}", error);
        }
    }
}
