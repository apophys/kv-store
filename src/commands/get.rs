// SPDX-License-Identifier: MIT

use super::*;
use crate::backend::get_backend_adapter;
use crate::config::Config;
use crate::GetCommand;

impl StorageCommand<()> for GetCommand {
    fn execute(&self, cfg: &Config) -> StorageCommandResult<()> {
        log::info!("Getting the value for key [{}]", self.key);

        let mut backend = get_backend_adapter(cfg)?;

        if let Some(value) = backend.get(&self.key)? {
            println!("{}", value);
        } else {
            log::warn!("Key [{}] not found.", self.key);
        }
        Ok(())
    }
}
