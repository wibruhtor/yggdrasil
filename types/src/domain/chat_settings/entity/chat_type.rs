use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ChatType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "default-reverse")]
    DefaultReverse,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "block-reverse")]
    BlockReverse,
    #[serde(rename = "alternative-block")]
    AlternativeBlock,
    #[serde(rename = "alternative-block-reverse")]
    AlternativeBlockReverse,
}
