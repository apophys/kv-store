// SPDX-License-Identifier: MIT

use simplelog;

use crate::config;

pub type ConfigResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn configure_logger(config: &config::Config) -> ConfigResult<()> {
    simplelog::TermLogger::init(
        config.logging_level,
        Default::default(),
        simplelog::TerminalMode::Stderr,
    )?;

    Ok(())
}