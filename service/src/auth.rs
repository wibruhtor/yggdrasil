use std::sync::Arc;

use tracing::instrument;
use uuid::Uuid;

use dao::{TokenDao, TwitchDataDao, UserDao};
use twitch_api::{Scope, TwitchApi};
use types::error::AppResult;
use utils::jwt::{Claims, JwtMaker};

pub struct AuthService {
    jwt: JwtMaker,
    twitch_api: Arc<TwitchApi>,
    user_dao: Arc<UserDao>,
    twitch_data_dao: Arc<TwitchDataDao>,
    token_dao: Arc<TokenDao>,
    scope: Vec<Scope>,
}

impl AuthService {
    pub fn new(
        jwt: JwtMaker,
        twitch_api: Arc<TwitchApi>,
        user_dao: Arc<UserDao>,
        twitch_data_dao: Arc<TwitchDataDao>,
        token_dao: Arc<TokenDao>,
    ) -> Self {
        AuthService {
            jwt,
            twitch_api,
            user_dao,
            twitch_data_dao,
            token_dao,
            scope: vec![],
        }
    }

    #[instrument(skip_all)]
    pub fn get_authorize_url(&self) -> String {
        self.twitch_api.get_authorize_url(self.scope.clone())
    }

    #[instrument(skip(self, code))]
    pub async fn exchange_code(
        &self,
        code: &str,
        user_agent: &str,
        ip: &str,
    ) -> AppResult<(String, String)> {
        let (token, info) = self.twitch_api.exchange_code(code).await?;

        let user = self.user_dao.get_or_create(&info.id, &info.login).await?;
        self.twitch_data_dao
            .create_or_update(&user.id, &token.refresh_token)
            .await?;

        let token = self.token_dao.create(&user.id, user_agent, ip).await?;

        let access_token = self
            .jwt
            .generate_access_token(&token.id, &user.id, &user.username, &token.refreshed_at)?
            .0;
        let refresh_token = self
            .jwt
            .generate_refresh_token(&token.id, &user.id, &user.username, &token.refreshed_at)?
            .0;

        Ok((access_token, refresh_token))
    }

    #[instrument(skip_all)]
    pub async fn validate_token(&self, token: &str) -> AppResult<Claims> {
        let claims = self.jwt.validate(token)?;

        let token = self
            .token_dao
            .get(&claims.jti)
            .await
            .map_err(|_| JwtMaker::INVALID_TOKEN_ERROR)?;

        if claims.sub != token.user_id {
            Err(JwtMaker::INVALID_TOKEN_ERROR)
        } else if claims.nbf != token.refreshed_at.timestamp() {
            Err(JwtMaker::EXPIRED_TOKEN_ERROR)
        } else {
            Ok(claims)
        }
    }

    #[instrument(skip_all)]
    pub async fn revoke_token(&self, token_id: &Uuid) -> AppResult {
        self.token_dao.delete(token_id).await
    }

    #[instrument(skip(self))]
    pub async fn refresh_token(&self, claims: &Claims) -> AppResult<(String, String)> {
        let token = self.token_dao.refresh(&claims.jti).await?;

        let access_token = self
            .jwt
            .generate_access_token(
                &token.id,
                &claims.sub,
                &claims.username,
                &token.refreshed_at,
            )?
            .0;
        let refresh_token = self
            .jwt
            .generate_refresh_token(
                &token.id,
                &claims.sub,
                &claims.username,
                &token.refreshed_at,
            )?
            .0;

        Ok((access_token, refresh_token))
    }
}
