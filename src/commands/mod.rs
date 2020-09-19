pub mod clear;
pub mod get;
pub mod set;

use crate::backend::BackendAdapterError;
use crate::config::Config;

#[derive(Debug)]
pub enum StorageCommandError {
    BackendError(BackendAdapterError),
}

pub type StorageCommandResult<T> = Result<T, StorageCommandError>;

pub trait StorageCommand<T> {
    fn execute(&self, cfg: &Config) -> StorageCommandResult<T>;
}

impl From<BackendAdapterError> for StorageCommandError {
    fn from(error: BackendAdapterError) -> Self {
        StorageCommandError::BackendError(error)
    }
}
