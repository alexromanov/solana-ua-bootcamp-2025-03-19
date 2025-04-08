use dotenv::dotenv;
use mpl_token_metadata::{
    instructions::{
        CreateMetadataAccountV3,
        CreateMetadataAccountV3InstructionArgs,
    },
    types::DataV2,
};
use solana_client::rpc_client::RpcClient;
use solana_program::{program_pack::Pack, pubkey::Pubkey, system_instruction};
use solana_sdk::{
    bs58,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::{
    instruction as token_instruction,
    state::Mint,
};
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

    let mint = Keypair::new();
    let mint_pubkey = mint.pubkey();
    println!("Mint address is: {}", mint_pubkey);

    let token_account = get_associated_token_address(&payer.pubkey(), &mint.pubkey());
    println!("Token account is: {}", token_account);

    let metadata_seeds = &[
        "metadata".as_bytes(),
        &mpl_token_metadata::ID.to_bytes(),
        &mint_pubkey.to_bytes(),
    ];
    let (metadata_pubkey, _) =
        Pubkey::find_program_address(metadata_seeds, &mpl_token_metadata::ID);
    println!("Metadata address is: {}", metadata_pubkey);

    let min_rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN)
        .expect("Failed to get minimum balance for rent exemption");

    let create_mint_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        min_rent,
        Mint::LEN as u64,
        &spl_token::ID,
    );

    let initialize_mint_ix = token_instruction::initialize_mint(
        &spl_token::ID,
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        2,
    ).expect("Failed to create initialize mint instruction");

    let create_assoc_account_ix =
        spl_associated_token_account::instruction::create_associated_token_account(
            &payer.pubkey(),
            &payer.pubkey(),
            &mint_pubkey,
            &spl_token::ID,
        );

    let mint_tokens_ix = token_instruction::mint_to(
        &spl_token::ID,
        &mint.pubkey(),
        &token_account,
        &payer.pubkey(),
        &[&payer.pubkey()],
        1000,
    ).expect("Failed to create mint token instruction");

    let metadata_args = CreateMetadataAccountV3InstructionArgs {
        data: DataV2 {
            name: "Solana UA Bootcamp 2025-04-08".to_string(),
            symbol: "UAB-3".to_string(),
            uri: "https://arweave.net/1234".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        is_mutable: true,
        collection_details: None,
    };

    let create_metadata_ix = CreateMetadataAccountV3 {
        metadata: metadata_pubkey,
        mint: mint_pubkey,
        mint_authority: payer.pubkey(),
        payer: payer.pubkey(),
        update_authority: (payer.pubkey(), true),
        system_program: solana_program::system_program::ID,
        rent: Some(solana_program::sysvar::rent::ID),
    }
    .instruction(metadata_args);

    let recent_blockhash = client.get_latest_blockhash()?;

    let transaction = Transaction::new_signed_with_payer(
        &[
            create_mint_account_ix,
            initialize_mint_ix,
            create_assoc_account_ix,
            mint_tokens_ix,
            create_metadata_ix,
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
