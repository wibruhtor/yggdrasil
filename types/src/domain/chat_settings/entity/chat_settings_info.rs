use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::ChatType;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatSettingsInfo {
    pub id: Uuid,
    pub name: String,
    pub chat_type: ChatType,
    pub user_id: String,
}
