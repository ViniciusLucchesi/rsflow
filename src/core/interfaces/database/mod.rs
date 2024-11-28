pub mod user;
pub mod group;

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError,
    NotFoundError,
    RegisterAlreadyExists,

    WriteLockError,
    ReadLockError,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionError => write!(f, "Connection error"),
            DatabaseError::NotFoundError => write!(f, "Not found error"),
            DatabaseError::RegisterAlreadyExists => write!(f, "Register already exists"),
            DatabaseError::WriteLockError => write!(f, "Write lock error"),
            DatabaseError::ReadLockError => write!(f, "Read lock error"),
        }
    }
}