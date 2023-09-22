use std::sync::Arc;

use axum::http::StatusCode;
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};

use crate::error::{AppError, AppResult};

pub struct Crypt {
    cipher: MagicCrypt256,
}

#[allow(dead_code)]
impl Crypt {
    pub fn new(secret: &str) -> Arc<Self> {
        Arc::new(Crypt {
            cipher: new_magic_crypt!(secret, 256),
        })
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        self.cipher.encrypt_bytes_to_bytes(data)
    }

    pub fn encrypt_str(&self, data: &str) -> String {
        self.cipher.encrypt_str_to_base64(data)
    }

    pub fn decrypt(&self, data: &[u8]) -> AppResult<Vec<u8>> {
        self.cipher.decrypt_bytes_to_bytes(data).map_err(|_| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("fail decrypt bytes".to_string())
        })
    }

    pub fn decrypt_str(&self, data: &str) -> AppResult<String> {
        self.cipher.decrypt_base64_to_string(data).map_err(|_| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("fail decrypt string".to_string())
        })
    }
}
