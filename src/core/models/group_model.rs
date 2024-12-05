#[derive(Debug, Clone)]
pub struct Group {
    pub id: GroupId,
    pub name: GroupName,
    pub description: GroupDescription,
}

impl Group {
    pub fn new(name: &str, description: &str) -> Result<Self, &'static str> {
        let id = GroupId::new();
        let name = GroupName::new(name)?;
        let description = GroupDescription::new(description)?;

        Ok(Group { id, name, description })
    }
}

#[derive(Debug, Clone)]
pub struct GroupId(uuid::Uuid);

impl GroupId {
    pub fn new() -> Self {        
        GroupId(uuid::Uuid::now_v7())
    }

    pub fn from_string(value: &str) -> Result<Self, &'static str> {
        match uuid::Uuid::try_parse(value) {
            Ok(uuid) => Ok(GroupId(uuid)),
            Err(_) => Err("Invalid UUID string"),
        }
    } 

    pub fn value(&self) -> String {
        self.0.hyphenated().to_string()
    }
}

#[derive(Debug, Clone)]
pub struct GroupName(String);

impl GroupName {
    pub fn new(name: &str) -> Result<Self, &'static str> {
        if name.trim().is_empty() {
            Err("Name cannot be empty")
        } else {
            Ok(GroupName(name.to_string()))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct GroupDescription(String);

impl GroupDescription {
    pub fn new(description: &str) -> Result<Self, &'static str> {
        if description.trim().is_empty() {
            Err("Description cannot be empty")
        } else {
            Ok(GroupDescription(description.to_string()))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_id() {
        let id = GroupId::new();
        assert_eq!(id.value().len(), 36);
    }

    #[test]
    fn test_group_name() {
        let name = GroupName::new("Group 1").unwrap();
        assert_eq!(name.value(), "Group 1");
    }

    #[test]
    fn test_group_name_empty() {
        let name = GroupName::new("");
        assert!(name.is_err());
    }    

    #[test]
    fn test_group_description() {
        let description = GroupDescription::new("Description 1").unwrap();
        assert_eq!(description.value(), "Description 1");
    }

    #[test]
    fn test_group_description_empty() {
        let description = GroupDescription::new("");
        assert!(description.is_err());
    }

    #[test]
    fn test_group() {
        let group = Group::new("Group 1", "Description 1").unwrap();
        assert_eq!(group.name.value(), "Group 1");
        assert_eq!(group.description.value(), "Description 1");
    }
}