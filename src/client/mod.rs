mod client;
pub use client::Client;

pub mod response;

/// see [SM.MS API](https://doc.sm.ms)
#[allow(unused)]
mod api {
    pub(crate) const BASE_URL: &'static str = "https://sm.ms/api/v2/";

    pub(crate) const PROFILE_API: &'static str = "https://sm.ms/api/v2/profile";

    pub(crate) const UPLOAD_API: &'static str = "https://sm.ms/api/v2/upload";

    pub(crate) const DELETE_API: &'static str = "https://sm.ms/api/v2/delete";

    pub(crate) const UPLOAD_HISTORY_API: &'static str = "https://sm.ms/api/v2/upload_history";
}
