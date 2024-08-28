pub mod client;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(
        "[RequestId: {request_id}] Image upload repeated limit,\
    this image exists at: {exists_image_url}"
    )]
    ImageRepeated {
        request_id: String,
        exists_image_url: String,
    },
    #[error("server returned unsuccessful, code: {code}, message: {message}")]
    NotSuccess { code: String, message: String },
    #[error("server respond success with empty data")]
    SuccessWithNoData,
}

pub type Result<T> = anyhow::Result<T>;
