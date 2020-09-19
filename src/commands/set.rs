use super::*;
use crate::backend::get_backend_adapter;
use crate::config::Config;
use crate::SetCommand;

impl StorageCommand<()> for SetCommand {
    fn execute(&self, cfg: &Config) -> StorageCommandResult<()> {
        if cfg.verbosity > 0 {
            println!("Getting the value for key [{}]", self.key.clone());
        }

        let mut backend = get_backend_adapter(cfg)?;
        let result = backend.set(&self.key, &self.value)?;
        println!("{}", result);

        Ok(())
    }
}
