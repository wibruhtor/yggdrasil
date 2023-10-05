use serde::Deserialize;

use types::twitch::UserInfo;

#[derive(Deserialize, Debug)]
pub struct GetUserInfoResponse {
    pub data: Vec<UserInfo>,
}