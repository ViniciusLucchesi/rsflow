use super::DatabaseError;
use crate::domain::models::user_model::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn get_user_by_id(&self, id: &str) -> Result<User, DatabaseError>;
    async fn get_user_by_email(&self, email: &str) -> Result<User, DatabaseError>;
    async fn get_all_users(&self) -> Result<Vec<User>, DatabaseError>;

    async fn create_user(&self, user: User) -> Result<User, DatabaseError>;
    async fn update_user(&self, user: User) -> Result<User, DatabaseError>;
    async fn delete_user(&self, id: &str) -> Result<(), DatabaseError>;
}
