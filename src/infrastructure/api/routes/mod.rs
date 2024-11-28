pub mod user_routes;
pub mod group_routes;


use axum::Router;
use std::sync::Arc;
use crate::core::services::group_service::GroupService;
use crate::core::services::user_service::UserService;


pub fn build_routes(user: Arc<UserService>, group: Arc<GroupService>) -> Router {    
    Router::new()
        .nest("/users", user_routes::build_routes(user))
        .nest("/groups", group_routes::build_routes(group))
}