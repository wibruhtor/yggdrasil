use axum::http::StatusCode;
use chrono::{Duration, NaiveDateTime};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use types::error::{AppError, AppResult};

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

    #[instrument(skip_all)]
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

    #[instrument(skip(self, id))]
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

    #[instrument(skip(self, id))]
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

    #[instrument(skip(self, id))]
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
    /// Token id
    pub jti: Uuid,
    /// Type of token. access_token or refresh_token
    pub typ: TokenType,
    /// Audience
    pub aud: String,
    /// Expiration time (as UTC timestamp)
    pub exp: i64,
    /// Issued at (as UTC timestamp)
    pub iat: i64,
    /// Issuer
    pub iss: String,
    /// Not Before (as UTC timestamp)
    pub nbf: i64,
    /// Subject (user id)
    pub sub: String,
    /// Username
    pub username: String,
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

#[cfg(test)]
pub mod tests {
    use chrono::{Duration, NaiveDateTime};
    use uuid::Uuid;

    use fake::{faker::name::en::Name, Dummy, Fake, Faker};

    use crate::jwt::{JwtMaker, TokenType, ACCESS_TOKEN_TTL_IN_HOURS, REFRESH_TOKEN_TTL_IN_DAYS};

    #[derive(Debug, Dummy)]
    #[allow(dead_code)]
    struct TestData {
        key: String,
        token_id: Uuid,
        user_id: String,
        #[dummy(faker = "Name()")]
        username: String,
        datetime: NaiveDateTime,
    }

    #[test]
    fn access_tokens() {
        for _ in 1..100 {
            let data = Faker.fake::<TestData>();

            let expired_at = data.datetime.timestamp()
                + Duration::hours(ACCESS_TOKEN_TTL_IN_HOURS).num_seconds();

            let jwt_maker = JwtMaker::new(&data.key);

            let access_token_result = jwt_maker.generate_access_token(
                &data.token_id,
                &data.user_id,
                &data.username,
                &data.datetime,
            );
            assert!(access_token_result.is_ok());
            let (access_token, claims) = access_token_result.unwrap();

            assert!(!access_token.is_empty());
            assert_eq!(claims.jti, data.token_id);
            assert_eq!(claims.sub, data.user_id);
            assert_eq!(claims.username, data.username);
            assert_eq!(claims.typ, TokenType::Access);
            assert_eq!(claims.iat, data.datetime.timestamp());
            assert_eq!(claims.exp, expired_at);
        }
    }

    #[test]
    fn refresh_tokens() {
        for _ in 1..100 {
            let data = Faker.fake::<TestData>();

            let expired_at =
                data.datetime.timestamp() + Duration::days(REFRESH_TOKEN_TTL_IN_DAYS).num_seconds();

            let jwt_maker = JwtMaker::new(&data.key);

            let refresh_token_result = jwt_maker.generate_refresh_token(
                &data.token_id,
                &data.user_id,
                &data.username,
                &data.datetime,
            );
            assert!(refresh_token_result.is_ok());
            let (refresh_token, claims) = refresh_token_result.unwrap();

            assert!(!refresh_token.is_empty());
            assert_eq!(claims.jti, data.token_id);
            assert_eq!(claims.sub, data.user_id);
            assert_eq!(claims.username, data.username);
            assert_eq!(claims.typ, TokenType::Refresh);
            assert_eq!(claims.iat, data.datetime.timestamp());
            assert_eq!(claims.exp, expired_at);
        }
    }
}
