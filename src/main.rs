use std::str::FromStr;

use ::clap::{Arg, Command};

use serde_json::to_string;
// use crate::serde::{back_to_dog, convert_dog};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::{self, Pubkey},
    signature::Keypair,
    signer::Signer,
};
mod clap;
mod http_reqwest;
mod serde;
use std::fs;
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

    let cli = Command::new("Wallet-Inspector")
        .about("This is a CLI based Solana wallet inspector created in rust !")
        .subcommand(
            Command::new("inspect")
                .about(
                    "Inspect a wallet address and get its balance, with optional airdrop on devnet",
                )
                .arg(
                    Arg::new("address")
                        .short('a')
                        .required(true)
                        .help("This argument takes your Solana wallet address")
                        .long("address"),
                )
                .arg(
                    Arg::new("airdrop")
                        .long("drop")
                        .help("Amount of SOL to airdrop to the address on devnet")
                        .value_name("AMOUNT"),
                ),
        )
        .subcommand(
            Command::new("create-wallet")
                .about("this command lets you create a wallet")
                .arg(
                    Arg::new("create"), // .short('c')
                ),
        )
        .get_matches();

    match cli.subcommand() {
        Some(("inspect", sub_m)) => {
            let address_str = sub_m
                .get_one::<String>("address")
                .expect("Address is required");

            println!("{}", address_str);

            let client = RpcClient::new_with_commitment(
                "https://api.mainnet-beta.solana.com".to_string(),
                CommitmentConfig::confirmed(),
            );

            let address = Pubkey::from_str(address_str).expect("Invalid address format");

            let balance = client.get_balance(&address).await.unwrap();

            println!("Balance: {} SOL", balance as f64 / LAMPORTS_PER_SOL as f64);

            if let Some(amount_str) = sub_m.get_one::<String>("airdrop") {
                let amount: f64 = amount_str.parse().expect("Invalid airdrop amount");
                let lamports = (amount * LAMPORTS_PER_SOL as f64) as u64;

                let devnet_client = RpcClient::new_with_commitment(
                    "https://api.devnet.solana.com".to_string(),
                    CommitmentConfig::confirmed(),
                );

                let airdrop_sig = devnet_client
                    .request_airdrop(&address, lamports)
                    .await
                    .unwrap();

                let confirmed = devnet_client
                    .confirm_transaction(&airdrop_sig)
                    .await
                    .unwrap();

                if confirmed {
                    println!("Airdrop successful: {}", airdrop_sig);
                } else {
                    println!("Airdrop failed or not confirmed.");
                }
            }
        }

        Some(("create-wallet", _sub_m)) => {
            if std::path::Path::new("wallet.json").exists() {
                println!("Wallet already exists. Loading wallet...");

                let readwallet = fs::read_to_string("wallet.json").unwrap();

                let secret_vec: Vec<u8> = serde_json::from_str(&readwallet).unwrap();

                let secret_array: [u8; 64] = secret_vec.try_into().unwrap();

                let keypair = Keypair::try_from(secret_array.as_slice()).unwrap();

                println!("Wallet loaded!");
                println!("Address: {}", keypair.pubkey());

            } else {
                println!("Creating new wallet...");

                let keypair = Keypair::new();

                let address = keypair.pubkey();

                let secret = keypair.to_bytes().to_vec();

                let secretjson = serde_json::to_string(&secret).unwrap();

                fs::write("wallet.json", secretjson).unwrap();

                println!("New wallet created!");
                println!("Address: {}", address);
            }
        }

        _ => {
            println!("No valid subcommand provided. Use --help for more information.");
        }
    }
}
