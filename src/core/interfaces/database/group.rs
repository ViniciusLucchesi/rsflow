use crate::core::models::group_model::Group;
use crate::core::interfaces::database::DatabaseError;

pub trait GroupRepository {
    fn get_group_by_id(&self, id: &str) -> Result<Group, DatabaseError>;
    fn get_group_by_name(&self, name: &str) -> Result<Group, DatabaseError>;    
    fn get_group_by_description(&self, description: &str) -> Result<Group, DatabaseError>;
    fn get_all_groups(&self) -> Result<Vec<Group>, DatabaseError>;
    
    fn create_group(&self, group: Group) -> Result<Group, DatabaseError>;
    fn update_group(&self, group: Group) -> Result<Group, DatabaseError>;
    fn delete_group(&self, id: &str) -> Result<(), DatabaseError>;
}
