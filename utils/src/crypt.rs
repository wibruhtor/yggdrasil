use axum::http::StatusCode;
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};

use types::error::{AppError, AppResult};

pub struct Crypt {
    cipher: MagicCrypt256,
}

impl Crypt {
    pub fn new(secret: &str) -> Self {
        Crypt {
            cipher: new_magic_crypt!(secret, 256),
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        self.cipher.encrypt_bytes_to_bytes(data)
    }

    pub fn encrypt_str(&self, data: &str) -> String {
        self.cipher.encrypt_str_to_base64(data)
    }

    pub fn decrypt(&self, data: &[u8]) -> AppResult<Vec<u8>> {
        self.cipher
            .decrypt_bytes_to_bytes(data)
            .map_err(|e| Crypt::FAIL_DECRYPT_BYTES_ERROR.clone().cause(e.into()))
    }

    pub fn decrypt_str(&self, data: &str) -> AppResult<String> {
        self.cipher
            .decrypt_base64_to_string(data)
            .map_err(|e| Crypt::FAIL_DECRYPT_STRING_ERROR.clone().cause(e.into()))
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
    (FAIL_DECRYPT_BYTES_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail decrypt bytes");
    (FAIL_DECRYPT_STRING_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail decrypt string");
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};

    use crate::crypt::Crypt;

    #[test]
    fn bytes() {
        for _ in 1..100 {
            let key = (0..64).fake::<String>();
            let bytes: Vec<u8> = (0..1024).map(|_| Faker.fake::<u8>()).collect();

            let crypt = Crypt::new(&key);

            let encrypted_bytes = crypt.encrypt(&bytes);
            let decrypted_bytes = crypt.decrypt(&encrypted_bytes);
            assert!(decrypted_bytes.is_ok());
            let decrypted_bytes = decrypted_bytes.unwrap();

            assert_ne!(bytes, encrypted_bytes);
            assert_eq!(bytes, decrypted_bytes);
        }
    }

    #[test]
    fn strings() {
        for _ in 1..100 {
            let key = (0..64).fake::<String>();
            let string = (0..1024).fake::<String>();

            let crypt = Crypt::new(&key);

            let encrypted_string = crypt.encrypt_str(&string);
            let decrypted_string = crypt.decrypt_str(&encrypted_string);

            assert_ne!(string, encrypted_string);
            assert!(decrypted_string.is_ok());
            assert_eq!(string, decrypted_string.unwrap());
        }
    }
}
