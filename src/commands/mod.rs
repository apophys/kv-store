pub mod set;
pub mod get;
pub mod clear;

use crate::config::Config;

pub trait StorageCommand {
    fn execute(&self, cfg: &mut Config) -> Result<bool, &'static str>;
}