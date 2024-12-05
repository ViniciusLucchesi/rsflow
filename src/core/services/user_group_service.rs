use std::sync::Arc;
use crate::core::models::user_group_model::UserGroup;
use crate::core::interfaces::database::DatabaseError;
use crate::core::interfaces::database::user_group::UserGroupRepository;

use crate::core::services::user_service::UserService;
use crate::core::services::group_service::GroupService;


pub struct UserGroupService {
    data: Box<dyn UserGroupRepository + Send + Sync>,
    user_service: Arc<UserService>,
    group_service: Arc<GroupService>,
}

impl UserGroupService {
    pub fn new<T>(repository: T, user_service: Arc<UserService>, group_service: Arc<GroupService>) -> UserGroupService
    where
        T: UserGroupRepository + Send + Sync + 'static,
    {
        UserGroupService {
            data: Box::new(repository),
            user_service,
            group_service,
        }
    }

    pub fn get_user_group_by_id(&self, id: &str) -> Result<UserGroup, DatabaseError> {
        self.data.get_user_group_by_id(id)
    }

    pub fn get_user_group_by_user_id(&self, name: &str) -> Result<Vec<UserGroup>, DatabaseError> {
        self.data.get_user_group_by_user_id(name)
    }

    pub fn get_user_group_by_group_id(&self, description: &str) -> Result<Vec<UserGroup>, DatabaseError> {
        self.data.get_user_group_by_group_id(description)
    }

    pub fn get_all_user_groups(&self) -> Result<Vec<UserGroup>, DatabaseError> {
        self.data.get_all_user_groups()
    }

    pub fn create_user_group(&self, user_group: UserGroup) -> Result<UserGroup, DatabaseError> {
        let _ = self.user_service.get_user_by_id(&user_group.user_id.value())?;
        let _ = self.group_service.get_group_by_id(&user_group.group_id.value())?;

        self.data.create_user_group(user_group)
    }

    pub fn update_user_group(&self, group: UserGroup) -> Result<UserGroup, DatabaseError> {
        self.data.update_user_group(group)
    }

    pub fn delete_user_group(&self, id: &str) -> Result<(), DatabaseError> {
        self.data.delete_user_group(id)
    }
}