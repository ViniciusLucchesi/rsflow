use crate::domain::models::user_model::User;
use crate::ports::database::DatabaseError;
use crate::ports::database::user::UserRepository;


pub struct UserService {
    data: Box<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new<T>(repository: T) -> UserService
    where
        T: UserRepository + Send + Sync + 'static + Clone,
    {
        UserService {
            data: Box::new(repository)
        }
    }

    pub fn get_user_by_id(&self, id: String) -> Result<User, DatabaseError> {
        self.data.get_user_by_id(&id)
    }

    pub fn get_user_by_email(&self, email: &str) -> Result<User, DatabaseError> {
        self.data.get_user_by_email(email)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, DatabaseError> {
        self.data.get_all_users()
    }

    pub fn create_user(&self, user: User) -> Result<User, DatabaseError> {
        self.data.create_user(user)
    }

    pub fn update_user(&self, user: User) -> Result<User, DatabaseError> {
        self.data.update_user(user)
    }

    pub fn delete_user(&self, id: String) -> Result<(), DatabaseError> {
        self.data.delete_user(&id)
    }
}
