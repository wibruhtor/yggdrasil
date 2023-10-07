use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

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

#[derive(Serialize, Deserialize, Validate, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBanWordFilter {
    #[validate(length(min = 2, max = 32))]
    pub name: String,
    #[validate(custom(function = "ban_word_vec::<1, 32>"))]
    pub ban_words: Vec<String>,
}

fn ban_word_vec<const MIN: usize, const MAX: usize>(
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
