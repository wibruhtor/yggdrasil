use std::fmt::{Display, Formatter};

use anyhow::Error;
use axum::{
    http::StatusCode,
    Json,
    response::{IntoResponse, Response},
};
use serde_json::{Map, Value};

pub type AppResult<T = ()> = Result<T, AppError>;

#[derive(Debug)]
pub struct AppError {
    pub status_code: StatusCode,
    pub message: Option<&'static str>,
    pub cause: Option<Error>,
    pub other: Option<Map<String, Value>>,
}

impl AppError {
    pub fn new(status_code: StatusCode) -> Self {
        AppError {
            status_code,
            message: None,
            cause: None,
            other: None,
        }
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(Box::leak(message.to_string().into_boxed_str()));
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

    pub fn other(mut self, field: String, value: Value) -> Self {
        self.other = match self.other {
            Some(mut map) => {
                map.insert(field, value);
                Some(map)
            }
            None => {
                let mut map = Map::new();
                map.insert(field, value);
                Some(map)
            }
        };
        self
    }

    pub fn other_map(mut self, map: Map<String, Value>) -> Self {
        self.other = match self.other {
            Some(mut other_map) => {
                map.iter().for_each(|(key, value)| {
                    other_map.insert(key.clone(), value.clone());
                });
                Some(map)
            }
            None => {
                let mut other_map = Map::new();
                map.iter().for_each(|(key, value)| {
                    other_map.insert(key.clone(), value.clone());
                });
                Some(map)
            }
        };

        self
    }

    pub fn clear_other(mut self) -> Self {
        self.other = None;
        self
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let mut map = Map::new();

        map.insert("message".to_string(), Value::String(self.message.unwrap_or("unexpected error").to_string()));
        if self.other.is_some() {
            let other = self.other.unwrap();
            if !other.is_empty() {
                map.insert("other".to_string(), Value::Object(other));
            }
        }

        if map.is_empty() {
            self.status_code.into_response()
        } else {
            (self.status_code, Json(map)).into_response()
        }
    }
}

impl<E> From<E> for AppError
    where
        E: Into<Error>,
{
    fn from(err: E) -> Self {
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR).cause(err.into())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:?} {:?} {:?}",
            self.status_code,
            self.message,
            self.cause,
            self.other
        )
    }
}

impl Default for AppError {
    fn default() -> Self {
        AppError::UNEXPECTED_ERROR.clone()
    }
}

impl Clone for AppError {
    /// On clone set cause and other to None
    fn clone(&self) -> Self {
        AppError {
            status_code: self.status_code.clone(),
            message: self.message.clone(),
            cause: None,
            other: None,
        }
    }
}

macro_rules! app_errors {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $status:expr, $phrase:expr);
        )+
    ) => {
        impl AppError {
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

app_errors! {
    (UNEXPECTED_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "unexpected error");
}