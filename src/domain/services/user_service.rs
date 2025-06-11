use crate::domain::models::user_model::User;
use crate::ports::database::user::UserRepository;
use crate::ports::database::DatabaseError;
use async_trait::async_trait;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user_by_id(&self, id: String) -> Result<User, DatabaseError>;
    async fn get_user_by_email(&self, email: &str) -> Result<User, DatabaseError>;
    async fn get_all_users(&self) -> Result<Vec<User>, DatabaseError>;
    async fn create_user(&self, user: User) -> Result<User, DatabaseError>;
    async fn update_user(&self, user: User) -> Result<User, DatabaseError>;
    async fn delete_user(&self, id: String) -> Result<(), DatabaseError>;
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

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_user_by_id(&self, id: String) -> Result<User, DatabaseError> {
        self.data.get_user_by_id(&id).await
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, DatabaseError> {
        self.data.get_user_by_email(email).await
    }

    async fn get_all_users(&self) -> Result<Vec<User>, DatabaseError> {
        self.data.get_all_users().await
    }

    async fn create_user(&self, user: User) -> Result<User, DatabaseError> {
        self.data.create_user(user).await
    }

    async fn update_user(&self, user: User) -> Result<User, DatabaseError> {
        self.data.update_user(user).await
    }

    async fn delete_user(&self, id: String) -> Result<(), DatabaseError> {
        self.data.delete_user(&id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    #[derive(Default)]
    struct DummyRepo;

    #[async_trait]
    impl UserRepository for DummyRepo {
        async fn get_user_by_id(&self, _id: &str) -> Result<User, DatabaseError> {
            unimplemented!()
        }

        async fn get_user_by_email(&self, _email: &str) -> Result<User, DatabaseError> {
            unimplemented!()
        }

        async fn get_all_users(&self) -> Result<Vec<User>, DatabaseError> {
            unimplemented!()
        }

        async fn create_user(&self, user: User) -> Result<User, DatabaseError> {
            Ok(user)
        }

        async fn update_user(&self, _user: User) -> Result<User, DatabaseError> {
            unimplemented!()
        }

        async fn delete_user(&self, _id: &str) -> Result<(), DatabaseError> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn create_user_delegates_to_repo() {
        let repo = DummyRepo::default();
        let user = User::new("Alice", "alice@example.com").unwrap();

        let service = UserServiceImpl::new(repo);
        let created = service.create_user(user.clone()).await.unwrap();
        assert_eq!(created.email.value(), user.email.value());
    }
}
