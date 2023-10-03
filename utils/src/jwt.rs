use axum::http::StatusCode;
use chrono::{Duration, NaiveDateTime};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use types::{AppError, AppResult};

const AUDIENCE: &str = "wibruhtor";
const ISSUER: &str = "api.wibruhtor.ru";
const ACCESS_TOKEN_TTL_IN_HOURS: i64 = 1;
const REFRESH_TOKEN_TTL_IN_DAYS: i64 = 365;

pub struct JwtMaker {
    secret: String,
    validation: Validation,
}

#[allow(dead_code)]
impl JwtMaker {
    pub fn new(secret: &str) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[AUDIENCE]);
        validation.set_issuer(&[ISSUER]);
        validation.validate_nbf = true;
        JwtMaker {
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
                ErrorKind::InvalidSignature => JwtMaker::INVALID_SIGNATURE_ERROR,
                ErrorKind::InvalidToken => JwtMaker::INVALID_TOKEN_ERROR,
                ErrorKind::ExpiredSignature => JwtMaker::EXPIRED_TOKEN_ERROR,
                _ => JwtMaker::FAIL_VALIDATE_TOKEN_ERROR,
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
        self.generate_token(id, user_id, username, TokenType::Access, &duration, time)
    }

    pub fn generate_refresh_token(
        &self,
        id: &Uuid,
        user_id: &str,
        username: &str,
        time: &NaiveDateTime,
    ) -> AppResult<(String, Claims)> {
        let duration = Duration::days(REFRESH_TOKEN_TTL_IN_DAYS);
        self.generate_token(id, user_id, username, TokenType::Refresh, &duration, time)
    }

    fn generate_token(
        &self,
        id: &Uuid,
        user_id: &str,
        username: &str,
        token_type: TokenType,
        token_ttl: &Duration,
        time: &NaiveDateTime,
    ) -> AppResult<(String, Claims)> {
        let timestamp = time.timestamp();
        let claims = Claims {
            jti: id.to_owned(),
            typ: token_type,
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
    pub jti: Uuid,
    // Token id
    pub typ: TokenType,
    // Type of token. access_token or refresh_token
    pub aud: String,
    // Audience
    pub exp: i64,
    // Expiration time (as UTC timestamp)
    pub iat: i64,
    // Issued at (as UTC timestamp)
    pub iss: String,
    // Issuer
    pub nbf: i64,
    // Not Before (as UTC timestamp)
    pub sub: String,
    // Subject (user id)
    pub username: String, // Username
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    #[serde(rename = "access_token")]
    Access,
    #[serde(rename = "refresh_token")]
    Refresh,
}

macro_rules! jwt_errors {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $status:expr, $phrase:expr);
        )+
    ) => {
        impl JwtMaker {
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

jwt_errors! {
    (INVALID_SIGNATURE_ERROR, StatusCode::FORBIDDEN, "invalid signature");
    (INVALID_TOKEN_ERROR, StatusCode::FORBIDDEN, "invalid token");
    (EXPIRED_TOKEN_ERROR, StatusCode::FORBIDDEN, "expired token");
    (FAIL_VALIDATE_TOKEN_ERROR, StatusCode::FORBIDDEN, "fail validate token");
}