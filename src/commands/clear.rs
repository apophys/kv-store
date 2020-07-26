use crate::config::Config;

use crate::Clear;
use crate::commands::StorageCommand;

impl StorageCommand for Clear {
    fn execute(&self, _cfg: &mut Config) -> Result<bool, &'static str> {
        println!("Clearing the key [{}]", self.key);
        Ok(true)
    }
}