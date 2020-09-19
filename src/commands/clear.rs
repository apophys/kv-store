use super::*;
use crate::backend::get_backend_adapter;
use crate::config::Config;
use crate::ClearCommand;

impl StorageCommand<()> for ClearCommand {
    fn execute(&self, cfg: &Config) -> StorageCommandResult<()> {
        if cfg.verbosity > 0 {
            println!("Getting the value for key [{}]", &self.key);
        }

        let mut backend = get_backend_adapter(cfg)?;
        let result = backend.clear(&self.key)?;
        println!("{}", result);
        Ok(())
    }
}
