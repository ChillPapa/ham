use clap::{Args, Parser, Subcommand};
use exitcode;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The command
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Args)]
struct SubArgs {
    /// The bits to decode/encode
    #[clap(short, long)]
    bits: String,
}

#[derive(Subcommand)]
enum SubCommand {
    Encode(SubArgs),
    Decode(SubArgs),
}

fn even_odd_one(vector: &Vec<bool>) -> bool {
    let mut one_count: u32 = 0;
    for bit in vector {
        if bit == &true {
            one_count += 1
        }
    }
    return if one_count % 2 == 0 { false } else { true };
}

fn main() {
    let args: &SubCommand = &Cli::parse().subcommand;
    let bits: &String = match args {
        SubCommand::Encode(sub) => &sub.bits,
        SubCommand::Decode(sub) => &sub.bits,
    };
    let encode: bool = match args {
        SubCommand::Encode(_) => true,
        SubCommand::Decode(_) => false,
    };
    let mut checked_bits: Vec<bool> = vec![];
    for ch in bits.chars() {
        if ch != '0' && ch != '1' {
            println!("\x1b[1;31merror:\x1b[0m The bits provided are invailid");
            if !(cfg!(feature = "ci_test")) {
                std::process::exit(exitcode::DATAERR);
            } else {
                std::process::exit(exitcode::OK);
            }
        }
        checked_bits.push(if ch == '0' { false } else { true });
    }
    let mut good_bits: Vec<bool>;
    if encode {
        if checked_bits.len() != 11 {
            println!("\x1b[1;31merror:\x1b[0m You must provide 11 bits only");
            if !(cfg!(feature = "ci_test")) {
                std::process::exit(exitcode::DATAERR);
            } else {
                std::process::exit(exitcode::OK);
            }
        }
        good_bits = vec![
            false,
            false,
            false,
            checked_bits[0],
            false,
            checked_bits[1],
            checked_bits[2],
            checked_bits[3],
            false,
            checked_bits[4],
            checked_bits[5],
            checked_bits[6],
            checked_bits[7],
            checked_bits[8],
            checked_bits[9],
            checked_bits[10],
        ];
        good_bits[1] = even_odd_one(&vec![
            good_bits[3],
            good_bits[5],
            good_bits[7],
            good_bits[9],
            good_bits[11],
            good_bits[13],
            good_bits[15],
        ]);
        good_bits[2] = even_odd_one(&vec![
            good_bits[3],
            good_bits[6],
            good_bits[7],
            good_bits[10],
            good_bits[11],
            good_bits[14],
            good_bits[15],
        ]);
        good_bits[4] = even_odd_one(&vec![
            good_bits[5],
            good_bits[6],
            good_bits[7],
            good_bits[12],
            good_bits[13],
            good_bits[14],
            good_bits[15],
        ]);
        good_bits[8] = even_odd_one(&vec![
            good_bits[9],
            good_bits[10],
            good_bits[11],
            good_bits[12],
            good_bits[13],
            good_bits[14],
            good_bits[15],
        ]);
        good_bits[0] = even_odd_one(&good_bits);
    } else {
        if checked_bits.len() != 16 {
            println!("\x1b[1;31merror:\x1b[0m You must provide 16 bits only");
            if !(cfg!(feature = "ci_test")) {
                std::process::exit(exitcode::DATAERR);
            } else {
                std::process::exit(exitcode::OK);
            }
        }
        let data_bits: Vec<bool> = vec![
            checked_bits[3],
            checked_bits[5],
            checked_bits[6],
            checked_bits[7],
            checked_bits[9],
            checked_bits[10],
            checked_bits[11],
            checked_bits[12],
            checked_bits[13],
            checked_bits[14],
            checked_bits[15],
        ];
        let mut reencoded_bits = vec![
            false,
            false,
            false,
            data_bits[0],
            false,
            data_bits[1],
            data_bits[2],
            data_bits[3],
            false,
            data_bits[4],
            data_bits[5],
            data_bits[6],
            data_bits[7],
            data_bits[8],
            data_bits[9],
            data_bits[10],
        ];
        reencoded_bits[1] = even_odd_one(&vec![
            reencoded_bits[3],
            reencoded_bits[5],
            reencoded_bits[7],
            reencoded_bits[9],
            reencoded_bits[11],
            reencoded_bits[13],
            reencoded_bits[15],
        ]);
        reencoded_bits[2] = even_odd_one(&vec![
            reencoded_bits[3],
            reencoded_bits[6],
            reencoded_bits[7],
            reencoded_bits[10],
            reencoded_bits[11],
            reencoded_bits[14],
            reencoded_bits[15],
        ]);
        reencoded_bits[4] = even_odd_one(&vec![
            reencoded_bits[5],
            reencoded_bits[6],
            reencoded_bits[7],
            reencoded_bits[12],
            reencoded_bits[13],
            reencoded_bits[14],
            reencoded_bits[15],
        ]);
        reencoded_bits[8] = even_odd_one(&vec![
            reencoded_bits[9],
            reencoded_bits[10],
            reencoded_bits[11],
            reencoded_bits[12],
            reencoded_bits[13],
            reencoded_bits[14],
            reencoded_bits[15],
        ]);
        reencoded_bits[0] = even_odd_one(&reencoded_bits);
        if reencoded_bits == checked_bits {
            good_bits = data_bits;
        } else {
            println!("\x1b[1;31merror:\x1b[0m The bits were altered");
            if !(cfg!(feature = "ci_test")) {
                std::process::exit(exitcode::DATAERR);
            } else {
                std::process::exit(exitcode::OK);
            }
        }
    }
    let mut bit_str: String = String::from("");
    for bit in good_bits {
        bit_str += (bit as u32).to_string().as_str()
    }
    println!("{}", &bit_str);
}
