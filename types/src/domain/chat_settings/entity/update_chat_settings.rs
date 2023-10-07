use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::{
    ChatType, UpdateChatColorSettings, UpdateChatFontSettings, UpdateChatHideSettings,
    UpdateChatSizeSettings,
};

#[derive(Serialize, Deserialize, Validate, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChatSettings {
    #[validate(length(min = 2, max = 32))]
    pub name: String,
    pub chat_type: ChatType,
    #[validate]
    pub color: UpdateChatColorSettings,
    #[validate]
    pub size: UpdateChatSizeSettings,
    #[validate]
    pub hide: UpdateChatHideSettings,
    #[validate]
    pub font: UpdateChatFontSettings,
}
