use crate::config::Config;

pub mod redis;

pub trait BackendAdapter {
    fn new() -> Self
    where
        Self: Sized;
    fn set(&mut self, key: &str, value: &str) -> Result<String, &'static str>;
    fn get(&mut self, key: &str) -> Result<String, &'static str>;
    fn clear(&mut self, key: &str) -> Result<String, &'static str>;
}

pub fn get_backend_adapter(_options: &Config) -> Box<dyn BackendAdapter> {
    Box::new(redis::RedisConnection::new())
}
