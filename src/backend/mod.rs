use crate::config::Config;

pub mod redis;

#[derive(Debug)]
pub enum BackendAdapterError {
    NetworkUnavailable(std::io::Error),
    ParseError(std::str::Utf8Error),
}

pub type BackendAdapterResult<T> = Result<T, BackendAdapterError>;

pub trait BackendAdapter {
    fn new() -> BackendAdapterResult<Self>
    where
        Self: Sized;
    fn set(&mut self, key: &str, value: &str) -> BackendAdapterResult<String>;
    fn get(&mut self, key: &str) -> BackendAdapterResult<String>;
    fn clear(&mut self, key: &str) -> BackendAdapterResult<String>;
}

pub fn get_backend_adapter(_options: &Config) -> BackendAdapterResult<Box<dyn BackendAdapter>> {
    Ok(Box::new(redis::RedisConnection::new()?))
}

impl From<std::str::Utf8Error> for BackendAdapterError {
    fn from(error: std::str::Utf8Error) -> Self {
        BackendAdapterError::ParseError(error)
    }
}

impl From<std::io::Error> for BackendAdapterError {
    fn from(error: std::io::Error) -> Self {
        BackendAdapterError::NetworkUnavailable(error)
    }
}
