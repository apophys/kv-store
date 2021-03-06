// SPDX-License-Identifier: MIT

mod backend;
mod commands;
mod config;
mod logging;

use clap::Clap;

use crate::config::{Config, ConfigBuilder};
use commands::{StorageCommand, StorageCommandResult};

#[derive(Clap)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,
    #[clap(short, long)]
    backend: Option<String>,
    #[clap(subcommand)]
    subcmd: CLISubCommand,
}

#[derive(Clap)]
enum CLISubCommand {
    Set(SetCommand),
    Get(GetCommand),
    Clear(ClearCommand),
}

#[derive(Clap)]
struct SetCommand {
    key: String,
    value: String,
}

#[derive(Clap)]
struct GetCommand {
    key: String,
}

#[derive(Clap)]
struct ClearCommand {
    key: String,
}

impl CLISubCommand {
    fn execute(&self, cfg: &Config) -> StorageCommandResult<()> {
        match self {
            Self::Set(cmd) => cmd.execute(cfg),
            Self::Get(cmd) => cmd.execute(cfg),
            Self::Clear(cmd) => cmd.execute(cfg),
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    let config = ConfigBuilder::new()
        .verbose(opts.verbose)
        .backend_url(opts.backend)
        .build();

    if logging::configure_logger(&config).is_ok() {
        log::info!("Successfully loaded config and initialized logging.");
    } else {
        eprintln!("Couldn't configure the logger");
    }

    if let Err(err) = opts.subcmd.execute(&config) {
        eprintln!("{}", err);
    }
}
