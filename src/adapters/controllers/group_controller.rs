use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::core::models::group_model::Group;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateGroupRequest {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, ToSchema)]
pub struct GroupResponse {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl From<Group> for GroupResponse {
    fn from(group: Group) -> Self {
        Self {
            id: group.id.value().to_string(),
            name: group.name.value().to_string(),
            description: group.description.value().to_string(),
        }
    }
}