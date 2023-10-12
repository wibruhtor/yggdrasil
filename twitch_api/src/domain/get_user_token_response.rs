use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetUserTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}
