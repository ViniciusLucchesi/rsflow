use axum::{
    routing::{get, post}, 
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
    Router
};
use std::sync::Arc;

use crate::adapters::controllers::user_controller::{CreateUserRequest, UserResponse};
use crate::core::services::user_service::UserService;
use crate::core::models::user_model::User;


// Factory function
pub fn build_routes(service: Arc<UserService>) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/:id", get(get_user_by_id))	
        .route("/all", get(get_all_users))
        .with_state(service)
}


// Controller functions
pub async fn create_user(
    State(service): State<Arc<UserService>>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let user = User::new(&payload.name, &payload.email).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    match service.create_user(user) {
        Ok(created_user) => Ok((StatusCode::CREATED, Json(UserResponse::from(created_user)))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn get_user_by_id(
    State(service): State<Arc<UserService>>,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, String> {
    let user = service.get_user_by_id(&id).map_err(|e| e.to_string())?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn get_all_users(State(service): State<Arc<UserService>>) -> Result<Json<Vec<UserResponse>>, String> {
    let users = service.get_all_users().map_err(|e| e.to_string())?;
    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

