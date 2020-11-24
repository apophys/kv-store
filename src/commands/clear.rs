// SPDX-License-Identifier: MIT

use super::*;
use crate::backend::get_backend_adapter;
use crate::config::Config;
use crate::ClearCommand;

impl StorageCommand<()> for ClearCommand {
    fn execute(&self, cfg: &Config) -> StorageCommandResult<()> {
        log::info!("Deleting the key [{}]", self.key);

        let mut backend = get_backend_adapter(cfg)?;
        backend.clear(&self.key)?;
        Ok(())
    }
}
