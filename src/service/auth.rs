use std::{collections::HashMap, sync::Arc};

use reqwest::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    config,
    dao::{TokenDao, TwitchDataDao, UserDao},
    error::{AppError, AppResult},
    jwt::{self, Claims},
};

const AUTHORIZE_URL: &str = "https://id.twitch.tv/oauth2/authorize";
const TOKEN_URL: &str = "https://id.twitch.tv/oauth2/token";
const GET_USERS_URL: &str = "https://api.twitch.tv/helix/users";

#[allow(dead_code)]
pub struct AuthService {
    twitch_config: config::twitch::Twitch,
    jwt: jwt::Jwt,
    user_dao: Arc<UserDao>,
    twitch_data_dao: Arc<TwitchDataDao>,
    token_dao: Arc<TokenDao>,
    scope: Vec<String>,
}

#[allow(dead_code)]
impl AuthService {
    pub fn new(
        twitch_config: config::twitch::Twitch,
        jwt: jwt::Jwt,
        user_dao: Arc<UserDao>,
        twitch_data_dao: Arc<TwitchDataDao>,
        token_dao: Arc<TokenDao>,
    ) -> Self {
        AuthService {
            twitch_config,
            jwt,
            user_dao,
            twitch_data_dao,
            token_dao,
            scope: vec![],
        }
    }

    pub fn authorize_url(&self) -> String {
        format!(
            "{}?client_id={}&force_verify=true&redirect_uri={}&response_type=code&scope={}",
            AUTHORIZE_URL,
            self.twitch_config.client_id,
            self.twitch_config.callback_url,
            self.scope.join(" ")
        )
    }

    pub async fn exchange_code(
        &self,
        code: &str,
        user_agent: &str,
        ip: &str,
    ) -> AppResult<(String, String)> {
        let span = tracing::debug_span!("exchange code");
        let _span = span.enter();

        // region: Generate params
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("client_id", &self.twitch_config.client_id);
        params.insert("client_secret", &self.twitch_config.client_secret);
        params.insert("redirect_uri", &self.twitch_config.callback_url);
        params.insert("code", code);
        params.insert("grant_type", "authorization_code");
        // endregion

        let client = reqwest::Client::new();

        // region: Exchange code
        span.in_scope(|| {
            tracing::debug!("send request to twitch for exchange code");
        });
        let resp = client
            .post(TOKEN_URL)
            .form(&params)
            .send()
            .await
            .map_err(|e| {
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .message("fail exchange code".to_string())
                    .cause(e.into())
            })?;

        if resp.status() != StatusCode::OK {
            return Err(
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR).message(format!(
                    "fail exchange code with status code: {}",
                    resp.status().as_u16()
                )),
            );
        }
        // endregion

        // region: Parse exchange code response to json
        span.in_scope(|| {
            tracing::debug!("parse response of exchange code");
        });
        let exchange_code_response = resp.json::<ExchangeCodeResponse>().await.map_err(|e| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("fail parse json of exchange code response".to_string())
                .cause(e.into())
        })?;
        // endregion

        // region: Get twitch user
        span.in_scope(|| {
            tracing::debug!("send request to twitch for get twitch user");
        });
        let resp = client
            .get(GET_USERS_URL)
            .header(
                "Authorization",
                format!("Bearer {}", exchange_code_response.access_token),
            )
            .header("Client-Id", &self.twitch_config.client_id)
            .send()
            .await
            .map_err(|e| {
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .message("fail get twitch user".to_string())
                    .cause(e.into())
            })?;

        if resp.status() != StatusCode::OK {
            return Err(
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR).message(format!(
                    "fail get twitch user with status code: {}",
                    resp.status().as_u16()
                )),
            );
        }
        // endregion

        // region: Parse get twitch user response to json
        span.in_scope(|| {
            tracing::debug!("parse response of get twich user");
        });
        let twitch_user_response = resp.json::<TwitchUserResponse>().await.map_err(|e| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("fail parse json of get twitch user response".to_string())
                .cause(e.into())
        })?;
        // endregion

        // region: Get twitch user from response
        span.in_scope(|| {
            tracing::debug!("get twitch user from response");
        });
        let twitch_user = twitch_user_response.data.first().ok_or(
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("fail get twitch user from response".to_string()),
        )?;
        // endregion

        // region: Get or create user
        span.in_scope(|| {
            tracing::debug!("get or create user");
        });
        let user = self
            .user_dao
            .get_or_create(&twitch_user.id, &twitch_user.login)
            .await?;
        // endregion

        // region: Create or update twitch data
        span.in_scope(|| {
            tracing::debug!("create or update twitch data");
        });
        self.twitch_data_dao
            .create_or_update(&user.id, &exchange_code_response.refresh_token)
            .await?;
        // endregion

        // region: Create token
        span.in_scope(|| {
            tracing::debug!("create token");
        });
        let token = self.token_dao.create(&user.id, user_agent, ip).await?;
        // endregion

        // region: Create jwt tokens
        span.in_scope(|| {
            tracing::debug!("create access token");
        });
        let access_token = self
            .jwt
            .generate_access_token(&token.id, &user.id, &user.username, &token.refreshed_at)?
            .0;
        span.in_scope(|| {
            tracing::debug!("create refresh token");
        });
        let refresh_token = self
            .jwt
            .generate_refresh_token(&token.id, &user.id, &user.username, &token.refreshed_at)?
            .0;
        // endregion

        Ok((access_token, refresh_token))
    }

    pub async fn validate_token(&self, token: &str) -> AppResult<Claims> {
        let claims = self.jwt.validate(token)?;

        let token = self.token_dao.get(&claims.jti).await.map_err(|err| {
            err.status_code(StatusCode::FORBIDDEN)
                .message("invalid token".to_string())
        })?;
        if claims.sub != token.user_id {
            Err(AppError::new(StatusCode::FORBIDDEN).message("invalid sub".to_string()))
        } else if claims.nbf != token.refreshed_at.timestamp() {
            Err(AppError::new(StatusCode::FORBIDDEN)
                .message("expired token by refresh".to_string()))
        } else {
            Ok(claims)
        }
    }

    pub async fn delete_token(&self, token_id: &Uuid) -> AppResult {
        self.token_dao.delete(token_id).await
    }
}

#[derive(Debug, Deserialize)]
struct ExchangeCodeResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Deserialize)]
struct TwitchUser {
    id: String,
    login: String,
}

#[derive(Debug, Deserialize)]
struct TwitchUserResponse {
    data: Vec<TwitchUser>,
}
