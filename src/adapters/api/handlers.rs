// src/api/handlers.rs
use axum::extract::{Path, State};
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::error::ApiError;
use crate::domain::models::user_model::User;
use crate::domain::services::user_service::UserService;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.value().to_string(),
            name: user.name.value().to_string(),
            email: user.email.value().to_string(),
        }
    }
}

pub async fn create_user(
    State(service): State<Arc<dyn UserService>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = User::new(&payload.name, &payload.email).map_err(ApiError::from)?;
    match service.create_user(user) {
        Ok(created_user) => Ok(Json(UserResponse::from(created_user))),
        Err(e) => Err(ApiError::from(e)),
    }
}

pub async fn get_user(
    State(service): State<Arc<dyn UserService>>,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = service.get_user_by_id(id).map_err(ApiError::from)?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn get_all_users(
    State(service): State<Arc<dyn UserService>>,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    let users = service.get_all_users().map_err(ApiError::from)?;
    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

pub fn config_handler(service: Arc<dyn UserService>) -> Router {
    Router::new()
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user))
        .route("/all-users", get(get_all_users))
        .with_state(service)
}
