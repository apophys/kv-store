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
}

#[derive(Clap)]
struct Set {
    key: String,
    value: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.verbose {
        _ => println!("Value of verbose: {}", opts.verbose),
    }

    match opts.subcmd {
        SubCommand::Set(s) => {
            println!("Key => value: {} => {}", s.key, s.value);
        }
    }
}
