use std::collections::BTreeMap;

use axum::http::StatusCode;
use serde_json::{Map, Value};
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

use super::AppError;

pub struct ValidationErrorsWrapper(ValidationErrors);

impl From<ValidationErrorsWrapper> for AppError {
    fn from(wrapper: ValidationErrorsWrapper) -> Self {
        wrapper.to_error()
    }
}

impl From<ValidationErrors> for ValidationErrorsWrapper {
    fn from(err: ValidationErrors) -> Self {
        ValidationErrorsWrapper(err)
    }
}

impl ValidationErrorsWrapper {
    fn to_error(&self) -> AppError {
        AppError::new(StatusCode::BAD_REQUEST)
            .message("fail validate".to_string())
            .other_map(self.validation_error_to_map(&self.0))
    }

    fn validation_error_to_map(&self, err: &ValidationErrors) -> Map<String, Value> {
        let mut map = Map::new();
        for (field, err) in err.errors() {
            match err.clone() {
                ValidationErrorsKind::Struct(err) => {
                    let struct_map = self.struct_error_to_map(&field, err);
                    for (key, value) in struct_map {
                        map.insert(key, value);
                    }
                }
                ValidationErrorsKind::List(err) => {
                    let list_map = self.list_error_to_map(&field, err);
                    for (key, value) in list_map {
                        map.insert(key, value);
                    }
                }
                ValidationErrorsKind::Field(err) => {
                    let field_map = self.field_error_to_map(err);
                    map.insert(field.to_string(), Value::Object(field_map));
                }
            }
        }
        map
    }

    fn list_error_to_map(
        &self,
        parent_field: &str,
        err: BTreeMap<usize, Box<ValidationErrors>>,
    ) -> Map<String, Value> {
        let mut map = Map::new();
        for (index, err) in err {
            let field_map = self.validation_error_to_map(&err);
            for (key, value) in field_map {
                map.insert(format!("{}[{}].{}", parent_field, index, key), value);
            }
        }
        map
    }

    fn field_error_to_map(&self, err: Vec<ValidationError>) -> Map<String, Value> {
        let mut map = Map::new();
        for err in err {
            let value = err.params.iter().find(|(key, _)| *key == "value");
            if value.is_some() {
                map.insert("value".to_string(), value.unwrap().1.to_owned());
            }
            map.insert("code".to_string(), Value::String(err.code.to_string()));
            let mut params_map = Map::new();
            err.params
                .iter()
                .filter(|(key, _)| *key != "value")
                .for_each(|(key, value)| {
                    let key = key.as_ref().to_owned();
                    params_map.insert(key, value.to_owned());
                });
            if !params_map.is_empty() {
                map.insert("params".to_string(), Value::Object(params_map));
            }
        }
        map
    }

    fn struct_error_to_map(
        &self,
        parent_field: &str,
        err: Box<ValidationErrors>,
    ) -> Map<String, Value> {
        let mut map = Map::new();
        let field_map = self.validation_error_to_map(&err);
        for (key, value) in field_map {
            map.insert(format!("{}.{}", parent_field, key), value);
        }
        map
    }
}
