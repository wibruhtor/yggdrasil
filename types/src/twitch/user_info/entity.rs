use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UserInfo {
    pub id: String,
    pub login: String,
}