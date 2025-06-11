use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(ErrorResponse {
            error: self.message,
        });
        (self.status, body).into_response()
    }
}

use crate::ports::database::DatabaseError;

impl From<DatabaseError> for ApiError {
    fn from(err: DatabaseError) -> Self {
        match err {
            DatabaseError::ConnectionError
            | DatabaseError::WriteLockError
            | DatabaseError::ReadLockError => {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
            DatabaseError::UserNotFound | DatabaseError::NotFoundError => {
                ApiError::new(StatusCode::NOT_FOUND, err.to_string())
            }
            DatabaseError::UserAlreadyExists => {
                ApiError::new(StatusCode::CONFLICT, err.to_string())
            }
        }
    }
}

impl From<&'static str> for ApiError {
    fn from(err: &'static str) -> Self {
        ApiError::new(StatusCode::BAD_REQUEST, err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_user_not_found() {
        let api: ApiError = DatabaseError::UserNotFound.into();
        assert_eq!(api.status, StatusCode::NOT_FOUND);
    }
}
