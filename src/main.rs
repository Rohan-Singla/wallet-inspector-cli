use clap::Parser;

use crate::serde::{back_to_dog, convert_dog};

mod serde;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    // get user wallet from terminal
    /// Wallet address to inspect
    #[arg(short, long)]
    wallet: String,
}

fn main() {
    // Serialize
    let json = convert_dog();

    println!("JSON: {}", json);

    // Deserialize
    let dog = back_to_dog(&json);

    println!("Dog Struct: {:?}", dog);
}