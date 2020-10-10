use super::*;
use crate::backend::get_backend_adapter;
use crate::config::Config;
use crate::GetCommand;

impl StorageCommand<()> for GetCommand {
    fn execute(&self, cfg: &Config) -> StorageCommandResult<()> {
        if cfg.verbosity > 0 {
            eprintln!("Getting the value for key [{}]", self.key);
        }

        let mut backend = get_backend_adapter(cfg)?;
        let result = backend.get(&self.key)?;

        match result {
            Some(value) => {
                println!("{}", value);
            }
            None => {
                if cfg.verbosity > 0 {
                    eprintln!("Key [{}] not found.", self.key);
                }
            }
        }
        Ok(())
    }
}
