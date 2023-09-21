use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BanWordFilter {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
