use crate::ContentId;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Lease Expired, data is no longer accessible")]
    LeaseExpired,

    #[error("Content with id {0} not found")]
    ContentNotFound(ContentId),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Storage has already been initialized")]
    AlreadyInitializedStorage,

    #[error("Storage has not been initialized")]
    StorageNotInit,
}
