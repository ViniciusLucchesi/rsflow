pub mod user_routes;
pub mod group_routes;
pub mod user_group_routers;


use axum::response::IntoResponse;
use axum::Router;
use std::sync::Arc;
use crate::core::interfaces::database::DatabaseError;
use crate::core::services::group_service::GroupService;
use crate::core::services::user_service::UserService;
use crate::core::services::user_group_service::UserGroupService;


pub fn build_routes(user: Arc<UserService>, group: Arc<GroupService>, user_group: Arc<UserGroupService>) -> Router {    
    Router::new()
        .nest("/users", user_routes::build_routes(user))
        .nest("/groups", group_routes::build_routes(group))
        .nest("/user-groups", user_group_routers::build_routes(user_group))
}


struct ErrorResponse {
    message: String,
}

impl From<DatabaseError> for ErrorResponse {
    fn from(error: DatabaseError) -> Self {
        ErrorResponse {
            message: error.to_string(),
        }
    }
}