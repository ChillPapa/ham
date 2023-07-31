use clap::{Args, Parser, Subcommand};
use exitcode;

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

fn even_odd_one(vector: &Vec<bool>) -> bool {
    let mut one_count: u32 = 0;
    for bit in vector {
        if bit == &true {
            one_count += 1
        }
    }
    return if one_count % 2 == 0 { false } else { true };
}

fn encode(bits: &String) {
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
    //todo: delete else
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
            ch
    let mut bit_str: String = String::from("");
    for bit in good_bits {
        bit_str += (bit as u8).to_string().as_str()
    }
    println!("{}", &bit_str);
}
