// SPDX-License-Identifier: MIT

use log;

#[derive(Debug)]
pub struct Config {
    pub logging_level: log::LevelFilter,
    pub backend_url: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn verbose(mut self, verbosity: bool) -> Self {
        if verbosity {
            self.logging_level = log::LevelFilter::Debug;
        }
        self
    }

    pub fn backend_url(mut self, backend_url: Option<String>) -> Self {
        match backend_url {
            Some(url) => {
                self.backend_url = Some(url);
                self
            }
            None => self,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            logging_level: log::LevelFilter::Off,
            backend_url: None,
        }
    }
}
