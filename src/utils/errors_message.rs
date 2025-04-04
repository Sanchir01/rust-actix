use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    HashingError,
    InvalidHashFormat,
    InvalidToken,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    NotFoundUserPhone,
    TokenNotProvided,
    NotFoundUserId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub messgae: String,
}

impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::WrongCredentials => "Email or password is wrong".to_string(),
            ErrorMessage::EmailExist => "A user with this email already exists".to_string(),
            ErrorMessage::UserNoLongerExist => {
                "User belonging to this token no longer exists".to_string()
            }
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::HashingError => "Error while hashing password".to_string(),
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(max_length) => {
                format!("Password must not be more than {} characters", max_length)
            }
            ErrorMessage::InvalidToken => "Authentication token is invalid or expired".to_string(),
            ErrorMessage::TokenNotProvided => {
                "You are not logged in, please provide a token".to_string()
            }
            ErrorMessage::NotFoundUserId => "Not Found User Id".to_string(),
            ErrorMessage::NotFoundUserPhone => "Not Found User Phone".to_string(),
        }
    }
}
impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
