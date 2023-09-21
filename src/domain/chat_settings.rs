use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    #[serde(rename = "banWordFilterId")]
    pub ban_word_filter_id: Option<Uuid>,
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
