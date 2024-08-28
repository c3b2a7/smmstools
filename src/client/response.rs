use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct Base<T> {
    pub success: bool,
    pub code: String,
    pub message: String,
    pub data: Option<T>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
    // for image_repeated use
    #[serde(rename = "images")]
    pub(crate) exists_image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub username: String,
    pub email: String,
    pub role: String,
    pub group_expire: String,
    pub email_verified: EmailVerified,
    pub disk_usage: String,
    pub disk_limit: String,
    pub disk_usage_raw: u64,
    pub disk_limit_raw: u64,
}

impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username: {}\nEmail: {}\nEmail Verified: {}\nUser Group: {}\nUser Group Expire: {}\nDisk Usage: {}\nDisk Limit: {}",
            self.username,
            self.email,
            self.email_verified,
            self.role,
            if self.group_expire.is_empty() { "-" } else { &self.group_expire },
            self.disk_usage,
            self.disk_limit,
        )
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum EmailVerified {
    NotVerified = 0,
    Verified = 1,
}

impl Display for EmailVerified {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailVerified::NotVerified => write!(f, "Not Verified"),
            EmailVerified::Verified => write!(f, "Verified"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadResult {
    pub file_id: u64,
    pub width: u64,
    pub height: u64,
    pub filename: String,
    pub storename: String,
    pub size: u64,
    pub path: String,
    pub hash: String,
    pub url: String,
    pub delete: String,
    pub page: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryRecord {
    pub width: u64,
    pub height: u64,
    pub filename: String,
    pub storename: String,
    pub size: u64,
    pub path: String,
    pub hash: String,
    pub created_at: String,
    pub url: String,
    pub delete: String,
    pub page: String,
}
