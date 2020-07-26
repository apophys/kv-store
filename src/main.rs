use clap::Clap;

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

trait StorageCommand {
    fn execute(&self) -> Result<bool, &'static str>;
}

impl StorageCommand for Set {
    fn execute(&self) -> Result<bool, &'static str> {
        println!("Setting a value [{}] to key [{}]", self.value, self.key);
        Ok(true)
    }
}

impl StorageCommand for Get {
    fn execute(&self) -> Result<bool, &'static str> {
        println!("Getting the value for key [{}]", self.key);
        Ok(true)
    }
}

impl StorageCommand for Clear {
    fn execute(&self) -> Result<bool, &'static str> {
        println!("Clearing the key [{}]", self.key);
        Ok(true)
    }
}

impl StorageCommand for SubCommand {
    fn execute(&self) -> Result<bool, &'static str> {
        match self {
            SubCommand::Set(cmd) => cmd.execute(),
            SubCommand::Get(cmd) => cmd.execute(),
            SubCommand::Clear(cmd) => cmd.execute(),
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.verbose {
        _ => println!("Value of verbose: {}", opts.verbose),
    }

    let _result = opts.subcmd.execute();
}
