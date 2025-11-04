use borsh::to_vec;
use hello_solana::instruction::CounterInstruction;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use solana_transaction_status::UiTransactionEncoding;

#[tokio::main]
async fn main() {
    let program_keypair = read_keypair_file("target/deploy/hello_solana-keypair.json").unwrap();
    let program_id = program_keypair.pubkey();

    let rpc_url = "http://localhost:8899";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let payer = Keypair::new();

    let airdrop_amount = LAMPORTS_PER_SOL * 10;
    let signature = client
        .request_airdrop(&payer.pubkey(), airdrop_amount)
        .expect("Airdrop failed");

    loop {
        let confirmed = client.confirm_transaction(&signature).unwrap();

        if confirmed {
            println!("Airdrop successful");
            break;
        }
    }

    let instruction_data = to_vec(&CounterInstruction::SayHello).unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &instruction_data,
        vec![AccountMeta::new(payer.pubkey(), true)],
    );

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[&payer], recent_blockhash);

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("Transaction Signature: {}", signature);

            match client.get_transaction_with_config(
                &signature,
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Json),
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(0),
                },
            ) {
                Ok(transaction) => {
                    let log_messages = transaction.transaction.meta.unwrap().log_messages.unwrap();
                    println!("Transaction Log Messages: {:?}", log_messages);
                }
                Err(err) => eprintln!("Error getting transaction: {:?}", err),
            }
        }

        Err(err) => eprintln!("Error sending transaction: {:?}", err),
    }
}
