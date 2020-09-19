use super::*;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::string::String;

#[derive(Debug)]
pub struct RedisConnection {
    conn: TcpStream,
}

impl BackendAdapter for RedisConnection {
    fn new() -> BackendAdapterResult<Self> {
        Ok(RedisConnection {
            conn: TcpStream::connect("127.0.0.1:6379")?,
        })
    }

    fn set(&mut self, key: &str, value: &str) -> BackendAdapterResult<String> {
        println!("Setting value [{}] for key [{}] in redis", &key, &value);
        self.conn
            .write(format!("SET {} {}\r\n", &key, &value).as_bytes())?;
        let mut result = [0; 512];
        self.conn.read(&mut result)?;
        let result_str = str::from_utf8(&result)?;

        Ok(result_str.to_string())
    }

    fn get(&mut self, key: &str) -> BackendAdapterResult<String> {
        self.conn.write(format!("GET {}\r\n", &key).as_bytes())?;
        let mut result = [0; 512];
        self.conn.read(&mut result)?;
        let result_str = str::from_utf8(&result)?;

        Ok(result_str.to_string())
    }

    fn clear(&mut self, key: &str) -> BackendAdapterResult<String> {
        self.conn.write(format!("DEL {}\r\n", &key).as_bytes())?;
        let mut result = [0; 512];
        self.conn.read(&mut result)?;

        let result_str = str::from_utf8(&result)?;

        Ok(result_str.to_string())
    }
}
