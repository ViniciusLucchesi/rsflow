use axum::{
    routing::{get, post}, 
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
    Router
};
use std::sync::Arc;

use crate::adapters::controllers::group_controller::{CreateGroupRequest, GroupResponse};
use crate::core::services::group_service::GroupService;
use crate::core::models::group_model::Group;


// Factory function
pub fn build_routes(service: Arc<GroupService>) -> Router {
    Router::new()
        .route("/", post(create_group))
        .route("/:id", get(get_group_by_id))
        .route("/all-groups", get(get_all_groups))
        .with_state(service)
}


// Controller functions
pub async fn create_group(
    State(service): State<Arc<GroupService>>,
    Json(payload): Json<CreateGroupRequest>,
) -> impl IntoResponse {
    let group = Group::new(&payload.name, &payload.description).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    match service.create_group(group) {
        Ok(created_group) => Ok((StatusCode::CREATED, Json(GroupResponse::from(created_group)))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn get_group_by_id(
    State(service): State<Arc<GroupService>>,
    Path(id): Path<String>,
) -> Result<Json<GroupResponse>, String> {
    let group = service.get_group_by_id(&id).map_err(|e| e.to_string())?;
    Ok(Json(GroupResponse::from(group)))
}


pub async fn get_all_groups(State(service): State<Arc<GroupService>>) -> Result<Json<Vec<GroupResponse>>, String> {
    let groups = service.get_all_groups().map_err(|e| e.to_string())?;
    Ok(Json(groups.into_iter().map(GroupResponse::from).collect()))
}

