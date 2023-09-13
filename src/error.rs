use anyhow::{anyhow, Error};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[allow(dead_code)]
pub type AppResult<T = ()> = Result<T, AppError>;

#[derive(Debug)]
pub struct AppError {
    pub status_code: StatusCode,
    pub message: Option<String>,
    pub cause: Error,
}

#[allow(dead_code)]
impl AppError {
    pub fn new(status_code: StatusCode, error: String) -> Self {
        AppError {
            status_code: status_code,
            message: None,
            cause: anyhow!(error),
        }
    }
    pub fn new_with_message(status_code: StatusCode, message: String, error: String) -> Self {
        AppError {
            status_code: status_code,
            message: Some(message),
            cause: anyhow!(error),
        }
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn clear_message(mut self) -> Self {
        self.message = None;
        self
    }

    pub fn cause(mut self, cause: Error) -> Self {
        self.cause = cause;
        self
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Something went wrong: {}", self.cause);
        (
            self.status_code,
            self.message.unwrap_or("Unexpected error".to_owned()),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: None,
            cause: err.into(),
        }
    }
}
