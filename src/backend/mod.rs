// SPDX-License-Identifier: MIT


use std::error::Error;
use std::fmt;

use crate::config::Config;

pub mod redis_adapter;

#[derive(Debug)]
pub enum BackendAdapterError {
    NetworkUnavailable(std::io::Error),
    ParseError(std::str::Utf8Error),
    TransportError(Box<dyn std::error::Error>),
}

pub type BackendAdapterResult<T> = Result<T, BackendAdapterError>;

pub trait BackendAdapter {
    fn new(config: &Config) -> BackendAdapterResult<Self>
    where
        Self: Sized;
    fn set(&mut self, key: &str, value: &str) -> BackendAdapterResult<()>;
    fn get(&mut self, key: &str) -> BackendAdapterResult<Option<String>>;
    fn clear(&mut self, key: &str) -> BackendAdapterResult<()>;
}

pub fn get_backend_adapter(options: &Config) -> BackendAdapterResult<Box<dyn BackendAdapter>> {
    Ok(Box::new(redis_adapter::RedisConnection::new(options)?))
}

impl fmt::Display for BackendAdapterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NetworkUnavailable(err) => write!(f, "Network Unavailable {}", err),
            Self::ParseError(err) => write!(f, "Couldn't parse utf-8 string {}", err),
            Self::TransportError(err) => write!(f, "Transport error: {}", err)
        }
    }
}

impl Error for BackendAdapterError {}


impl From<std::str::Utf8Error> for BackendAdapterError {
    fn from(error: std::str::Utf8Error) -> Self {
        Self::ParseError(error)
    }
}

impl From<std::io::Error> for BackendAdapterError {
    fn from(error: std::io::Error) -> Self {
        Self::NetworkUnavailable(error)
    }
}
