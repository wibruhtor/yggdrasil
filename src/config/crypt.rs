use std::env;

#[derive(Debug, Clone, PartialEq)]
pub struct Crypt {
    pub secret: String,
}

impl Crypt {
    pub fn new() -> Self {
        Crypt {
            secret: env::var("CRYPT_SECRET").expect("fail get CRYPT_SECRET"),
        }
    }
}
