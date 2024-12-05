use serde::{Deserialize, Serialize};
use crate::core::models::user_group_model::UserGroup;
// use crate::core::models::user_model::UserId;
// use crate::core::models::group_model::GroupId;

#[derive(Serialize, Deserialize)]
pub struct CreateUserGroupRequest {
    pub user_id: String,
    pub group_id: String,
}

#[derive(Serialize)]
pub struct UserGroupResponse {
    pub id: String,
    pub user_id: String,
    pub group_id: String,
}

impl From<UserGroup> for UserGroupResponse {
    fn from(user_group: UserGroup) -> Self {
        Self {
            id: user_group.id.value().to_string(),
            user_id: user_group.user_id.value().to_string(),
            group_id: user_group.group_id.value().to_string(),
        }
    }
}