use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::core::models::user_model::User;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, ToSchema)]
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