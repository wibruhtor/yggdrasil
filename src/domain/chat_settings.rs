use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChatSettings {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "chatType")]
    pub chat_type: ChatType,
    pub color: ChatColorSettings,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChatColorSettings {
    #[serde(rename = "nicknameColor")]
    pub nickname_color: u32,
    #[serde(rename = "backgroundColor")]
    pub background_color: u32,
    #[serde(rename = "textColor")]
    pub text_color: u32,
    #[serde(rename = "gradientOnlyForCustomNicknames")]
    pub gradient_only_for_custom_nicknames: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ChatType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "block")]
    Block,
}
