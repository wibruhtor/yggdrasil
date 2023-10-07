use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BanWordFilter {
    pub id: Uuid,
    pub name: String,
    pub ban_words: Vec<String>,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BanWordFilterInfo {
    pub id: Uuid,
    pub name: String,
    pub user_id: String,
}
