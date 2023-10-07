use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::UpdateCustomNickname;

#[derive(Serialize, Deserialize, Validate, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChatColorSettings {
    #[validate(range(min = 0, max = 4294967295))]
    pub nickname_color: i64,
    #[validate(range(min = 0, max = 4294967295))]
    pub background_color: i64,
    #[validate(range(min = 0, max = 4294967295))]
    pub text_color: i64,
    pub gradient_only_for_custom_nicknames: bool,
    #[validate]
    pub custom_nicknames: Vec<UpdateCustomNickname>,
}
