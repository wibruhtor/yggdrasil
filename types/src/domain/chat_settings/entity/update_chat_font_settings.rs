use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChatFontSettings {
    pub font_family: String,
    #[validate(range(min = 0, max = 1000))]
    pub nickname_font_weight: i32,
    #[validate(range(min = 0, max = 1000))]
    pub text_font_weight: i32,
    #[validate(range(min = 0))]
    pub font_size: f64,
}
