use rsflow::adapters::api::handlers::{config_handler, UserResponse};
use rsflow::adapters::repositories::in_memory_repository::InMemoryUserRepository;
use rsflow::domain::services::user_service::{UserService, UserServiceImpl};
use std::sync::Arc;
use tokio::task::JoinHandle;

async fn spawn_app() -> (String, JoinHandle<()>) {
    let repo = InMemoryUserRepository::new();
    let service: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(repo));
    let app = config_handler(service);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    (format!("http://{}", addr), server)
}

#[tokio::test]
async fn post_users_returns_success() {
    let (address, server) = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({"name": "Alice", "email": "alice@example.com"});
    let resp = client
        .post(&format!("{}/users", address))
        .json(&body)
        .send()
        .await
        .unwrap();

    assert!(resp.status().is_success());
    let created: UserResponse = resp.json().await.unwrap();
    assert_eq!(created.name, "Alice");
    server.abort();
}

#[tokio::test]
async fn get_user_returns_success() {
    let (address, server) = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({"name": "Bob", "email": "bob@example.com"});
    let resp = client
        .post(&format!("{}/users", address))
        .json(&body)
        .send()
        .await
        .unwrap();
    let created: UserResponse = resp.json().await.unwrap();

    let resp = client
        .get(&format!("{}/users/{}", address, created.id))
        .send()
        .await
        .unwrap();

    assert!(resp.status().is_success());
    let fetched: UserResponse = resp.json().await.unwrap();
    assert_eq!(fetched.id, created.id);
    server.abort();
}
