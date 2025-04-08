use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    bs58,
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::{env, str::FromStr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let private_key_b58 = env::var("SECRET_KEY").expect("Solana private key is not set in .env");

    let private_key = bs58::decode(private_key_b58)
        .into_vec()
        .expect("Failed to decode base58 key");

    let sender =
        Keypair::from_bytes(&private_key).expect("Failed to create keypair from private key");

    let url = env::var("DEVNET_URL").expect("DEVNET_URL has not been found in .env");

    let connection = RpcClient::new(url.to_string());

    println!("Our public key is {}", sender.pubkey());

    let recipient = Pubkey::from_str("9JYvpbuMGGHgRABtAKfzeAMXhv68XeURBNfMW5pWmMxz")?;

    println!("Attempting to send 0.01 SOL to {}", recipient);

    let lamports = (0.01 * solana_sdk::native_token::LAMPORTS_PER_SOL as f64) as u64;
    let send_sol_instruction = system_instruction::transfer(&sender.pubkey(), &recipient, lamports);

    let memo_program_id = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")?;
    let memo_text = "Hello from Alex to Solana in Rust - 2024-04-05!";

    let add_memo_instruction = Instruction {
        program_id: memo_program_id,
        accounts: vec![AccountMeta::new(sender.pubkey(), true)],
        data: memo_text.as_bytes().to_vec(),
    };

    println!("Memo is: {}", memo_text);

    let add_priority_fee = ComputeBudgetInstruction::set_compute_unit_price(26000);

    let recent_blockhash = connection.get_latest_blockhash()?;

    let message = Message::new_with_blockhash(
        &[send_sol_instruction, add_memo_instruction, add_priority_fee],
        Some(&sender.pubkey()),
        &recent_blockhash,
    );

    let transaction = Transaction::new(&[&sender], message, recent_blockhash);
    let signature = connection.send_and_confirm_transaction(&transaction)?;
    println!("Transaction confirmed, signature: {}", signature);
    Ok(())
}
