use serde::{Deserialize, Serialize};
use crate::core::models::user_model::User;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.value().to_string(),
            name: user.name.value().to_string(),
            email: user.email.value().to_string(),
        }
    }
}