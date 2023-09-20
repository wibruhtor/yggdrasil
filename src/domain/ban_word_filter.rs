use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq)]
pub struct BanWordFilter {
    pub id: Uuid,
    pub name: String,
    pub user_id: String,
}
