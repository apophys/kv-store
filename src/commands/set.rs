use super::StorageCommand;
use crate::config::Config;
use crate::Set;

impl StorageCommand for Set {
    fn execute(&self, _cfg: &mut Config) -> Result<bool, &'static str> {
        println!("Setting a value [{}] to key [{}]", self.value, self.key);
        Ok(true)
    }
}
