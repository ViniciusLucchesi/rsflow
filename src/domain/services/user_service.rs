use crate::domain::models::user_model::User;
use crate::ports::database::user::UserRepository;
use crate::ports::database::DatabaseError;

pub trait UserService: Send + Sync {
    fn get_user_by_id(&self, id: String) -> Result<User, DatabaseError>;
    fn get_user_by_email(&self, email: &str) -> Result<User, DatabaseError>;
    fn get_all_users(&self) -> Result<Vec<User>, DatabaseError>;
    fn create_user(&self, user: User) -> Result<User, DatabaseError>;
    fn update_user(&self, user: User) -> Result<User, DatabaseError>;
    fn delete_user(&self, id: String) -> Result<(), DatabaseError>;
}

pub struct UserServiceImpl {
    data: Box<dyn UserRepository + Send + Sync>,
}

impl UserServiceImpl {
    pub fn new<T>(repository: T) -> Self
    where
        T: UserRepository + Send + Sync + 'static,
    {
        UserServiceImpl {
            data: Box::new(repository),
        }
    }

    pub fn repository(&self) -> &dyn UserRepository {
        &*self.data
    }
}

impl UserService for UserServiceImpl {
    fn get_user_by_id(&self, id: String) -> Result<User, DatabaseError> {
        self.data.get_user_by_id(&id)
    }

    fn get_user_by_email(&self, email: &str) -> Result<User, DatabaseError> {
        self.data.get_user_by_email(email)
    }

    fn get_all_users(&self) -> Result<Vec<User>, DatabaseError> {
        self.data.get_all_users()
    }

    fn create_user(&self, user: User) -> Result<User, DatabaseError> {
        self.data.create_user(user)
    }

    fn update_user(&self, user: User) -> Result<User, DatabaseError> {
        self.data.update_user(user)
    }

    fn delete_user(&self, id: String) -> Result<(), DatabaseError> {
        self.data.delete_user(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{mock, predicate::*};

    mock! {
        pub Repo {}
        impl UserRepository for Repo {
            fn get_user_by_id(&self, id: &str) -> Result<User, DatabaseError>;
            fn get_user_by_email(&self, email: &str) -> Result<User, DatabaseError>;
            fn get_all_users(&self) -> Result<Vec<User>, DatabaseError>;
            fn create_user(&self, user: User) -> Result<User, DatabaseError>;
            fn update_user(&self, user: User) -> Result<User, DatabaseError>;
            fn delete_user(&self, id: &str) -> Result<(), DatabaseError>;
        }
    }

    #[test]
    fn create_user_delegates_to_repo() {
        let mut repo = MockRepo::new();
        let user = User::new("Alice", "alice@example.com").unwrap();
        repo.expect_create_user().returning(|u| Ok(u));

        let service = UserServiceImpl::new(repo);
        let created = service.create_user(user.clone()).unwrap();
        assert_eq!(created.email.value(), user.email.value());
    }
}
