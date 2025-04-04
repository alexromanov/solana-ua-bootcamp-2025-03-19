use dotenv::dotenv;
use solana_sdk::{bs58, pubkey::Pubkey, signature::Keypair, signer::Signer};
use std::env;
use solana_client::rpc_client::RpcClient;
use log::{info, debug};
use std::time::Instant;

fn main() {
    env_logger::init();
    info!("Generate Solana keypair");
    let generated_key = generate_keypair();
    print_keypair(&generated_key);

    info!("Load Solana keypair from .env");
    let load_key = load_keypair();
    print_keypair(&load_key);

    info!("Check balance of the configured address");
    let balance = check_balance(&load_key.pubkey());
    println!("Balance for the wallet at address: {} is {} SOL", load_key.pubkey(), balance);

    let prefix = "A";
    info!("Generate key with prefix: {prefix}");
    let generated = generate_keypair_with_prefix(prefix);
    print_keypair(&generated);
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
    debug!("Keypair is generated: {:?}", keypair);
    keypair
}

fn load_keypair() -> Keypair {
    dotenv().ok();

    debug!("Reading Base58 SECRET_KEY from .env");
    let private_key_b58 = env::var("SECRET_KEY")
        .expect("Solana private key is not set in .env");

    debug!("Getting private key bytes vec from base58");
    let private_key = bs58::decode(private_key_b58).into_vec()
        .expect("Failed to decode base58 key");

    Keypair::from_bytes(&private_key)
        .expect("Failed to create keypair from private key")
}

fn check_balance(public_key: &Pubkey) -> f64 {
    dotenv().ok();

    let url = env::var("DEVNET_URL")
        .expect("DEVNET_URL has not been found in .env");

    let client = RpcClient::new(url);

    debug!("Getting the balance from a pubkey: {}", public_key);
    let balance = client.get_balance(public_key)
        .expect("Failed to retrieve a balance from the walllet");

    let balance_in_sol = balance as f64 / 1_000_000_000.0;
    balance_in_sol
}

fn generate_keypair_with_prefix(prefix: &str) -> Keypair {
    debug!("Generating keypair with public key prefix: {}", prefix);

    let start = Instant::now();
    let mut attempts = 0;
    let mut keypair;

    loop {
        attempts += 1;
        keypair = Keypair::new();
        let public_key = keypair.pubkey().to_string();

        if public_key.starts_with(prefix) {
            debug!("Key found after {} attempts!", attempts);
            debug!("Public Key: {}", public_key);
            debug!("Private Key (Base58): {}", bs58::encode(keypair.to_bytes()).into_string());
            debug!("Took {:.2?} seconds", start.elapsed());
            break;
        }

        if attempts % 100_000 == 0 {
            debug!("Attempts: {}", attempts);
        }
    }
    keypair
}
