mod adapters;
mod domain;
mod ports;

use adapters::api::handlers::config_handler;
use adapters::repositories::in_memory_repository::InMemoryUserRepository;
use domain::services::user_service::{UserService, UserServiceImpl};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let repository = InMemoryUserRepository::new();
    let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(repository));

    let app = config_handler(user_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
