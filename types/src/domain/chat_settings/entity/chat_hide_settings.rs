use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatHideSettings {
    pub hide_message_pattern: String,
    pub hide_point_rewards: bool,
    pub hide_links: bool,
    pub link_replacement: String,
    pub ban_word_replacement: String,
    pub nicknames: Vec<String>,
    pub ban_word_filter_id: Option<Uuid>,
}
