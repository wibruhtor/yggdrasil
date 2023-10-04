use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{ChatColorSettings, ChatFontSettings, ChatHideSettings, ChatSizeSettings, ChatType};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatSettings {
    pub id: Uuid,
    pub name: String,
    pub chat_type: ChatType,
    pub color: ChatColorSettings,
    pub size: ChatSizeSettings,
    pub hide: ChatHideSettings,
    pub font: ChatFontSettings,
    pub user_id: String,
}
