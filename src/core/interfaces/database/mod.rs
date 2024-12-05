use std::collections::HashMap;

pub mod user;
pub mod group;
pub mod user_group;


#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError,
    NotFoundError,
    UserNotFound,
    GroupNotFound,
    RegisterAlreadyExists,

    WriteLockError,
    ReadLockError,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionError => write!(f, "Connection error"),
            DatabaseError::NotFoundError => write!(f, "Not found error"),
            DatabaseError::UserNotFound => write!(f, "User not found"),
            DatabaseError::GroupNotFound => write!(f, "Group not found"),
            DatabaseError::RegisterAlreadyExists => write!(f, "Register already exists"),
            DatabaseError::WriteLockError => write!(f, "Write lock error"),
            DatabaseError::ReadLockError => write!(f, "Read lock error"),
        }
    }
}

// pub trait OrmModel {
//     fn table_name() -> &'static str;
// }



// pub trait Repository<T> {
//     fn get_by_query(&self, query: &str) -> Result<Vec<T>, DatabaseError>;
//     fn get_by_id(&self, id: &str) -> Result<T, DatabaseError>;
//     fn get_all(&self) -> Result<Vec<T>, DatabaseError>;
    
//     fn create(&self, entity: &T) -> Result<T, DatabaseError>;
//     fn update(&self, entity: &T) -> Result<T, DatabaseError>;
//     fn delete(&self, id: &str) -> Result<(), DatabaseError>;
// }