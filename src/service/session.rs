use std::sync::Arc;

use uuid::Uuid;

use crate::{dao::TokenDao, domain::Token, error::AppResult};

pub struct SessionService {
    token_dao: Arc<TokenDao>,
}

impl SessionService {
    pub fn new(token_dao: Arc<TokenDao>) -> Self {
        SessionService { token_dao }
    }

    pub async fn get_all_sessions(&self, user_id: &str) -> AppResult<Vec<Token>> {
        tracing::debug!("get all sessions by user id");
        let tokens = self.token_dao.get_all_by_user_id(user_id).await?;
        Ok(tokens)
    }

    pub async fn delete_session(&self, user_id: &str, token_id: &Uuid) -> AppResult {
        tracing::debug!("delete session by user id and token id");
        self.token_dao
            .delete_with_user_id(token_id, user_id)
            .await?;

        Ok(())
    }

    pub async fn delete_all_sessions_exclude_current(
        &self,
        user_id: &str,
        token_id: &Uuid,
    ) -> AppResult {
        tracing::debug!("delete all sessions by user id exclude one token id");
        self.token_dao
            .delete_all_by_user_id_exclude_one(user_id, token_id)
            .await?;

        Ok(())
    }
}
