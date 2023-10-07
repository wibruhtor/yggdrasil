use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChatSizeSettings {
    #[validate(range(min = 0))]
    pub margin_top: f64,
    #[validate(range(min = 0))]
    pub margin_right: f64,
    #[validate(range(min = 0))]
    pub margin_bottom: f64,
    #[validate(range(min = 0))]
    pub margin_left: f64,
    #[validate(range(min = 0))]
    pub padding_top: f64,
    #[validate(range(min = 0))]
    pub padding_right: f64,
    #[validate(range(min = 0))]
    pub padding_bottom: f64,
    #[validate(range(min = 0))]
    pub padding_left: f64,
    #[validate(range(min = 0))]
    pub border_top_left_radius: f64,
    #[validate(range(min = 0))]
    pub border_top_right_radius: f64,
    #[validate(range(min = 0))]
    pub border_bottom_left_radius: f64,
    #[validate(range(min = 0))]
    pub border_bottom_right_radius: f64,
    #[validate(range(min = 5, max = 100))]
    pub max_messages: i32,
}
