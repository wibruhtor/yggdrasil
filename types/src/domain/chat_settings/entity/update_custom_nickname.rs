use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::CustomNickname;

#[derive(Serialize, Deserialize, Validate, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomNickname {
    #[validate(length(min = 4, max = 25))]
    pub nickname: String,
    #[validate(range(min = 0, max = 4294967295))]
    pub start_color: i64,
    #[validate(range(min = 0, max = 4294967295))]
    pub end_color: i64,
}

impl Into<CustomNickname> for UpdateCustomNickname {
    fn into(self) -> CustomNickname {
        CustomNickname {
            nickname: self.nickname,
            start_color: self.start_color,
            end_color: self.end_color,
        }
    }
}
