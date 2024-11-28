use crate::core::interfaces::database::DatabaseError;

pub mod user_controller;
pub mod group_controller;

pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn new(message: &str) -> Self {
        ErrorResponse {
            message: message.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.message
    }
}

impl From<DatabaseError> for ErrorResponse {
    fn from(error: DatabaseError) -> Self {
        ErrorResponse {
            message: error.to_string(),
        }
    }
}

