use super::StorageCommand;
use crate::config::Config;
use crate::Clear;

impl StorageCommand for Clear {
    fn execute(&self, _cfg: &Config) -> Result<bool, &'static str> {
        println!("Clearing the key [{}]", self.key);
        Ok(true)
    }
}
