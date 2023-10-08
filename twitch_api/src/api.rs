use std::collections::HashMap;
use std::ops::Add;
use std::sync::RwLock;

use axum::http::StatusCode;
use chrono::{Duration, Utc};
use reqwest::{Client, RequestBuilder, Response, Url};
use serde::de::DeserializeOwned;

use config::TwitchConfig;
use types::error::{AppError, AppResult};
use types::twitch::{Badge, Emote, UserInfo};

use crate::consts::{AUTHORIZE_URL, HELIX_URL, TOKEN_URL};
use crate::domain::{
    AppAccessToken, GetAppAccessTokenResponse, GetBadgesResponse, GetEmotesResponse,
    GetUserInfoResponse, GetUserTokenResponse, Scope,
};

pub struct TwitchApi {
    twitch_config: TwitchConfig,
    token: RwLock<AppAccessToken>,
}

impl TwitchApi {
    pub fn new(twitch_config: TwitchConfig) -> Self {
        TwitchApi {
            twitch_config,
            token: RwLock::default(),
        }
    }

    pub fn get_authorize_url(&self, scope: Vec<Scope>) -> String {
        let scope: Vec<String> = scope.iter().map(Scope::string).collect();
        format!(
            "{}?client_id={}&force_verify=true&redirect_uri={}&response_type=code&scope={}",
            AUTHORIZE_URL,
            self.twitch_config.client_id(),
            self.twitch_config.callback_url(),
            scope.join(" ")
        )
    }

    pub async fn get_user_info(&self, login: &str) -> AppResult<UserInfo> {
        let query_params: HashMap<&str, &str> = HashMap::from([("login", login)]);

        let request = self.request("/users", Some(query_params)).await?;

        let response = request
            .send()
            .await
            .map_err(|e| TwitchApi::FAIL_GET_USER_INFO_ERROR.clone().cause(e.into()))?;

        if !response.status().is_success() {
            return Err(TwitchApi::FAIL_REQUEST_WITH_STATUS_CODE_ERROR
                .clone()
                .message(&format!(
                    "fail get user info with status code: {}",
                    response.status().as_u16()
                )));
        }

        let get_user_info_response = self.parse_json::<GetUserInfoResponse>(response).await?;

        match get_user_info_response.data.first() {
            Some(info) => Ok(info.clone()),
            None => Err(TwitchApi::NOT_FOUND_USER_INFO_ERROR),
        }
    }

    pub async fn get_global_emotes(&self) -> AppResult<Vec<Emote>> {
        let request = self.request("/chat/emotes/global", None).await?;

        let response = request.send().await.map_err(|e| {
            TwitchApi::FAIL_GET_GLOBAL_EMOTES_ERROR
                .clone()
                .cause(e.into())
        })?;

        if !response.status().is_success() {
            return Err(TwitchApi::FAIL_REQUEST_WITH_STATUS_CODE_ERROR
                .clone()
                .message(&format!(
                    "fail get global emotes with status code: {}",
                    response.status().as_u16()
                )));
        }

        let get_emotes_response = self.parse_json::<GetEmotesResponse>(response).await?;

        let emotes = get_emotes_response.to_twitch_emotes();

        Ok(emotes)
    }

    pub async fn get_channel_emotes(&self, channel_id: &str) -> AppResult<Vec<Emote>> {
        let query_params: HashMap<&str, &str> = HashMap::from([("broadcaster_id", channel_id)]);

        let request = self.request("/chat/emotes", Some(query_params)).await?;

        let response = request.send().await.map_err(|e| {
            TwitchApi::FAIL_GET_CHANNEL_EMOTES_ERROR
                .clone()
                .cause(e.into())
        })?;

        if !response.status().is_success() {
            return Err(TwitchApi::FAIL_REQUEST_WITH_STATUS_CODE_ERROR
                .clone()
                .message(&format!(
                    "fail get channel emotes with status code: {}",
                    response.status().as_u16()
                )));
        }

        let get_emotes_response = self.parse_json::<GetEmotesResponse>(response).await?;

        let emotes = get_emotes_response.to_twitch_emotes();

        Ok(emotes)
    }

    pub async fn get_global_badges(&self) -> AppResult<Vec<Badge>> {
        let request = self.request("/chat/badges/global", None).await?;

        let response = request.send().await.map_err(|e| {
            TwitchApi::FAIL_GET_GLOBAL_BADGES_ERROR
                .clone()
                .cause(e.into())
        })?;

        if !response.status().is_success() {
            return Err(TwitchApi::FAIL_REQUEST_WITH_STATUS_CODE_ERROR
                .clone()
                .message(&format!(
                    "fail get global badges with status code: {}",
                    response.status().as_u16()
                )));
        }

        let get_badges_response = self.parse_json::<GetBadgesResponse>(response).await?;

        let badges = get_badges_response.to_twitch_badges();

        Ok(badges)
    }

    pub async fn get_channel_badges(&self, channel_id: &str) -> AppResult<Vec<Badge>> {
        let query_params: HashMap<&str, &str> = HashMap::from([("broadcaster_id", channel_id)]);

        let request = self.request("/chat/badges", Some(query_params)).await?;

        let response = request.send().await.map_err(|e| {
            TwitchApi::FAIL_GET_CHANNEL_BADGES_ERROR
                .clone()
                .cause(e.into())
        })?;

        if !response.status().is_success() {
            return Err(TwitchApi::FAIL_REQUEST_WITH_STATUS_CODE_ERROR
                .clone()
                .message(&format!(
                    "fail get channel badges with status code: {}",
                    response.status().as_u16()
                )));
        }

        let get_badges_response = self.parse_json::<GetBadgesResponse>(response).await?;

        let badges = get_badges_response.to_twitch_badges();

        Ok(badges)
    }

