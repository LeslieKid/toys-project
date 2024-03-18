#[derive(thiserror::Error, Debug)]
pub enum KvsError {
    #[error("Key not found")]
    NotFound,
    #[error("Serde error")]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, KvsError>;
