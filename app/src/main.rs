use std::sync::Arc;

use config::Config;
use dao::{BanWordFilterDao, ChatSettingsDao, Database, TokenDao, TwitchDataDao, UserDao};
use service::{AuthService, BanWordService, ChatService, SessionService, TwitchService};
use twitch_api::TwitchApi;
use types::error::AppResult;
use utils::crypt::Crypt;
use utils::jwt::JwtMaker;
use web_server::Services;

#[tokio::main]
async fn main() -> AppResult {
    let config = Config::load()?;
    init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    let jwt = JwtMaker::new(config.jwt_config().secret());
    let crypt = Crypt::new(config.crypt_config().secret());

    let database = Database::new(config.database_config().postgres_url()).await?;

    let user_dao = Arc::new(UserDao::new(database.postgres()));
    let token_dao = Arc::new(TokenDao::new(database.postgres(), crypt));
    let twitch_data_dao = Arc::new(TwitchDataDao::new(database.postgres()));
    let ban_word_filter_dao = Arc::new(BanWordFilterDao::new(database.postgres()));
    let chat_settings_dao = Arc::new(ChatSettingsDao::new(database.postgres()));

    let twitch_api = Arc::new(TwitchApi::new(config.twitch_config().clone()));

    let auth_service = Arc::new(AuthService::new(
        jwt,
        twitch_api.clone(),
        user_dao.clone(),
        twitch_data_dao.clone(),
        token_dao.clone(),
    ));
    let session_service = Arc::new(SessionService::new(token_dao.clone()));
    let twitch_service = Arc::new(TwitchService::new(twitch_api.clone()));
    let ban_word_service = Arc::new(BanWordService::new(ban_word_filter_dao.clone()));
    let chat_service = Arc::new(ChatService::new(chat_settings_dao.clone()));

    web_server::run(
        config.http_config().clone(),
        Services {
            auth: auth_service,
            session: session_service,
            twitch: twitch_service,
            ban_word: ban_word_service,
            chat: chat_service,
        },
    )
    .await;
    Ok(())
}
