use std::sync::Arc;

use axum::http::StatusCode;
use tracing::instrument;
use uuid::Uuid;

use dao::BanWordFilterDao;
use types::domain::{BanWordFilter, BanWordFilterInfo, UpdateBanWordFilter};
use types::error::{AppError, AppResult};

pub struct BanWordService {
    ban_word_filter_dao: Arc<BanWordFilterDao>,
}

impl BanWordService {
    pub fn new(ban_word_filter_dao: Arc<BanWordFilterDao>) -> Self {
        BanWordService {
            ban_word_filter_dao,
        }
    }

    #[instrument(skip(self))]
    pub async fn create_filter(&self, user_id: &str, name: &str) -> AppResult<BanWordFilter> {
        self.ban_word_filter_dao.create(user_id, name).await
    }

    #[instrument(skip(self))]
    pub async fn get_filter(&self, ban_word_filter_id: &Uuid) -> AppResult<BanWordFilter> {
        self.ban_word_filter_dao.get(ban_word_filter_id).await
    }

    #[instrument(skip(self))]
    pub async fn get_all_filters(&self, user_id: &str) -> AppResult<Vec<BanWordFilterInfo>> {
        self.ban_word_filter_dao.get_all_by_user_id(user_id).await
    }

    #[instrument(skip(self))]
    pub async fn update_filter(
        &self,
        user_id: &str,
        ban_word_filter_id: &Uuid,
        update_ban_word_filter: &UpdateBanWordFilter,
    ) -> AppResult<BanWordFilter> {
        self.check_user_owning_of_filter_by_id(user_id, ban_word_filter_id)
            .await?;

        self.ban_word_filter_dao
            .update(ban_word_filter_id, update_ban_word_filter)
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_filter(&self, user_id: &str, ban_word_filter_id: &Uuid) -> AppResult {
        self.check_user_owning_of_filter_by_id(user_id, ban_word_filter_id)
            .await?;

        self.ban_word_filter_dao.delete(ban_word_filter_id).await
    }

    #[instrument(skip(self))]
    async fn check_user_owning_of_filter_by_id(
        &self,
        user_id: &str,
        ban_word_filter_id: &Uuid,
    ) -> AppResult {
        let is_owner = self
            .ban_word_filter_dao
            .is_belongs_to_user(ban_word_filter_id, user_id)
            .await?;

        if !is_owner {
            return Err(BanWordService::IS_NOT_OWNER_ERROR);
        }

        Ok(())
    }
}

macro_rules! ban_word_service_errors {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $status:expr, $phrase:expr);
        )+
    ) => {
        impl BanWordService {
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

ban_word_service_errors! {
    (IS_NOT_OWNER_ERROR, StatusCode::UNAUTHORIZED, "ban word filter is not your");
}
