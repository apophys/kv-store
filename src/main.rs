// SPDX-License-Identifier: MIT

mod backend;
mod commands;
mod config;

use clap::Clap;

use commands::{StorageCommand, StorageCommandResult};
use config::Config;

#[derive(Clap)]
struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
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

    let config = Config {
        verbosity: opts.verbose,
        backend_url: None,
    };

    if let Err(err) = opts.subcmd.execute(&config) {
        println!("{:?}", err)
    }
}
