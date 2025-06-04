use axum::{
    routing::{get, post}, 
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
    Router
};
use std::sync::Arc;
use utoipa::path;
use serde_json::json;
use utoipa::path;

use crate::adapters::controllers::user_group_controller::{CreateUserGroupRequest, UserGroupResponse};
use crate::core::services::user_group_service::UserGroupService;
use crate::core::models::user_group_model::UserGroup;


// Factory function
pub fn build_routes(service: Arc<UserGroupService>) -> Router {
    Router::new()
        .route("/", post(create_user_group))
        .route("/:id", get(get_user_group_by_id))
        .route("/user/:user_id", get(get_user_group_by_user_id))
        .route("/group/:group_id", get(get_user_group_by_group_id))
        .route("/all", get(get_all_user_groups))
        .with_state(service)
}



// Controller functions
#[utoipa::path(
    post,
    path = "/user-groups",
    request_body = CreateUserGroupRequest,
    responses(
        (status = 201, description = "Associação criada", body = UserGroupResponse, example = json!({"id":"f0000000-0000-0000-0000-000000000000","user_id":"11111111-1111-1111-1111-111111111111","group_id":"22222222-2222-2222-2222-222222222222"})),
        (status = 400, description = "Usuário ou grupo inexistente")
    ),
    tag = "UserGroups",
    summary = "Criar associação usuário-grupo",
    description = "Verifica se usuário e grupo existem antes de criar a associação"
)]
pub async fn create_user_group(
    State(service): State<Arc<UserGroupService>>,
    Json(payload): Json<CreateUserGroupRequest>,
) -> impl IntoResponse {
    let user_group = UserGroup::new(&payload.user_id, &payload.group_id).map_err(|e| (StatusCode::BAD_REQUEST, Json(e.to_string())))?;
    match service.create_user_group(user_group) {
        Ok(created_user_group) => Ok((StatusCode::CREATED, Json(UserGroupResponse::from(created_user_group)))),
        Err(error) => Err((StatusCode::BAD_REQUEST, Json(error.to_string()))),
    }
}

pub async fn update_user_group(
    State(service): State<Arc<UserGroupService>>,
    Json(payload): Json<UserGroupResponse>,
) -> impl IntoResponse {
    let user_group = UserGroup::new(&payload.user_id, &payload.group_id).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    match service.update_user_group(user_group) {
        Ok(updated_user_group) => Ok((StatusCode::CREATED, Json(UserGroupResponse::from(updated_user_group)))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn delete_user_group(
    State(service): State<Arc<UserGroupService>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match service.delete_user_group(&id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/user-groups/{id}",
    params(("id" = String, Path, description = "Identificador da relação")),
    responses(
        (status = 200, description = "Relação encontrada", body = UserGroupResponse),
        (status = 404, description = "Relação não encontrada")
    ),
    tag = "UserGroups",
    summary = "Buscar relação por id",
    description = "Retorna a associação usuário-grupo"
)]
pub async fn get_user_group_by_id(
    State(service): State<Arc<UserGroupService>>,
    Path(id): Path<String>,
) -> Result<Json<UserGroupResponse>, String> {
    let user_group = service.get_user_group_by_id(&id).map_err(|e| e.to_string())?;
    Ok(Json(UserGroupResponse::from(user_group)))
}

#[utoipa::path(
    get,
    path = "/user-groups/user/{user_id}",
    params(("user_id" = String, Path, description = "Id do usuário")),
    responses(
        (status = 200, description = "Relações do usuário", body = [UserGroupResponse])
    ),
    tag = "UserGroups",
    summary = "Listar por usuário",
    description = "Recupera associações de um usuário"
)]
pub async fn get_user_group_by_user_id(
    State(service): State<Arc<UserGroupService>>,
    Path(user_id): Path<String>,
) -> Result<Json<Vec<UserGroupResponse>>, String> {
    let user_groups = service.get_user_group_by_user_id(&user_id).map_err(|e| e.to_string())?;
    Ok(Json(user_groups.into_iter().map(UserGroupResponse::from).collect()))
}

#[utoipa::path(
    get,
    path = "/user-groups/group/{group_id}",
    params(("group_id" = String, Path, description = "Id do grupo")),
    responses(
        (status = 200, description = "Relações do grupo", body = [UserGroupResponse])
    ),
    tag = "UserGroups",
    summary = "Listar por grupo",
    description = "Recupera associações de um grupo"
)]
pub async fn get_user_group_by_group_id(
    State(service): State<Arc<UserGroupService>>,
    Path(group_id): Path<String>,
) -> Result<Json<Vec<UserGroupResponse>>, String> {
    let user_groups = service.get_user_group_by_group_id(&group_id).map_err(|e| e.to_string())?;
    Ok(Json(user_groups.into_iter().map(UserGroupResponse::from).collect()))
}

#[utoipa::path(
    get,
    path = "/user-groups/all",
    responses((status = 200, description = "Lista de todas as associações", body = [UserGroupResponse])),
    tag = "UserGroups",
    summary = "Listar todas as relações",
    description = "Retorna todas as ligações usuário-grupo"
)]
pub async fn get_all_user_groups(State(service): State<Arc<UserGroupService>>) -> Result<Json<Vec<UserGroupResponse>>, String> {
    let user_groups = service.get_all_user_groups().map_err(|e| e.to_string())?;
    Ok(Json(user_groups.into_iter().map(UserGroupResponse::from).collect()))
}
