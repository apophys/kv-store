use clap::Clap;

mod commands;
mod config;

use commands::StorageCommand;
use config::Config;

#[derive(Clap)]
struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Set(Set),
    Get(Get),
    Clear(Clear),
}

#[derive(Clap)]
struct Set {
    key: String,
    value: String,
}

#[derive(Clap)]
struct Get {
    key: String,
}

#[derive(Clap)]
struct Clear {
    key: String,
}

impl StorageCommand for SubCommand {
    fn execute(&self, cfg: &mut Config) -> Result<bool, &'static str> {
        match self {
            SubCommand::Set(cmd) => cmd.execute(cfg),
            SubCommand::Get(cmd) => cmd.execute(cfg),
            SubCommand::Clear(cmd) => cmd.execute(cfg),
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut config = Config {
        verbosity: opts.verbose,
    };

    let _result = opts.subcmd.execute(&mut config);
}
