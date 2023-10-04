use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Badge {
    pub id: String,
    pub set: String,
    pub image: String,
}