use std::env;

#[derive(Debug, Clone, PartialEq)]
pub struct Jwt {
    pub secret: String,
}

impl Jwt {
    pub fn new() -> Self {
        Jwt {
            secret: env::var("JWT_SECRET").expect("fail get JWT_SECRET"),
        }
    }
}
