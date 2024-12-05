#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub email: UserEmail,
}

impl User {
    pub fn new(name: &str, email: &str) -> Result<Self, &'static str> {
        let id = UserId::new();
        let name = UserName::new(name)?;
        let email = UserEmail::new(email)?;
        
        Ok(User{ id, name, email })
    }
}


#[derive(Debug, Clone)]
pub struct UserId (uuid::Uuid);

impl UserId {
    pub fn new() -> Self {
        UserId(uuid::Uuid::now_v7())
    }

    pub fn from_string(value: &str) -> Result<Self, &'static str> {
        match uuid::Uuid::try_parse(value) {
            Ok(uuid) => Ok(UserId(uuid)),
            Err(_) => Err("Invalid UUID string"),
        }
    } 

    pub fn value(&self) -> String {
        self.0.hyphenated().to_string()
    }
}

#[derive(Debug, Clone)]
pub struct UserName (String);

impl UserName {
    pub fn new(name: &str) -> Result<Self, &'static str> {
        if name.trim().is_empty() {
            Err("Name cannot be empty")
        } else {
            Ok(UserName(name.to_string()))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct UserEmail (String);

impl UserEmail {
    fn new(email: &str) -> Result<Self, &'static str> {
        if email.trim().is_empty() {
            Err("Email cannot be empty")
        } else if email.find('@').is_none() {
            Err("Email must contain @")
        } else {
            Ok(UserEmail(email.to_string()))
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
    fn test_user_id() {
        let user_id = UserId::new();
        assert_eq!(user_id.value().len(), 36);
    }

    #[test]
    fn test_user_id_from_string_valid() {
        let uuid_string = "550e8400-e29b-41d4-a716-446655440000";
        let user_id = UserId::from_string(uuid_string);
        assert!(user_id.is_ok());
        assert_eq!(user_id.unwrap().value(), uuid_string);
    }

    #[test]
    fn test_user_name() {
        let user_name = UserName::new("John Doe");
        assert_eq!(user_name.is_ok(), true);
    }

    #[test]
    fn test_user_name_empty() {
        let user_name = UserName::new("");
        assert_eq!(user_name.is_err(), true);
    }

    #[test]
    fn test_user_email() {
        let user_email = UserEmail::new("john.doe@gmail.com");
        assert_eq!(user_email.is_ok(), true);
    }

    #[test]
    fn test_user_email_empty() {
        let user_email = UserEmail::new("");
        assert_eq!(user_email.is_err(), true);
    }
}