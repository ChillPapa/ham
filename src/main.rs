use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Args)]
struct SubArgs {
    #[clap(short, long)]
    bits: String,
}

#[derive(Subcommand)]
enum SubCommand {
    Encode(SubArgs),
    Decode(SubArgs),
}

fn main() {
    let args: &SubCommand = &Cli::parse().subcommand;
    match args {
        SubCommand::Encode(sub) =>
            println!("{}", hamingifyer::encode(&sub.bits)),
        SubCommand::Decode(sub) =>
            println!("{}", hamingifyer::decode(&sub.bits)),
    };
}
