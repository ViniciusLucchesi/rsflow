use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::domain::models::user_model::User;
use crate::ports::database::DatabaseError;
use crate::ports::database::user::UserRepository;

#[derive(Debug, Clone)]
pub struct InMemoryUserRepository {
    data: Arc<RwLock<HashMap<String, Arc<RwLock<User>>>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn get_user_by_id(&self, id: &str) -> Result<User, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;
        
        // Acquire read lock on the specific User (if it exists)
        if let Some(user_lock) = data.get(id) {
            let user = user_lock.read().map_err(|_| DatabaseError::ReadLockError)?;
            Ok(user.clone())
        } else {
            Err(DatabaseError::UserNotFound)
        }
    }

    fn get_user_by_email(&self, email: &str) -> Result<User, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        // Iterate over all entries and check by email
        for user_lock in data.values() {
            let user = user_lock.read().map_err(|_| DatabaseError::ReadLockError)?;
            if user.email.value() == email {
                return Ok(user.clone());
            }
        }
        Err(DatabaseError::UserNotFound)
    }

    fn get_all_users(&self) -> Result<Vec<User>, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        // Iterate over all entries and clone them
        Ok(data.values().map(|user_lock| {
            user_lock.read().map(|user| user.clone()).unwrap()
        }).collect())
    }
    
    fn create_user(&self, user: User) -> Result<User, DatabaseError> {
        // Acquire write lock on the main HashMap to insert a new user
        let mut data = self.data.write().map_err(|_| DatabaseError::WriteLockError)?;
        
        if data.contains_key(&user.id.value().to_string()) {
            return Err(DatabaseError::UserAlreadyExists);
        }

        // Insert the new user with an Arc<RwLock<User>>
        data.insert(user.id.value().to_string().clone(), Arc::new(RwLock::new(user.clone())));
        Ok(user)
    }

    fn update_user(&self, user: User) -> Result<User, DatabaseError> {
        // Acquire read lock on the main HashMap to find the specific user
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        if let Some(user_lock) = data.get(user.id.value()) {
            // Acquire write lock on the specific User entry to modify it
            let mut user_entry = user_lock.write().map_err(|_| DatabaseError::WriteLockError)?;
            *user_entry = user.clone();
            Ok(user)
        } else {
            Err(DatabaseError::UserNotFound)
        }
    }

    fn delete_user(&self, id: &str) -> Result<(), DatabaseError> {
        // Acquire write lock on the main HashMap to remove the user
        let mut data = self.data.write().map_err(|_| DatabaseError::WriteLockError)?;

        if data.remove(id).is_some() {
            Ok(())
        } else {
            Err(DatabaseError::UserNotFound)
        }
    }
}
