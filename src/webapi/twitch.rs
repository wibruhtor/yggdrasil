use std::{collections::HashMap, ops::Add, sync::Arc};

use axum::http::StatusCode;
use chrono::{Duration, NaiveDateTime, Utc};
use serde::Deserialize;
use tokio::sync::RwLock;
use tracing::Instrument;

use crate::{
    config::twitch::Twitch,
    domain::TwitchUserInfo,
    error::{AppError, AppResult},
};

const TOKEN_URL: &str = "https://id.twitch.tv/oauth2/token";
const USERS_URL: &str = "https://api.twitch.tv/helix/users";

pub struct TwitchWebApi {
    twitch_config: Arc<Twitch>,
    token: RwLock<Token>,
}

impl TwitchWebApi {
    pub fn new(twitch_config: Arc<Twitch>) -> Arc<Self> {
        Arc::new(TwitchWebApi {
            twitch_config,
            token: RwLock::new(Token {
                token: None,
                expired_at: Utc::now().naive_utc(),
            }),
        })
    }

    pub async fn get_user_info(&self, login: &str) -> AppResult<TwitchUserInfo> {
        let span = tracing::debug_span!("get user info");

        let client = reqwest::Client::new();

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("login", login);

        let url = reqwest::Url::parse_with_params(USERS_URL, params).map_err(|e| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                .message("fail parse users url".to_string())
                .cause(e.into())
        })?;
        let access_token = self.get_access_token().instrument(span.clone()).await?;

        let resp = client
            .get(url)
            .bearer_auth(access_token)
            .header("Client-Id", &self.twitch_config.client_id)
            .send()
            .instrument(span.clone())
            .await
            .map_err(|e| {
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .message("fail get user info".to_string())
                    .cause(e.into())
            })?;

        if resp.status() != StatusCode::OK {
            return Err(
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR).message(format!(
                    "fail get user info with status code: {}",
                    resp.status().as_u16()
                )),
            );
        }

        let get_user_info_response = resp
            .json::<GetUserInfoResponse>()
            .instrument(span)
            .await
            .map_err(|e| {
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .message("fail parse json of get user info response".to_string())
                    .cause(e.into())
            })?;

        match get_user_info_response.data.first() {
            Some(info) => Ok(info.clone()),
            None => Err(AppError::new(StatusCode::NOT_FOUND).message("not found".to_string())),
        }
    }

    async fn get_access_token(&self) -> AppResult<String> {
        let span = tracing::debug_span!("get app access token");

        let token = self.token.read().await;
        match &token.token {
            Some(access_token) => {
                if token.expired_at < Utc::now().add(Duration::minutes(-1)).naive_utc() {
                    drop(token);
                    return self.fetch_access_token().instrument(span).await;
                } else {
                    Ok(access_token.clone())
                }
            }
            None => {
                drop(token);
                self.fetch_access_token().await
            }
        }
    }

    async fn fetch_access_token(&self) -> AppResult<String> {
        let span = tracing::debug_span!("fetch app access token");

        let client = reqwest::Client::new();

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("client_id", &self.twitch_config.client_id);
        params.insert("client_secret", &self.twitch_config.client_secret);
        params.insert("grant_type", "client_credentials");

        let resp = client
            .post(TOKEN_URL)
            .form(&params)
            .send()
            .instrument(span.clone())
            .await
            .map_err(|e| {
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .message("fail get app access token".to_string())
                    .cause(e.into())
            })?;

        if resp.status() != StatusCode::OK {
            return Err(
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR).message(format!(
                    "fail get app access token with status code: {}",
                    resp.status().as_u16()
                )),
            );
        }

        let get_app_access_token_response = resp
            .json::<GetAppAccessTokenResponse>()
            .instrument(span)
            .await
            .map_err(|e| {
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .message("fail parse json of get app access token response".to_string())
                    .cause(e.into())
            })?;

        let expired_at = Utc::now()
            .add(Duration::seconds(get_app_access_token_response.expires_in))
            .naive_utc();

        let mut blocking = self.token.write().await;
        blocking.token = Some(get_app_access_token_response.access_token.clone());
        blocking.expired_at = expired_at;

        Ok(get_app_access_token_response.access_token)
    }
}

#[derive(Debug)]
struct Token {
    token: Option<String>,
    expired_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
struct GetAppAccessTokenResponse {
    access_token: String,
    expires_in: i64,
}

#[derive(Deserialize, Debug, Clone)]
struct GetUserInfoResponse {
    data: Vec<TwitchUserInfo>,
}
