use std::{fs, str::FromStr};

use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use solana_system_interface::instruction::transfer;

use crate::rpc_client;

pub fn create_or_load() -> Keypair {
    if std::path::Path::new("wallet.json").exists() {
        println!("Wallet already exists. Loading wallet...");

        let readwallet = fs::read_to_string("wallet.json").unwrap();
        let secret_vec: Vec<u8> = serde_json::from_str(&readwallet).unwrap();
        let secret_array: [u8; 64] = secret_vec.try_into().unwrap();

        // SDK v3+: try_from replaces from_bytes
        Keypair::try_from(secret_array.as_ref()).unwrap()
    } else {
        println!("Creating new wallet...");

        let keypair = Keypair::new();
        let secret = keypair.to_bytes().to_vec();
        fs::write("wallet.json", serde_json::to_string(&secret).unwrap()).unwrap();

        keypair
    }
}

pub async fn airdrop_sol(address_str: &str, airdrop: Option<&String>) -> Option<Signature> {
    let client = rpc_client::get_rpc_client();
    let address = Pubkey::from_str(address_str).expect("Invalid address format");

    if let Some(amount_str) = airdrop {
        let amount: f64 = amount_str.parse().expect("Invalid airdrop amount");
        let lamports = (amount * LAMPORTS_PER_SOL as f64) as u64;

        let airdrop_sig = client
            .request_airdrop(&address, lamports)
            .await
            .unwrap();

        loop {
            if client.confirm_transaction(&airdrop_sig).await.unwrap() {
                println!("Airdrop confirmed!");
                return Some(airdrop_sig);
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }

    None
}

pub async fn check_balance(address_str: &str) -> f64 {
    let client = rpc_client::get_rpc_client();
    let address = Pubkey::from_str(address_str).expect("Invalid address format");
    let balance = client.get_balance(&address).await.unwrap();
    balance as f64 / LAMPORTS_PER_SOL as f64
}

pub async fn send_sol(wallet: &Keypair, amount: f64, receiver_address: &str) {
    let client = rpc_client::get_rpc_client();

    let receiver = Pubkey::from_str(receiver_address).expect("Invalid receiver address");
    let lamports = (amount * LAMPORTS_PER_SOL as f64) as u64;

    let balance = client.get_balance(&wallet.pubkey()).await.unwrap();
    if balance < lamports {
        println!("Insufficient balance");
        return;
    }

    let transfer_ix = transfer(&wallet.pubkey(), &receiver, lamports);

    let latest_blockhash = client.get_latest_blockhash().await.unwrap();

    let mut transaction = Transaction::new_with_payer(&[transfer_ix], Some(&wallet.pubkey()));
    transaction.sign(&[wallet], latest_blockhash);

    match client.send_and_confirm_transaction(&transaction).await {
        Ok(signature) => {
            println!("Transaction successful!");
            println!("Signature: {}", signature);
        }
        Err(err) => {
            println!("Transaction failed: {:?}", err);
        }
    }
}