use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatFontSettings {
    pub font_family: String,
    pub nickname_font_weight: i32,
    pub text_font_weight: i32,
    pub font_size: f64,
}
