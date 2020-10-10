// SPDX-License-Identifier: MIT

use super::*;
use crate::backend::get_backend_adapter;
use crate::config::Config;
use crate::ClearCommand;

impl StorageCommand<()> for ClearCommand {
    fn execute(&self, cfg: &Config) -> StorageCommandResult<()> {
        if cfg.verbosity > 0 {
            eprintln!("Deleting the key [{}]", self.key);
        }

        let mut backend = get_backend_adapter(cfg)?;
        backend.clear(&self.key)?;
        Ok(())
    }
}
