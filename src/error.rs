use anyhow::Error;
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
    pub cause: Option<Error>,
}

#[allow(dead_code)]
impl AppError {
    pub fn new(status_code: StatusCode) -> Self {
        AppError {
            status_code,
            message: None,
            cause: None,
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
        self.cause = Some(cause);
        self
    }

    pub fn clear_cause(mut self) -> Self {
        self.cause = None;
        self
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match &self.message {
            Some(message) => match self.cause {
                Some(cause) => tracing::error!({ cause = format!("{}", cause) }, "{}", message),
                None => {
                    tracing::error!(message)
                }
            },
            None => match self.cause {
                Some(cause) => {
                    tracing::error!({ cause = format!("{}", cause) }, "unexpected error")
                }
                None => {
                    tracing::error!("unexpected error")
                }
            },
        }

        (
            self.status_code,
            self.message.unwrap_or("unexpected error".to_owned()),
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
            cause: Some(err.into()),
        }
    }
}
