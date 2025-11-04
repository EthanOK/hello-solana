#[cfg(test)]
mod tests {
    use borsh::to_vec;
    use hello_solana::instruction::CounterInstruction;
    use litesvm::LiteSVM;
    use mollusk_svm::{result::Check, Mollusk};
    use solana_program::{native_token::LAMPORTS_PER_SOL, system_program::ID as SYSTEM_PROGRAM_ID};
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        message::Message,
        // pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        transaction::Transaction,
    };

    #[test]
    fn test_initialize_counter() {
        let mut svm = LiteSVM::new();
        let payer_keypair = Keypair::new();
        let payer = payer_keypair.pubkey();
        svm.airdrop(&payer, LAMPORTS_PER_SOL).unwrap();

        let program_keypair = read_keypair_file("target/deploy/hello_solana-keypair.json").unwrap();
        let program_id = program_keypair.pubkey();
        // let program_id = Pubkey::new_unique();
        svm.add_program_from_file(program_id, "target/deploy/hello_solana.so")
            .unwrap();

        let counter_keypair = Keypair::new();
        let counter_account = counter_keypair.pubkey();

        // Build Transaction
        let instruction_data =
            to_vec(&CounterInstruction::InitializeCounter { initial_value: 1 }).unwrap();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &instruction_data,
            vec![
                AccountMeta::new(counter_account, true), // counter account (必须是签名者才能创建)
                AccountMeta::new(payer, true),           // payer (签名者)
                AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false), // system program
            ],
        );
        let message = Message::new(&[instruction], Some(&payer));
        let blockhash = svm.latest_blockhash();
        let transaction = Transaction::new(&[&payer_keypair, &counter_keypair], message, blockhash);

        // Send the transaction
        let result = svm.send_transaction(transaction);
        match result {
            Ok(response) => {
                println!("Transaction successful!");
                let logs = response.logs;
                println!("Logs: {:?}", logs);
            }
            Err(e) => {
                eprintln!("Transaction failed: {:?}", e);
                // panic!("Transaction failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_hello_solana_mollusk() {
        let program_keypair = read_keypair_file("target/deploy/hello_solana-keypair.json").unwrap();
        let program_id = program_keypair.pubkey();
        let mollusk = Mollusk::new(&program_id, "target/deploy/hello_solana");

        let instruction_data = to_vec(&CounterInstruction::SayHello).unwrap();

        let instruction = Instruction::new_with_bytes(program_id, &instruction_data, vec![]);

        let result =
            mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);
        println!("Result: {:?}", result);
    }
}
