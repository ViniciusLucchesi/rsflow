use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::ports::database::DatabaseError;

#[derive(Debug)]
pub enum ApiError {
    Database(DatabaseError),
    Validation(String),
}

impl From<DatabaseError> for ApiError {
    fn from(err: DatabaseError) -> Self {
        ApiError::Database(err)
    }
}

impl From<&'static str> for ApiError {
    fn from(err: &'static str) -> Self {
        ApiError::Validation(err.to_string())
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Database(e) => write!(f, "{}", e),
            ApiError::Validation(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Database(db_err) => {
                let status = match db_err {
                    DatabaseError::UserNotFound | DatabaseError::NotFoundError => {
                        StatusCode::NOT_FOUND
                    }
                    DatabaseError::UserAlreadyExists => StatusCode::CONFLICT,
                    DatabaseError::ConnectionError
                    | DatabaseError::WriteLockError
                    | DatabaseError::ReadLockError => StatusCode::INTERNAL_SERVER_ERROR,
                };
                (status, db_err.to_string()).into_response()
            }
            ApiError::Validation(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn database_error_maps_to_status_code() {
        let err = ApiError::from(DatabaseError::UserAlreadyExists);
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }
}
