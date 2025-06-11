use crate::domain::models::user_model::User;
use crate::ports::database::user::{UserReadRepository, UserWriteRepository};
use crate::ports::database::DatabaseError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct InMemoryUserRepository {
    data: RwLock<HashMap<String, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository {
            data: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl UserReadRepository for InMemoryUserRepository {
    async fn get_user_by_id(&self, id: &str) -> Result<User, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        // Lookup the user and clone it
        if let Some(user) = data.get(id) {
            Ok(user.clone())
        } else {
            Err(DatabaseError::UserNotFound)
        }
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        // Iterate over all entries and check by email
        for user in data.values() {
            if user.email.value() == email {
                return Ok(user.clone());
            }
        }
        Err(DatabaseError::UserNotFound)
    }

    async fn get_all_users(&self) -> Result<Vec<User>, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        // Iterate over all entries and clone them
        Ok(data.values().cloned().collect())
    }
}

#[async_trait]
impl UserWriteRepository for InMemoryUserRepository {
    async fn create_user(&self, user: User) -> Result<User, DatabaseError> {
        // Acquire write lock on the main HashMap to insert a new user
        let mut data = self
            .data
            .write()
            .map_err(|_| DatabaseError::WriteLockError)?;

        if data.contains_key(&user.id.value().to_string()) {
            return Err(DatabaseError::UserAlreadyExists);
        }

        // Insert the new user
        data.insert(user.id.value().to_string(), user.clone());
        Ok(user)
    }

    async fn update_user(&self, user: User) -> Result<User, DatabaseError> {
        // Acquire write lock to update the user
        let mut data = self
            .data
            .write()
            .map_err(|_| DatabaseError::WriteLockError)?;

        if data.contains_key(user.id.value()) {
            data.insert(user.id.value().to_string(), user.clone());
            Ok(user)
        } else {
            Err(DatabaseError::UserNotFound)
        }
    }

    async fn delete_user(&self, id: &str) -> Result<(), DatabaseError> {
        // Acquire write lock on the main HashMap to remove the user
        let mut data = self
            .data
            .write()
            .map_err(|_| DatabaseError::WriteLockError)?;

        if data.remove(id).is_some() {
            Ok(())
        } else {
            Err(DatabaseError::UserNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::user_model::UserName;

    #[tokio::test]
    async fn crud_flow() {
        let repo = InMemoryUserRepository::new();
        let user = User::new("Bob", "bob@example.com").unwrap();

        let created = repo.create_user(user.clone()).await.unwrap();
        assert_eq!(created.name.value(), "Bob");

        let fetched = repo.get_user_by_id(user.id.value()).await.unwrap();
        assert_eq!(fetched.email.value(), "bob@example.com");

        let updated_user = User {
            name: UserName::new("Bobby").unwrap(),
            ..user.clone()
        };
        repo.update_user(updated_user.clone()).await.unwrap();

        let after_update = repo.get_user_by_id(user.id.value()).await.unwrap();
        assert_eq!(after_update.name.value(), "Bobby");

        repo.delete_user(user.id.value()).await.unwrap();
        assert!(repo.get_user_by_id(user.id.value()).await.is_err());
    }
}
