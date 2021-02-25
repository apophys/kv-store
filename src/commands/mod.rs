// SPDX-License-Identifier: MIT

pub mod clear;
pub mod get;
pub mod set;

use std::fmt;
use std::error::Error;

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
        Self::BackendError(error)
    }
}

impl fmt::Display for StorageCommandError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BackendError(err) => write!(f, "{}", err)
		}
	}
}

impl Error for StorageCommandError {}