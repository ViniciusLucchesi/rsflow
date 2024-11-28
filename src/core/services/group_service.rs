use crate::core::models::group_model::Group;
use crate::core::interfaces::database::DatabaseError;
use crate::core::interfaces::database::group::GroupRepository;


pub struct GroupService {
    data: Box<dyn GroupRepository + Send + Sync>,
}

impl GroupService {
    pub fn new<T>(repository: T) -> GroupService
    where
        T: GroupRepository + Send + Sync + 'static + Clone,
    {
        GroupService {
            data: Box::new(repository)
        }
    }

    pub fn get_group_by_id(&self, id: &str) -> Result<Group, DatabaseError> {
        self.data.get_group_by_id(id)
    }

    pub fn get_group_by_name(&self, name: &str) -> Result<Group, DatabaseError> {
        self.data.get_group_by_name(name)
    }

    pub fn get_group_by_description(&self, description: &str) -> Result<Group, DatabaseError> {
        self.data.get_group_by_description(description)
    }

    pub fn get_all_groups(&self) -> Result<Vec<Group>, DatabaseError> {
        self.data.get_all_groups()
    }

    pub fn create_group(&self, group: Group) -> Result<Group, DatabaseError> {
        self.data.create_group(group)
    }

    pub fn update_group(&self, group: Group) -> Result<Group, DatabaseError> {
        self.data.update_group(group)
    }

    pub fn delete_group(&self, id: &str) -> Result<(), DatabaseError> {
        self.data.delete_group(id)
    }
}