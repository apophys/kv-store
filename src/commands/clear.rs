use super::StorageCommand;
use crate::backend::get_backend_adapter;
use crate::config::Config;
use crate::Clear;

impl StorageCommand for Clear {
    fn execute(&self, cfg: &Config) -> Result<bool, &'static str> {
        if cfg.verbosity > 0 {
            println!("Getting the value for key [{}]", self.key.clone());
        }

        let mut backend = get_backend_adapter(cfg);
        let result = backend
            .clear(self.key.clone())
            .expect("Couldn't save the value");
        println!("{}", result);
        Ok(true)
    }
}
