use crate::domain::models::user_model::User;
use crate::ports::database::user::{UserReadRepository, UserWriteRepository};
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
    read_repo: std::sync::Arc<dyn UserReadRepository + Send + Sync>,
    write_repo: std::sync::Arc<dyn UserWriteRepository + Send + Sync>,
}

impl UserServiceImpl {
    pub fn new<T>(repository: T) -> Self
    where
        T: UserReadRepository + UserWriteRepository + Send + Sync + 'static,
    {
        let repo = std::sync::Arc::new(repository);
        UserServiceImpl {
            read_repo: repo.clone(),
            write_repo: repo,
        }
    }

    pub fn read_repository(&self) -> &dyn UserReadRepository {
        &*self.read_repo
    }

    pub fn write_repository(&self) -> &dyn UserWriteRepository {
        &*self.write_repo
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_user_by_id(&self, id: String) -> Result<User, DatabaseError> {
        self.read_repo.get_user_by_id(&id).await
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, DatabaseError> {
        self.read_repo.get_user_by_email(email).await
    }

    async fn get_all_users(&self) -> Result<Vec<User>, DatabaseError> {
        self.read_repo.get_all_users().await
    }

    async fn create_user(&self, user: User) -> Result<User, DatabaseError> {
        self.write_repo.create_user(user).await
    }

    async fn update_user(&self, user: User) -> Result<User, DatabaseError> {
        self.write_repo.update_user(user).await
    }

    async fn delete_user(&self, id: String) -> Result<(), DatabaseError> {
        self.write_repo.delete_user(&id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    #[derive(Default)]
    struct DummyRepo;

    #[async_trait]
    impl UserReadRepository for DummyRepo {
        async fn get_user_by_id(&self, _id: &str) -> Result<User, DatabaseError> {
            unimplemented!()
        }

        async fn get_user_by_email(&self, _email: &str) -> Result<User, DatabaseError> {
            unimplemented!()
        }

        async fn get_all_users(&self) -> Result<Vec<User>, DatabaseError> {
            unimplemented!()
        }
    }

    #[async_trait]
    impl UserWriteRepository for DummyRepo {
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
