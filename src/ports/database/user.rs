use super::DatabaseError;
use crate::domain::models::user_model::User;

pub trait UserRepository {
    fn get_user_by_id(&self, id: &str) -> Result<User, DatabaseError>;
    fn get_user_by_email(&self, email: &str) -> Result<User, DatabaseError>;
    fn get_all_users(&self) -> Result<Vec<User>, DatabaseError>;

    fn create_user(&self, user: User) -> Result<User, DatabaseError>;
    fn update_user(&self, user: User) -> Result<User, DatabaseError>;
    fn delete_user(&self, id: &str) -> Result<(), DatabaseError>;
}
