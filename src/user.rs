use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{Error, TIDAL_API_BASE_URL, TidalClient, User};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimpleUser {
    #[serde(rename = "acceptedEULA")]
    pub accpeted_eula: bool,
    pub apple_uid: Option<String>,
    pub artist_id: u64,
    pub country_code: String,
    pub created: String,
    pub date_of_birth: String,
    pub early_access_program: bool,
    pub email: String,
    pub email_verified: bool,
    pub etag: String,
    pub facebook_uid: u64,
    pub first_name: Option<String>,
    pub id: u64,
    pub last_name: Option<String>,
    pub newsletter: bool,
    pub nostr_public_key: Option<String>,
    pub parent_id: u64,
    pub partner: u64,
    pub profile_name: Option<String>,
    pub tidal_id: String,
    pub username: String,
    pub year_of_birth: u64,
}

impl TidalClient {
    pub async fn user(&self, user_id: u64) -> Result<SimpleUser, Error> {
        let url = format!("{TIDAL_API_BASE_URL}/users/{user_id}");

        let params = serde_json::json!({
            "countryCode": self.get_country_code(),
            "locale": self.get_locale(),
            "deviceType": self.get_device_type().as_ref(),
        });

        let resp: SimpleUser = self
            .do_request(Method::GET, &url, Some(params), None)
            .await?;

        Ok(resp)
    }
}
