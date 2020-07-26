use super::BackendAdapter;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::string::String;

#[derive(Debug)]
pub struct RedisConnection {
    conn: TcpStream,
}

impl BackendAdapter for RedisConnection {
    fn new() -> Self {
        RedisConnection {
            conn: TcpStream::connect("127.0.0.1:6379")
                .expect("Couldn't connect to redis on 127.0.0.1:6379"),
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<String, &'static str> {
        println!(
            "Setting value [{}] for key [{}] in redis",
            key.clone(),
            value.clone()
        );
        self.conn
            .write(format!("SET {} {}\r\n", key, value.clone()).as_bytes())
            .expect("Fail to communicate with redis");
        let mut result = [0; 512];
        self.conn
            .read(&mut result)
            .expect("Fail to communicate with redis");
        let result_str = str::from_utf8(&result).expect("Handle me better");

        Ok(result_str.to_string())
    }

    fn get(&mut self, key: String) -> Result<String, &'static str> {
        self.conn
            .write(format!("GET {}\r\n", key).as_bytes())
            .expect("Fail to communicate with redis");
        let mut result = [0; 512];
        self.conn
            .read(&mut result)
            .expect("Fail to communicate with redis");
        let result_str = str::from_utf8(&result).expect("Handle me better");

        Ok(result_str.to_string())
    }

    fn clear(&mut self, key: String) -> Result<String, &'static str> {
        self.conn
            .write(format!("DEL {}\r\n", key).as_bytes())
            .expect("Fail to communicate with redis");
        let mut result = [0; 512];
        self.conn
            .read(&mut result)
            .expect("Fail to communicate with redis");

        let result_str = str::from_utf8(&result).expect("Handle me better");

        Ok(result_str.to_string())
    }
}
