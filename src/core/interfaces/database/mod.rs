pub mod user;

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError,
    NotFoundError,
    
    UserNotFound,
    UserAlreadyExists,

    WriteLockError,
    ReadLockError,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionError => write!(f, "Connection error"),
            DatabaseError::NotFoundError => write!(f, "Not found error"),
            DatabaseError::UserNotFound => write!(f, "User not found"),
            DatabaseError::UserAlreadyExists => write!(f, "User already exists"),
            DatabaseError::WriteLockError => write!(f, "Write lock error"),
            DatabaseError::ReadLockError => write!(f, "Read lock error"),
        }
    }
}