// SPDX-License-Identifier: MIT

use super::*;
use crate::backend::get_backend_adapter;
use crate::config::Config;
use crate::SetCommand;

impl StorageCommand<()> for SetCommand {
    fn execute(&self, cfg: &Config) -> StorageCommandResult<()> {
        if cfg.verbosity > 0 {
            eprintln!("Setting the value [{}] for key [{}]", self.value, self.key);
        }

        let mut backend = get_backend_adapter(cfg)?;
        backend.set(&self.key, &self.value)?;

        Ok(())
    }
}
