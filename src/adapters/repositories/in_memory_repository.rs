use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use crate::core::models::user_model::User;
use crate::core::models::group_model::Group;
use crate::core::models::user_group_model::UserGroup;
use crate::core::interfaces::database::DatabaseError;
use crate::core::interfaces::database::user::UserRepository;
use crate::core::interfaces::database::group::GroupRepository;
use crate::core::interfaces::database::user_group::UserGroupRepository;


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
        Err(DatabaseError::NotFoundError)
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
            return Err(DatabaseError::RegisterAlreadyExists);
        }

        // Insert the new user with an Arc<RwLock<User>>
        data.insert(user.id.value().to_string().clone(), Arc::new(RwLock::new(user.clone())));
        Ok(user)
    }

    fn update_user(&self, user: User) -> Result<User, DatabaseError> {
        // Acquire read lock on the main HashMap to find the specific user
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        if let Some(user_lock) = data.get(&user.id.value()) {
            // Acquire write lock on the specific User entry to modify it
            let mut user_entry = user_lock.write().map_err(|_| DatabaseError::WriteLockError)?;
            *user_entry = user.clone();
            Ok(user)
        } else {
            Err(DatabaseError::NotFoundError)
        }
    }

    fn delete_user(&self, id: &str) -> Result<(), DatabaseError> {
        // Acquire write lock on the main HashMap to remove the user
        let mut data = self.data.write().map_err(|_| DatabaseError::WriteLockError)?;

        if data.remove(id).is_some() {
            Ok(())
        } else {
            Err(DatabaseError::NotFoundError)
        }
    }
}



#[derive(Debug, Clone)]
pub struct InMemoryGroupRepository {
    data: Arc<RwLock<HashMap<String, Arc<RwLock<Group>>>>>,
}

impl InMemoryGroupRepository {
    pub fn new() -> Self {
        InMemoryGroupRepository {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl GroupRepository for InMemoryGroupRepository {
    fn get_group_by_id(&self, id: &str) -> Result<Group, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;
        
        // Acquire read lock on the specific Group (if it exists)
        if let Some(group_lock) = data.get(id) {
            let group = group_lock.read().map_err(|_| DatabaseError::ReadLockError)?;
            Ok(group.clone())
        } else {
            Err(DatabaseError::GroupNotFound)
        }
    }

    fn get_group_by_name(&self, name: &str) -> Result<Group, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        // Iterate over all entries and check by name
        for group_lock in data.values() {
            let group = group_lock.read().map_err(|_| DatabaseError::ReadLockError)?;
            if group.name.value() == name {
                return Ok(group.clone());
            }
        }
        Err(DatabaseError::NotFoundError)
    }

    fn get_group_by_description(&self, description: &str) -> Result<Group, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        // Iterate over all entries and check by description
        for group_lock in data.values() {
            let group = group_lock.read().map_err(|_| DatabaseError::ReadLockError)?;
            if group.description.value() == description {
                return Ok(group.clone());
            }
        }
        Err(DatabaseError::NotFoundError)
    }

    fn get_all_groups(&self) -> Result<Vec<Group>, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        // Iterate over all entries and clone them
        Ok(data.values().map(|group_lock| {
            group_lock.read().map(|group| group.clone()).unwrap()
        }).collect())
    }
    
    fn create_group(&self, group: Group) -> Result<Group, DatabaseError> {
        // Acquire write lock on the main HashMap to insert a new group
        let mut data = self.data.write().map_err(|_| DatabaseError::WriteLockError)?;

        if data.contains_key(&group.id.value().to_string()) {
            return Err(DatabaseError::RegisterAlreadyExists);
        }

        // Insert the new group with an Arc<RwLock<Group>>
        data.insert(group.id.value().to_string().clone(), Arc::new(RwLock::new(group.clone())));
        Ok(group)
    }

    fn update_group(&self, group: Group) -> Result<Group, DatabaseError> {
        // Acquire read lock on the main HashMap to find the specific group
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        if let Some(group_lock) = data.get(&group.id.value()) {
            // Acquire write lock on the specific Group entry to modify it
            let mut group_entry = group_lock.write().map_err(|_| DatabaseError::WriteLockError)?;
            *group_entry = group.clone();
            Ok(group)
        } else {
            Err(DatabaseError::NotFoundError)
        }
    }

    fn delete_group(&self, id: &str) -> Result<(), DatabaseError> {
        // Acquire write lock on the main HashMap to remove the group
        let mut data = self.data.write().map_err(|_| DatabaseError::WriteLockError)?;

        if data.remove(id).is_some() {
            Ok(())
        } else {
            Err(DatabaseError::NotFoundError)
        }
    }
}



