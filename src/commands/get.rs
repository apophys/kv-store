use super::*;
use crate::backend::get_backend_adapter;
use crate::config::Config;
use crate::GetCommand;

impl StorageCommand<()> for GetCommand {
    fn execute(&self, cfg: &Config) -> StorageCommandResult<()> {
        if cfg.verbosity > 0 {
            println!("Getting the value for key [{}]", self.key.clone());
        }

        let mut backend = get_backend_adapter(cfg)?;
        let result = backend.get(&self.key)?;
        println!("{}", result);
        Ok(())
    }
}
