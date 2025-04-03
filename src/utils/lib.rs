use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, SaltString, rand_core::OsRng},
};
use serde_json::{Value, json};
use validator::ValidationErrors;

use super::errors_message::ErrorMessage;

const MAX_PASSWORD_LENGTH: usize = 64;

pub fn hashing_passwortd(password: impl Into<String>) -> Result<String, ErrorMessage> {
    let password = password.into();

    if password.is_empty() {
        return Err(ErrorMessage::EmptyPassword);
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ErrorMessage::HashingError)?
        .to_string();

    Ok(hashed_password)
}
pub fn format_validation_errors(errors: &ValidationErrors) -> Value {
    let mut error_map = serde_json::Map::new();

    for (field, errs) in errors.field_errors() {
        let messages: Vec<String> = errs
            .iter()
            .filter_map(|e| e.message.as_ref().map(|msg| msg.to_string()))
            .collect();
        error_map.insert(field.to_string(), json!(messages));
    }

    json!(error_map)
}
