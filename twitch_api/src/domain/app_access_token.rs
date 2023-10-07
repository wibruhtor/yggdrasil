use std::ops::Add;

use chrono::{Duration, NaiveDateTime, Utc};

#[derive(Debug)]
pub struct AppAccessToken {
    token: Option<String>,
    expired_at: NaiveDateTime,
}

impl AppAccessToken {
    pub fn is_expired(&self) -> bool {
        if self.token.is_none() {
            false
        } else {
            self.expired_at < Utc::now().add(Duration::minutes(-1)).naive_utc()
        }
    }

    pub fn set(&mut self, token: &str, expired_at: &NaiveDateTime) {
        self.token = Some(token.to_string());
        self.expired_at = expired_at.clone();
    }

    pub fn token(&self) -> Option<String> {
        self.token.clone()
    }
}

impl Default for AppAccessToken {
    fn default() -> Self {
        AppAccessToken {
            token: None,
            expired_at: Utc::now().naive_utc(),
        }
    }
}
