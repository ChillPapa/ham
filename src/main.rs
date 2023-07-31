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
    let bits: &String = match args {
        SubCommand::Encode(sub) => &sub.bits,
        SubCommand::Decode(sub) => &sub.bits,
    };
    let should_encode: bool = match args {
        SubCommand::Encode(_) => true,
        SubCommand::Decode(_) => false,
    };

    if should_encode {
        println!("{}", hamingifyer::encode(bits))
    } else {
        println!("{}", hamingifyer::decode(bits))
    }
}
