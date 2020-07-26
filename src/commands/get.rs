use super::StorageCommand;
use crate::config::Config;
use crate::Get;

impl StorageCommand for Get {
    fn execute(&self, _cfg: &mut Config) -> Result<bool, &'static str> {
        println!("Getting the value for key [{}]", self.key);
        Ok(true)
    }
}
