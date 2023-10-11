use std::sync::Arc;

use tracing::instrument;
use uuid::Uuid;

use dao::TokenDao;
use types::domain::Token;
use types::error::AppResult;

pub struct SessionService {
    token_dao: Arc<TokenDao>,
}

impl SessionService {
    pub fn new(token_dao: Arc<TokenDao>) -> Self {
        SessionService { token_dao }
    }

    #[instrument(skip(self))]
    pub async fn get_all_sessions(&self, user_id: &str) -> AppResult<Vec<Token>> {
        self.token_dao.get_all_by_user_id(user_id).await
    }

    #[instrument(skip(self))]
    pub async fn delete_session(&self, user_id: &str, token_id: &Uuid) -> AppResult {
        self.token_dao.delete_with_user_id(token_id, user_id).await
    }

    #[instrument(skip(self))]
    pub async fn delete_all_sessions_exclude_current(
        &self,
        user_id: &str,
        token_id: &Uuid,
    ) -> AppResult {
        self.token_dao
            .delete_all_by_user_id_exclude_one(user_id, token_id)
            .await
    }
}
