use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use serde_json::json;
use crate::core::models::user_group_model::UserGroup;
// use crate::core::models::user_model::UserId;
// use crate::core::models::group_model::GroupId;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateUserGroupRequest {
    pub user_id: String,
    pub group_id: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"id":"f0000000-0000-0000-0000-000000000000","user_id":"11111111-1111-1111-1111-111111111111","group_id":"22222222-2222-2222-2222-222222222222"}))]
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