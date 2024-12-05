use crate::core::models::group_model::GroupId;
use crate::core::models::user_model::UserId;


#[derive(Debug, Clone)]
pub struct UserGroup {
    pub id: UserGroupId,
    pub user_id: UserId,
    pub group_id: GroupId,
}

impl UserGroup {
    pub fn new(user_id: &str, group_id: &str) -> Result<Self, &'static str> {
        let user_id = UserId::from_string(user_id)?;
        let group_id = GroupId::from_string(group_id)?;

        Ok(UserGroup {
            id: UserGroupId::new(),
            user_id,
            group_id,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UserGroupId(uuid::Uuid);

impl UserGroupId {
    pub fn new() -> Self {
        UserGroupId(uuid::Uuid::now_v7())
    }

    pub fn value(&self) -> String {
        self.0.hyphenated().to_string()
    }

    pub fn from_string(value: &str) -> Result<Self, &'static str> {
        match uuid::Uuid::try_parse(value) {
            Ok(uuid) => Ok(UserGroupId(uuid)),
            Err(_) => Err("Invalid UUID string"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_group_id() {
        let id = UserGroupId::new();
        assert_eq!(id.value().len(), 36);
    }

    #[test]
    fn test_user_group() {
        let user_id = UserId::new();
        let group_id = GroupId::new();
        let user_group = UserGroup::new(&user_id.value(), &group_id.value()).unwrap();        
        assert_eq!(user_group.user_id.value(), user_id.value());
        assert_eq!(user_group.group_id.value(), group_id.value());
    }
}