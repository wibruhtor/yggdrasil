use std::sync::Arc;

use axum::http::StatusCode;
use magic_crypt::{MagicCrypt256, MagicCryptTrait, new_magic_crypt};

use types::{AppError, AppResult};

pub struct Crypt {
    cipher: MagicCrypt256,
}

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
        self.cipher.decrypt_bytes_to_bytes(data).map_err(|e| {
            Crypt::FAIL_DECRYPT_BYTES.clone().cause(e.into())
        })
    }

    pub fn decrypt_str(&self, data: &str) -> AppResult<String> {
        self.cipher.decrypt_base64_to_string(data).map_err(|e| {
            Crypt::FAIL_DECRYPT_STRING.clone().cause(e.into())
        })
    }
}

macro_rules! crypt_errors {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $status:expr, $phrase:expr);
        )+
    ) => {
        impl Crypt {
        $(
            $(#[$docs])*
            pub const $name: AppError = AppError {
                status_code: $status,
                message: Some($phrase),
                cause: None,
                other: None
            };
        )+
        }
    }
}

crypt_errors! {
    (FAIL_DECRYPT_BYTES, StatusCode::INTERNAL_SERVER_ERROR, "fail decrypt bytes");
    (FAIL_DECRYPT_STRING, StatusCode::INTERNAL_SERVER_ERROR, "fail decrypt string");
}