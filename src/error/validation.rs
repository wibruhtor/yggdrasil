use axum::http::StatusCode;

use super::AppError;

pub struct ValidationErrorsWrapper(validator::ValidationErrors);

impl From<ValidationErrorsWrapper> for AppError {
    fn from(wrapper: ValidationErrorsWrapper) -> Self {
        wrapper.to_error()
    }
}

impl From<validator::ValidationErrors> for ValidationErrorsWrapper {
    fn from(err: validator::ValidationErrors) -> Self {
        ValidationErrorsWrapper(err)
    }
}

impl ValidationErrorsWrapper {
    fn to_error(self) -> AppError {
        for (field, err) in self.0.field_errors() {
            for err in err {
                let value = match err.params.iter().find(|(key, _)| *key == "value") {
                    Some(v) => v.1.to_string(),
                    None => "".to_string(),
                };
                let options: Vec<String> = err
                    .params
                    .iter()
                    .filter(|(key, _)| *key != "value")
                    .map(|(key, value)| format!("{} {}", key, value.to_string()))
                    .collect();
                let message = if options.len() == 0 {
                    format!("{}({}): {}", field, value, err.code)
                } else {
                    format!("{}({}): {} {}", field, value, err.code, options.join(", "))
                };

                return AppError {
                    status_code: StatusCode::BAD_REQUEST,
                    message: Some(message),
                    cause: Some(self.0.into()),
                };
            }
        }
        return AppError {
            status_code: StatusCode::BAD_REQUEST,
            message: None,
            cause: Some(self.0.into()),
        };
    }
}
