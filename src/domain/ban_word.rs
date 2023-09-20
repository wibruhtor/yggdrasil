use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BanWord {
    pub ban_word_filter_id: Uuid,
    pub word: String,
}
