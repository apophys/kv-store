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

    fn set(&mut self, key: &str, value: &str) -> String {
        println!("Setting value [{}] for key [{}] in redis", &key, &value);
        self.conn
            .write(format!("SET {} {}\r\n", &key, &value).as_bytes())
            .expect("Fail to communicate with redis");
        let mut result = [0; 512];
        self.conn
            .read(&mut result)
            .expect("Fail to communicate with redis");
        let result_str = str::from_utf8(&result).expect("Handle me better");

        result_str.to_string()
    }

    fn get(&mut self, key: &str) -> String {
        self.conn
            .write(format!("GET {}\r\n", &key).as_bytes())
            .expect("Fail to communicate with redis");
        let mut result = [0; 512];
        self.conn
            .read(&mut result)
            .expect("Fail to communicate with redis");
        let result_str = str::from_utf8(&result).expect("Handle me better");

        result_str.to_string()
    }

    fn clear(&mut self, key: &str) -> String {
        self.conn
            .write(format!("DEL {}\r\n", &key).as_bytes())
            .expect("Fail to communicate with redis");
        let mut result = [0; 512];
        self.conn
            .read(&mut result)
            .expect("Fail to communicate with redis");

        let result_str = str::from_utf8(&result).expect("Handle me better");

        result_str.to_string()
    }
}
