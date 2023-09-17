use chrono::{Duration, NaiveDateTime};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, AppResult};

const AUDIENCE: &str = "wibruhtor";
const ISSUER: &str = "api.wibruhtor.ru";
const ACCESS_TOKEN_TTL_IN_HOURS: i64 = 1;
const REFRESH_TOKEN_TTL_IN_DAYS: i64 = 365;

#[allow(dead_code)]
pub struct Jwt {
    secret: String,
    validation: Validation,
}

#[allow(dead_code)]
impl Jwt {
    pub fn new(secret: &str) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[AUDIENCE]);
        validation.set_issuer(&[ISSUER]);
        validation.validate_nbf = true;
        Jwt {
            secret: secret.to_string(),
            validation,
        }
    }

    pub fn validate(&self, token: &str) -> AppResult<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret.as_bytes()),
            &self.validation,
        )
        .map_err(|err| match *err.kind() {
            ErrorKind::InvalidSignature => {
                AppError::new(StatusCode::FORBIDDEN).message("invalid signature".to_string())
            }
            ErrorKind::InvalidToken => {
                AppError::new(StatusCode::FORBIDDEN).message("invalid token".to_string())
            }
            ErrorKind::ExpiredSignature => {
                AppError::new(StatusCode::FORBIDDEN).message("expired token".to_string())
            }
            _ => AppError::new(StatusCode::FORBIDDEN)
                .message("fail validate token".to_string())
                .cause(err.into()),
        })?;

        Ok(token_data.claims)
    }

    pub fn generate_access_token(
        &self,
        id: &Uuid,
        user_id: &str,
        username: &str,
        time: &NaiveDateTime,
    ) -> AppResult<(String, Claims)> {
        let duration = Duration::hours(ACCESS_TOKEN_TTL_IN_HOURS);
        self.generate_token(id, user_id, username, "access_token", &duration, time)
    }

    pub fn generate_refresh_token(
        &self,
        id: &Uuid,
        user_id: &str,
        username: &str,
        time: &NaiveDateTime,
    ) -> AppResult<(String, Claims)> {
        let duration = Duration::days(REFRESH_TOKEN_TTL_IN_DAYS);
        self.generate_token(id, user_id, username, "refresh_token", &duration, time)
    }

    fn generate_token(
        &self,
        id: &Uuid,
        user_id: &str,
        username: &str,
        token_type: &str,
        token_ttl: &Duration,
        time: &NaiveDateTime,
    ) -> AppResult<(String, Claims)> {
        let timestamp = time.timestamp();
        let claims = Claims {
            jti: id.to_owned(),
            typ: token_type.to_string(),
            aud: AUDIENCE.to_string(),
            exp: timestamp + token_ttl.num_seconds(),
            iat: timestamp,
            iss: ISSUER.to_string(),
            nbf: timestamp,
            sub: user_id.to_string(),
            username: username.to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret.as_bytes()),
        )?;

        Ok((token, claims))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub jti: Uuid,        // Token id
    pub typ: String,      // Type of token. access_token or refresh_token
    pub aud: String,      // Audience
    pub exp: i64,         // Expiration time (as UTC timestamp)
    pub iat: i64,         // Issued at (as UTC timestamp)
    pub iss: String,      // Issuer
    pub nbf: i64,         // Not Before (as UTC timestamp)
    pub sub: String,      // Subject (user id)
    pub username: String, // Username
}
