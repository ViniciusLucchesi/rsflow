use crate::core::models::user_group_model::UserGroup;
use crate::core::interfaces::database::DatabaseError;

pub trait UserGroupRepository {
    fn get_user_group_by_id(&self, id: &str) -> Result<UserGroup, DatabaseError>;
    fn get_user_group_by_user_id(&self, name: &str) -> Result<Vec<UserGroup>, DatabaseError>;
    fn get_user_group_by_group_id(&self, description: &str) -> Result<Vec<UserGroup>, DatabaseError>;
    fn get_all_user_groups(&self) -> Result<Vec<UserGroup>, DatabaseError>;
    
    fn create_user_group(&self, group: UserGroup) -> Result<UserGroup, DatabaseError>;
    fn update_user_group(&self, group: UserGroup) -> Result<UserGroup, DatabaseError>;
    fn delete_user_group(&self, id: &str) -> Result<(), DatabaseError>;
}
