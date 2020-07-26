pub mod clear;
pub mod get;
pub mod set;

use crate::config::Config;

pub trait StorageCommand {
    fn execute(&self, cfg: &mut Config) -> Result<bool, &'static str>;
}
