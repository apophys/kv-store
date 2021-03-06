// SPDX-License-Identifier: MIT

use super::*;

use std::str;
use std::string::String;

use crate::config::Config;
use redis::Commands;

impl From<redis::RedisError> for BackendAdapterError {
    fn from(error: redis::RedisError) -> Self {
        Self::TransportError(Box::new(error))
    }
}

#[derive(Debug)]
pub struct RedisConnection {
    client: redis::Client,
}

impl BackendAdapter for RedisConnection {
    fn new(config: &Config) -> BackendAdapterResult<Self> {
        Ok(RedisConnection {
            client: match config.backend_url {
                Some(ref address) => {
                    redis::Client::open(&address[..])?
                },
                None => {
                    redis::Client::open("redis://127.0.0.1:6379")?
                }
            }
        })
    }

    fn set(&mut self, key: &str, value: &str) -> BackendAdapterResult<()> {
        let mut conn = self.client.get_connection()?;
        conn.set(key, value)?;

        Ok(())
    }

    fn get(&mut self, key: &str) -> BackendAdapterResult<Option<String>> {
        let mut conn = self.client.get_connection()?;

        Ok(conn.get(key)?)
    }

    fn clear(&mut self, key: &str) -> BackendAdapterResult<()> {
        let mut conn = self.client.get_connection()?;

        conn.del(key)?;
        Ok(())
    }
}
