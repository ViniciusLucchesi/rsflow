mod core;
mod adapters;
mod infrastructure;

use std::sync::Arc;
use core::services::user_service::UserService;
use adapters::repositories::in_memory_repository::InMemoryUserRepository;
use infrastructure::api::handlers::{config_handler};

#[tokio::main]
async fn main() {
    let repository = InMemoryUserRepository::new();
    let user_service = Arc::new(UserService::new(repository));

    let app = config_handler(user_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}