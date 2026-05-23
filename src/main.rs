
use std::str::FromStr;

use ::clap::{Arg, Command};

use crate::serde::{back_to_dog, convert_dog};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::{self, Pubkey}};
mod serde;
mod http_reqwest;
mod clap;

#[tokio::main]
async fn main() {
    // Serialize
    // let json = convert_dog();

    // println!("JSON: {}", json);

    // Deserialize
    // let dog = back_to_dog(&json);

    // println!("Dog Struct: {:?}", dog);

    // let result = http_reqwest::send_request();

    // clap::try_clap();

    let getaddress = Command::new("Wallet-Inspector")
    .about("This is a CLI based Solana wallet inspector created in rust !")
    .arg(
        Arg::new("address")
        .short('a')
        .required(true)
        .help("this argument takes your solana wallet address")
        .long("address")
    )
    .get_matches();

 
    let address = getaddress.get_one::<String>("address").unwrap();

    println!("{}",address);

    let client = RpcClient::new_with_commitment(
        String::from("https://api.mainnet-beta.solana.com"),
        CommitmentConfig::confirmed(),
    );

    let address = Pubkey::from_str(address);

    if(address.is_ok()){

        let balance = client.get_balance(&address.unwrap()).await.unwrap();

        println!("Balance: {} SOL", balance as f64 / LAMPORTS_PER_SOL as f64);

    }


}
