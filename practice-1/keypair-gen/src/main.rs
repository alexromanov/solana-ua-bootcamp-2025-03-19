use dotenv::dotenv;
use solana_sdk::{bs58, pubkey::Pubkey, signature::Keypair, signer::Signer};
use std::env;
use solana_client::rpc_client::RpcClient;

fn main() {
    println!("\nGenerate Solana keypair\n");
    let generated_key = generate_keypair();
    print_keypair(&generated_key);

    println!("\nLoad Solana keypair from .env\n");
    let load_key = load_keypair();
    print_keypair(&load_key);

    println!("\nCheck balance\n");
    let balance = check_balance(&load_key.pubkey());
    println!("Balance for the wallet at address: {} is {} SOL", load_key.pubkey(), balance);
}

fn print_keypair(keypair: &Keypair) {
    let public_key = keypair.pubkey();
    println!("Public key: {}", public_key);

    let private_key = keypair.to_bytes();
    println!("Private key: {:?}", private_key);

    let private_key_b58 = bs58::encode(private_key).into_string();
    println!("Private key (base58): {}", private_key_b58);
}

fn generate_keypair() -> Keypair {
    let keypair = Keypair::new();
    keypair
}

fn load_keypair() -> Keypair {
    dotenv().ok();

    let private_key_b58 = env::var("SECRET_KEY")
        .expect("Solana private key is not set in .env");

    let private_key = bs58::decode(private_key_b58).into_vec()
        .expect("Failed to decode base58 key");

    Keypair::from_bytes(&private_key)
        .expect("Failed to create keypair from private key")
}

fn check_balance(public_key: &Pubkey) -> f64 {
    let url = "https://api.devnet.solana.com";
    let client = RpcClient::new(url);

    let balance = client.get_balance(public_key)
        .expect("Failed to retrieve a balance from the walllet");

    let balance_in_sol = balance as f64 / 1_000_000_000.0;
    balance_in_sol
}
