
use ::clap::{Arg, Command};

use solana_sdk::{
    signer::Signer,
};
mod clap;
mod http_reqwest;
mod serde;

mod mylib;
mod utils;

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
        .subcommand(
            Command::new("send_sol")
                .about("this command is used to transfer SOl !")
                .arg(
                    Arg::new("reciever_address")
                        .alias("reciever")
                        .required(true)
                        .help("This argument takes reciever address"),
                )
                .arg(
                    Arg::new("amount")
                        .help("this argumemt takes the amount of SOL you want to send !")
                        .required(true),
                ),
        )
        .get_matches();

    match cli.subcommand() {
        Some(("inspect", sub_m)) => {
            let address_str = sub_m
                .get_one::<String>("address")
                .expect("Address is required");

            let airdrop = sub_m.get_one::<String>("airdrop");

            // If airdrop is Some, perform the airdrop first
            if let Some(airdrop_amount) = airdrop {
                // Call airdrop_sol, which returns a future, so .await it
                let result = utils::airdrop_sol(address_str, Some(airdrop_amount)).await;

                match result {
                    Some(signature) => println!("Airdrop signature: {}", signature),
                    None => println!("Airdrop failed or not confirmed."),
                }
            }

            // Check and print balance regardless of airdrop
            let balance = utils::check_balance(address_str).await;
            println!("Address: {}", address_str);
            println!("Balance: {} SOL", balance);
        }
        Some(("create-wallet", _sub_m)) => {
            let result = utils::create_or_load();

            println!("Address: {}", result.pubkey());
        }

        Some(("send_sol", sub_m)) => {

            println!("Sending SOL ...");
        
            let wallet =
                utils::create_or_load();
        
            let amount: f64 =
                sub_m
                .get_one::<String>("amount")
                .unwrap()
                .parse()
                .unwrap();
        
            let receiver_address =
                sub_m
                .get_one::<String>("reciever_address")
                .unwrap();
        
            utils::send_sol(
                &wallet,
                amount,
                receiver_address,
            ).await;
        }

        _ => {
            println!("No valid subcommand provided. Use --help for more information.");
        }
    }
}
