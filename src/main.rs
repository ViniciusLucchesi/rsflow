mod core;
mod adapters;
mod infrastructure;

use std::sync::Arc;
use core::services::{
    group_service::GroupService, 
    user_service::UserService,
    user_group_service::UserGroupService,
};
use adapters::repositories::in_memory_repository;
use infrastructure::api::routes::build_routes;

#[tokio::main]
async fn main() {
    let user_repository = in_memory_repository::InMemoryUserRepository::new();
    let group_repository = in_memory_repository::InMemoryGroupRepository::new();
    let user_group_repository = in_memory_repository::InMemoryUserGroupRepository::new();

    let user_service = Arc::new(UserService::new(user_repository));
    let group_service = Arc::new(GroupService::new(group_repository));
    let user_group_service = Arc::new(UserGroupService::new(user_group_repository, user_service.clone(), group_service.clone()));

    let app = build_routes(user_service, group_service, user_group_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}