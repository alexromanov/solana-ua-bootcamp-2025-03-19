use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    bs58,
    program_pack::Pack,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use spl_associated_token_account::{get_associated_token_address, instruction as ata_instruction};
use spl_token::{ID as token_program_id, instruction as token_instruction, state::Mint};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let private_key_b58 = env::var("SECRET_KEY").expect("Solana private key is not set in .env");

    let private_key = bs58::decode(private_key_b58)
        .into_vec()
        .expect("Failed to decode base58 key");

    let payer =
        Keypair::from_bytes(&private_key).expect("Failed to create keypair from private key");

    println!("Public key is: {}", payer.pubkey());

    let url = env::var("DEVNET_URL").expect("DEVNET_URL has not been found in .env");

    let client = RpcClient::new(url.to_string());

    let decimals = 2;
    let mint = Keypair::new();

    let mint_rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN)
        .expect("Failed to get minimum balance for rent");

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &token_program_id,
    );

    let initialize_mint_ix = token_instruction::initialize_mint(
        &token_program_id,
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        decimals,
    ).expect("Failed to create initialize mint instruction");

    let associated_token_address = get_associated_token_address(&payer.pubkey(), &mint.pubkey());

    let create_ata_ix = ata_instruction::create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint.pubkey(),
        &token_program_id,
    );

    let base: u32 = 10;
    let exp: u32 = 2;
    let minor_unit_per_major_units = base.pow(exp);
    let amount = 10 * minor_unit_per_major_units;

    let mint_to_ix = token_instruction::mint_to(
        &token_program_id,
        &mint.pubkey(),
        &associated_token_address,
        &payer.pubkey(),
        &[],
        amount.into(),
    ).expect("Failed to create mint intruction");

    let recent_blockhash = client.get_latest_blockhash()?;

    let transaction = Transaction::new_signed_with_payer(
        &[
            create_account_ix,
            initialize_mint_ix,
            create_ata_ix,
            mint_to_ix,
        ],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)
        .expect("Failed to send and confirm transaction");

    let explorer_link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        mint.pubkey()
    );
    println!("Token mint created: {}", explorer_link);
    println!("Transaction signature: {}", signature);

    Ok(())
}
