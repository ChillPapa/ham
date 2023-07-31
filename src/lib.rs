use exitcode;
use colored::Colorize;

fn str_to_bit_arr(bits: &String) -> Vec<bool> {
    let mut checked_bits: Vec<bool> = vec![];
    for ch in bits.chars() {
        if ch != '0' && ch != '1' {
            println!("{} The bits provided are invailid", "ERROR".bold().red());
            if !(cfg!(feature = "ci_test")) {
                std::process::exit(exitcode::DATAERR);
            } else {
                std::process::exit(exitcode::OK);
            }
        }
        checked_bits.push(if ch == '0' { false } else { true });
    }
    checked_bits
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

pub fn encode(bits: &String) -> String {
    let checked_bits = str_to_bit_arr(bits);
    let mut good_bits: Vec<bool>;
    if checked_bits.len() != 11 {
        println!("{} You must provide 11 bits only", "ERROR".bold().red());
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
    let mut bit_str: String = String::from("");
    for bit in good_bits {
        bit_str += (bit as u8).to_string().as_str()
    }
    format!("{}", &bit_str)
}

pub fn decode(bits: &String) -> String {
    let bits_as_vec = str_to_bit_arr(bits);
    if bits_as_vec.len() != 16 {
        println!("{} You must provide 16 bits only", "ERROR".bold().red());
        if !(cfg!(feature = "ci_test")) {
            std::process::exit(exitcode::DATAERR);
        } else {
            std::process::exit(exitcode::OK);
        }
    }
    let encodable_bits = vec![
        bits_as_vec[3],
        bits_as_vec[5],
        bits_as_vec[6],
        bits_as_vec[7],
        bits_as_vec[9],
        bits_as_vec[10],
        bits_as_vec[11],
        bits_as_vec[12],
        bits_as_vec[13],
        bits_as_vec[14],
        bits_as_vec[15]
    ];
    let mut bit_str: String = String::from("");
    for bit in encodable_bits {
        bit_str += (bit as u8).to_string().as_str();
    }
    let encoded_bits = encode(&bit_str);
    if encoded_bits != *bits {
        println!("{} Bits aren't correct", "ERROR".bold().red());
        if !(cfg!(feature = "ci_test")) {
            std::process::exit(exitcode::DATAERR);
        } else {
            std::process::exit(exitcode::OK);
        }
    }
    bit_str
}
