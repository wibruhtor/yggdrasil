use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetAppAccessTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
}