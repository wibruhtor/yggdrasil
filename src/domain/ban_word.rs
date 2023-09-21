use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BanWord {
    #[serde(rename = "banWordFilterId")]
    pub ban_word_filter_id: Uuid,
    pub word: String,
}
