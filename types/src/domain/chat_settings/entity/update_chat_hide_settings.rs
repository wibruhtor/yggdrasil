use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Validate, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChatHideSettings {
    pub hide_message_pattern: String,
    pub hide_point_rewards: bool,
    pub hide_links: bool,
    pub link_replacement: String,
    pub ban_word_replacement: String,
    #[validate(custom(function = "nickname_vec::<4, 25>"))]
    pub nicknames: Vec<String>,
    pub ban_word_filter_id: Option<Uuid>,
}

fn nickname_vec<const MIN: usize, const MAX: usize>(
    value: &Vec<String>,
) -> Result<(), ValidationError> {
    for s in value.iter() {
        if s.len() > MAX {
            return Err(ValidationError::new("string too long"));
        } else if s.len() < MIN {
            return Err(ValidationError::new("string too short"));
        }
    }
    Ok(())
}
