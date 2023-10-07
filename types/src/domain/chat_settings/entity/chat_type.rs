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

impl ChatType {
    pub fn from_str(chat_type: &str) -> ChatType {
        match chat_type {
            "default-reverse" => ChatType::DefaultReverse,
            "block" => ChatType::Block,
            "block-reverse" => ChatType::BlockReverse,
            "alternative-block" => ChatType::AlternativeBlock,
            "alternative-block-reverse" => ChatType::AlternativeBlockReverse,
            _ => ChatType::Default,
        }
    }

    pub fn to_str(&self) -> &str {
        match *self {
            ChatType::Default => "default",
            ChatType::DefaultReverse => "default-reverse",
            ChatType::Block => "block",
            ChatType::BlockReverse => "block-reverse",
            ChatType::AlternativeBlock => "alternative-block",
            ChatType::AlternativeBlockReverse => "alternative-block-reverse",
        }
    }
}
