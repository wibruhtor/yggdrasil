use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChatSettingsInfo {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "chatType")]
    pub chat_type: ChatType,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChatSettings {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "chatType")]
    pub chat_type: ChatType,
    pub color: ChatColorSettings,
    pub size: ChatSizeSettings,
    pub hide: ChatHideSettings,
    pub font: ChatFontSettings,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ChatType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "block")]
    Block,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate)]
pub struct ChatColorSettings {
    #[serde(rename = "nicknameColor")]
    #[validate(range(min = 0, max = 4294967295))]
    pub nickname_color: i64,
    #[serde(rename = "backgroundColor")]
    #[validate(range(min = 0, max = 4294967295))]
    pub background_color: i64,
    #[serde(rename = "textColor")]
    #[validate(range(min = 0, max = 4294967295))]
    pub text_color: i64,
    #[serde(rename = "gradientOnlyForCustomNicknames")]
    pub gradient_only_for_custom_nicknames: bool,
    #[serde(rename = "customNicknames")]
    pub custom_nicknames: Vec<CustomNickname>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate, Clone)]
pub struct CustomNickname {
    #[validate(length(min = 4, max = 25))]
    pub nickname: String,
    #[serde(rename = "startColor")]
    #[validate(range(min = 0, max = 4294967295))]
    pub start_color: i64,
    #[serde(rename = "endColor")]
    #[validate(range(min = 0, max = 4294967295))]
    pub end_color: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChatSizeSettings {
    #[serde(rename = "maringTop")]
    pub margin_top: f64,
    #[serde(rename = "maringRight")]
    pub margin_right: f64,
    #[serde(rename = "maringBottom")]
    pub margin_bottom: f64,
    #[serde(rename = "maringLeft")]
    pub margin_left: f64,
    #[serde(rename = "paddingTop")]
    pub padding_top: f64,
    #[serde(rename = "paddingRight")]
    pub padding_right: f64,
    #[serde(rename = "paddingBottom")]
    pub padding_bottom: f64,
    #[serde(rename = "paddingLeft")]
    pub padding_left: f64,
    #[serde(rename = "borderTopLeftRadius")]
    pub border_top_left_radius: f64,
    #[serde(rename = "borderTopRightRadius")]
    pub border_top_right_radius: f64,
    #[serde(rename = "borderBottomLeftRadius")]
    pub border_bottom_left_radius: f64,
    #[serde(rename = "borderBottomRightRadius")]
    pub border_bottom_right_radius: f64,
    #[serde(rename = "maxMessages")]
    pub max_messages: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate)]
pub struct ChatHideSettings {
    #[serde(rename = "hideMessagePattern")]
    pub hide_message_pattern: String,
    #[serde(rename = "hidePointRewards")]
    pub hide_point_rewards: bool,
    #[serde(rename = "hideLinks")]
    pub hide_links: bool,
    #[serde(rename = "linkReplacement")]
    pub link_replacement: String,
    #[serde(rename = "banWordReplacement")]
    pub ban_word_replacement: String,
    #[serde(rename = "nicknames")]
    #[validate(custom(function = "vec_string_max_len::<4, 25>"))]
    pub nicknames: Vec<String>,
    #[serde(rename = "banWordFilterId")]
    pub ban_word_filter_id: Option<Uuid>,
}

fn vec_string_max_len<const MIN: usize, const MAX: usize>(
    value: &Vec<String>,
) -> Result<(), ValidationError> {
    for s in value.iter() {
        if s.len() > MAX {
            return Err(ValidationError::new("String too long"));
        } else if s.len() < MIN {
            return Err(ValidationError::new("String too short"));
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChatFontSettings {
    #[serde(rename = "fontFamily")]
    pub font_family: String,
    #[serde(rename = "nicknameFontWeight")]
    pub nickname_font_weight: i32,
    #[serde(rename = "textFontWeight")]
    pub text_font_weight: i32,
    #[serde(rename = "fontSize")]
    pub font_size: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate)]
pub struct UpdateChatSettings {
    #[validate(length(min = 2, max = 32))]
    pub name: String,
    #[serde(rename = "chatType")]
    pub chat_type: ChatType,
    #[validate]
    pub color: ChatColorSettings,
    #[validate]
    pub size: UpdateChatSizeSettings,
    #[validate]
    pub hide: ChatHideSettings,
    #[validate]
    pub font: UpdateChatFontSettings,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate)]
pub struct UpdateChatSizeSettings {
    #[serde(rename = "maringTop")]
    #[validate(range(min = 0))]
    pub margin_top: f64,
    #[serde(rename = "maringRight")]
    #[validate(range(min = 0))]
    pub margin_right: f64,
    #[serde(rename = "maringBottom")]
    #[validate(range(min = 0))]
    pub margin_bottom: f64,
    #[serde(rename = "maringLeft")]
    #[validate(range(min = 0))]
    pub margin_left: f64,
    #[serde(rename = "paddingTop")]
    #[validate(range(min = 0))]
    pub padding_top: f64,
    #[serde(rename = "paddingRight")]
    #[validate(range(min = 0))]
    pub padding_right: f64,
    #[serde(rename = "paddingBottom")]
    #[validate(range(min = 0))]
    pub padding_bottom: f64,
    #[serde(rename = "paddingLeft")]
    #[validate(range(min = 0))]
    pub padding_left: f64,
    #[serde(rename = "borderTopLeftRadius")]
    #[validate(range(min = 0))]
    pub border_top_left_radius: f64,
    #[serde(rename = "borderTopRightRadius")]
    #[validate(range(min = 0))]
    pub border_top_right_radius: f64,
    #[serde(rename = "borderBottomLeftRadius")]
    #[validate(range(min = 0))]
    pub border_bottom_left_radius: f64,
    #[serde(rename = "borderBottomRightRadius")]
    #[validate(range(min = 0))]
    pub border_bottom_right_radius: f64,
    #[serde(rename = "maxMessages")]
    #[validate(range(min = 5, max = 100))]
    pub max_messages: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate)]
pub struct UpdateChatFontSettings {
    #[serde(rename = "fontFamily")]
    pub font_family: String,
    #[serde(rename = "nicknameFontWeight")]
    #[validate(range(min = 0, max = 1000))]
    pub nickname_font_weight: i32,
    #[serde(rename = "textFontWeight")]
    #[validate(range(min = 0, max = 1000))]
    pub text_font_weight: i32,
    #[serde(rename = "fontSize")]
    #[validate(range(min = 0))]
    pub font_size: f64,
}
