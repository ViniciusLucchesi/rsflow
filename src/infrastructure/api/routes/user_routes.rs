use axum::{
    routing::{get, post},
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
    Router
};
use std::sync::Arc;
use utoipa::path;

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
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User successfully created", body = UserResponse),
        (status = 400, description = "Validation error")
    ),
    tag = "Users",
    summary = "Create a new user",
    description = "Valida que o nome e o email não estejam vazios e que o email contenha '@'"
)]
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

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = String, Path, description = "Identificador do usuário")
    ),
    responses(
        (status = 200, description = "Usuário encontrado", body = UserResponse),
        (status = 404, description = "Usuário não encontrado")
    ),
    tag = "Users",
    summary = "Obter usuário por id",
    description = "Retorna o usuário correspondente ao id informado"
)]
pub async fn get_user_by_id(
    State(service): State<Arc<UserService>>,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, String> {
    let user = service.get_user_by_id(&id).map_err(|e| e.to_string())?;
    Ok(Json(UserResponse::from(user)))
}

#[utoipa::path(
    get,
    path = "/users/all",
    responses(
        (status = 200, description = "Lista todos os usuários", body = [UserResponse])
    ),
    tag = "Users",
    summary = "Listar usuários",
    description = "Retorna todos os usuários cadastrados"
)]
pub async fn get_all_users(State(service): State<Arc<UserService>>) -> Result<Json<Vec<UserResponse>>, String> {
    let users = service.get_all_users().map_err(|e| e.to_string())?;
    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

