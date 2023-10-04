use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatSizeSettings {
    pub margin_top: f64,
    pub margin_right: f64,
    pub margin_bottom: f64,
    pub margin_left: f64,
    pub padding_top: f64,
    pub padding_right: f64,
    pub padding_bottom: f64,
    pub padding_left: f64,
    pub border_top_left_radius: f64,
    pub border_top_right_radius: f64,
    pub border_bottom_left_radius: f64,
    pub border_bottom_right_radius: f64,
    pub max_messages: i32,
}