#[derive(Debug, Clone)]
pub struct InMemoryUserGroupRepository {
    data: Arc<RwLock<HashMap<String, Arc<RwLock<UserGroup>>>>>,
}

impl InMemoryUserGroupRepository {
    pub fn new() -> Self {
        InMemoryUserGroupRepository {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl UserGroupRepository for InMemoryUserGroupRepository {
    fn get_user_group_by_id(&self, id: &str) -> Result<UserGroup, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;
        
        // Acquire read lock on the specific UserGroup (if it exists)
        if let Some(user_group_lock) = data.get(id) {
            let user_group = user_group_lock.read().map_err(|_| DatabaseError::ReadLockError)?;
            Ok(user_group.clone())
        } else {
            Err(DatabaseError::NotFoundError)
        }
    }

    fn get_user_group_by_group_id(&self, description: &str) -> Result<Vec<UserGroup>, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        // Iterate over all entries and check by group_id
        let mut user_groups = Vec::new();
        for user_group_lock in data.values() {
            let user_group = user_group_lock.read().map_err(|_| DatabaseError::ReadLockError)?;
            if user_group.group_id.value() == description {
                user_groups.push(user_group.clone());
            }
        }
        Ok(user_groups)
    }

    fn get_user_group_by_user_id(&self, name: &str) -> Result<Vec<UserGroup>, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        // Iterate over all entries and check by user_id
        let mut user_groups = Vec::new();
        for user_group_lock in data.values() {
            let user_group = user_group_lock.read().map_err(|_| DatabaseError::ReadLockError)?;
            if user_group.user_id.value() == name {
                user_groups.push(user_group.clone());
            }
        }
        Ok(user_groups)
    }

    fn get_all_user_groups(&self) -> Result<Vec<UserGroup>, DatabaseError> {
        // Acquire read lock on the main HashMap
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        // Iterate over all entries and clone them
        Ok(data.values().map(|user_group_lock| {
            user_group_lock.read().map(|user_group| user_group.clone()).unwrap()
        }).collect())
    }

    fn create_user_group(&self, user_group: UserGroup) -> Result<UserGroup, DatabaseError> {
        // Acquire write lock on the main HashMap to insert a new user_group
        let mut data = self.data.write().map_err(|_| DatabaseError::WriteLockError)?;

        if data.contains_key(&user_group.id.value().to_string()) {
            return Err(DatabaseError::RegisterAlreadyExists);
        }

        

        // Insert the new user_group with an Arc<RwLock<UserGroup>>
        data.insert(user_group.id.value().to_string().clone(), Arc::new(RwLock::new(user_group.clone())));
        Ok(user_group)
    }

    fn update_user_group(&self, user_group: UserGroup) -> Result<UserGroup, DatabaseError> {
        // Acquire read lock on the main HashMap to find the specific user_group
        let data = self.data.read().map_err(|_| DatabaseError::ReadLockError)?;

        if let Some(user_group_lock) = data.get(&user_group.id.value()) {
            // Acquire write lock on the specific UserGroup entry to modify it
            let mut user_group_entry = user_group_lock.write().map_err(|_| DatabaseError::WriteLockError)?;
            *user_group_entry = user_group.clone();
            Ok(user_group)
        } else {
            Err(DatabaseError::NotFoundError)
        }
    }

    fn delete_user_group(&self, id: &str) -> Result<(), DatabaseError> {
        // Acquire write lock on the main HashMap to remove the user_group
        let mut data = self.data.write().map_err(|_| DatabaseError::WriteLockError)?;

        if data.remove(id).is_some() {
            Ok(())
        } else {
            Err(DatabaseError::NotFoundError)
        }
    }    
}