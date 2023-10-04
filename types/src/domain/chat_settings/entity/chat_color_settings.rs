use serde::{Deserialize, Serialize};

use crate::domain::CustomNickname;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatColorSettings {
    pub nickname_color: i64,
    pub background_color: i64,
    pub text_color: i64,
    pub gradient_only_for_custom_nicknames: bool,
    pub custom_nicknames: Vec<CustomNickname>,
}
