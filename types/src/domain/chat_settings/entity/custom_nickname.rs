use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomNickname {
    pub nickname: String,
    pub start_color: i64,
    pub end_color: i64,
}