    pub async fn exchange_code(&self, code: &str) -> AppResult<(GetUserTokenResponse, UserInfo)> {
        let response = self.get_user_token(code).await?;
        let user_info = self
            .get_user_info_by_user_token(&response.access_token)
            .await?;

        Ok((response, user_info))
    }

    async fn get_user_token(&self, code: &str) -> AppResult<GetUserTokenResponse> {
        let form = HashMap::from([
            ("client_id", self.twitch_config.client_id()),
            ("client_secret", self.twitch_config.client_secret()),
            ("redirect_uri", self.twitch_config.callback_url()),
            ("code", code),
            ("grant_type", "authorization_code"),
        ]);

        let client = Client::new();

        let request = client.post(TOKEN_URL).form(&form);

        let response = request
            .send()
            .await
            .map_err(|e| TwitchApi::FAIL_GET_USER_TOKEN_ERROR.clone().cause(e.into()))?;

        if !response.status().is_success() {
            return Err(TwitchApi::FAIL_REQUEST_WITH_STATUS_CODE_ERROR
                .clone()
                .message(&format!(
                    "fail get user token with status code: {}",
                    response.status().as_u16()
                )));
        }

        self.parse_json::<GetUserTokenResponse>(response).await
    }

    async fn get_user_info_by_user_token(&self, token: &str) -> AppResult<UserInfo> {
        let client = Client::new();

        let request = client
            .get(&format!("{}/users", HELIX_URL))
            .bearer_auth(token)
            .header("Client-Id", self.twitch_config.client_id());

        let response = request
            .send()
            .await
            .map_err(|e| TwitchApi::FAIL_GET_USER_INFO_ERROR.clone().cause(e.into()))?;

        if !response.status().is_success() {
            return Err(TwitchApi::FAIL_REQUEST_WITH_STATUS_CODE_ERROR
                .clone()
                .message(&format!(
                    "fail get user info by access token with status code: {}",
                    response.status().as_u16()
                )));
        }

        let get_user_info_response = self.parse_json::<GetUserInfoResponse>(response).await?;

        match get_user_info_response.data.first() {
            Some(info) => Ok(info.clone()),
            None => Err(TwitchApi::NOT_FOUND_USER_INFO_ERROR),
        }
    }

    async fn get_app_access_token(&self) -> AppResult<String> {
        let token = self.token.read().unwrap();
        if !token.is_expired() {
            return Ok(token.token().unwrap());
        }
        let form = HashMap::from([
            ("client_id", self.twitch_config.client_id()),
            ("client_secret", self.twitch_config.client_secret()),
            ("grant_type", "client_credentials"),
        ]);

        let client = Client::new();

        let request = client.post(TOKEN_URL).form(&form);

        let response = request.send().await.map_err(|e| {
            TwitchApi::FAIL_GET_APP_ACCESS_TOKEN_ERROR
                .clone()
                .cause(e.into())
        })?;

        if !response.status().is_success() {
            return Err(TwitchApi::FAIL_REQUEST_WITH_STATUS_CODE_ERROR
                .clone()
                .message(&format!(
                    "fail get app access token with status code: {}",
                    response.status().as_u16()
                )));
        }

        let get_app_access_token_response = self
            .parse_json::<GetAppAccessTokenResponse>(response)
            .await?;

        let expired_at = Utc::now()
            .add(Duration::seconds(get_app_access_token_response.expires_in))
            .naive_utc();

        let mut token = self.token.write().unwrap();
        token.set(&get_app_access_token_response.access_token, &expired_at);
        Ok(get_app_access_token_response.access_token)
    }

    async fn parse_json<T: DeserializeOwned>(&self, response: Response) -> AppResult<T> {
        response.json::<T>().await.map_err(|e| {
            TwitchApi::FAIL_PARSE_JSON_OF_RESPONSE_ERROR
                .clone()
                .cause(e.into())
        })
    }

    async fn request(
        &self,
        path: &str,
        query_params: Option<HashMap<&str, &str>>,
    ) -> AppResult<RequestBuilder> {
        let url = format!("{}{}", HELIX_URL, path);
        let url = match query_params {
            Some(params) => Url::parse_with_params(&url, params),
            None => Url::parse(&url),
        }?;
        let access_token = self.get_app_access_token().await?;

        let client = Client::new();

        Ok(client
            .get(url)
            .bearer_auth(access_token)
            .header("Client-Id", self.twitch_config.client_id()))
    }
}

macro_rules! twitch_api_errors {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $status:expr, $phrase:expr);
        )+
    ) => {
        impl TwitchApi {
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

twitch_api_errors! {
    (FAIL_GET_APP_ACCESS_TOKEN_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail get app access token");
    (FAIL_GET_USER_INFO_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail get user info");
    (NOT_FOUND_USER_INFO_ERROR, StatusCode::NOT_FOUND, "not found user info");
    (FAIL_GET_GLOBAL_EMOTES_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail get global emotes");
    (FAIL_GET_CHANNEL_EMOTES_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail get channel emotes");
    (FAIL_GET_GLOBAL_BADGES_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail get global badges");
    (FAIL_GET_CHANNEL_BADGES_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail get channel badges");
    (FAIL_GET_USER_TOKEN_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail get user token");
    (FAIL_REQUEST_WITH_STATUS_CODE_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail request");
    (FAIL_PARSE_JSON_OF_RESPONSE_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail parse json of response");
    (FAIL_PARSE_URL_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail parse url");
}
