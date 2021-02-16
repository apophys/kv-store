// SPDX-License-Identifier: MIT

#[derive(Debug)]
pub struct Config {
    pub logging_level: log::LevelFilter,
    pub backend_url: Option<String>,
}

#[derive(Debug)]
pub struct ConfigBuilder {
    pub logging_level: log::LevelFilter,
    pub backend_url: Option<String>,
}

impl ConfigBuilder {
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

    pub fn build(self) -> Config {
        Config {
            logging_level: self.logging_level,
            backend_url: self.backend_url,
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

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder {
            logging_level: log::LevelFilter::Off,
            backend_url: None,
        }
    }
}
