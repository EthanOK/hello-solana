#![allow(deprecated)]

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};


use crate::{instruction::CounterInstruction, state::CounterAccount};

pub struct Processor {}
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CounterInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            CounterInstruction::SayHello => {
                Self::process_say_hello(program_id, accounts)?;
                Ok(())
            }
            CounterInstruction::InitializeCounter { initial_value } => {
                Self::process_initialize_counter(program_id, accounts, initial_value)?;
                Ok(())
            }
            CounterInstruction::IncrementCounter => {
                Self::process_increment_counter(program_id, accounts)?;
                Ok(())
            }
        }
    }

    fn process_say_hello(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Hello, Solana!");
        Ok(())
    }

    fn process_initialize_counter(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        initial_value: u64,
    ) -> ProgramResult {
        // Create an iterator over the accounts
        let accounts_iter = &mut accounts.iter();

        let counter_account = next_account_info(accounts_iter)?;
        let payer_account = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        // Calculate the space needed for our counter data
        let account_space = 8; // 8 bytes for a u64

        // Get the minimum balance required for rent exemption
        let rent = Rent::get()?;
        let required_lamports = rent.minimum_balance(account_space);

        invoke(
            &system_instruction::create_account(
                payer_account.key,    // Account paying for creation
                counter_account.key,  // New account being created
                required_lamports,    // Lamports to transfer
                account_space as u64, // Space to allocate in bytes
                program_id,           // Program that will own this account (our program)
            ),
            &[
                payer_account.clone(),
                counter_account.clone(),
                system_program.clone(),
            ],
        )?;

        // Initialize the counter data
        let counter_data = CounterAccount {
            count: initial_value,
        };

        // Serialize and write the data to the account
        let mut account_data = &mut counter_account.data.borrow_mut()[..];
        counter_data.serialize(&mut account_data)?;

        msg!("Counter initialized with value: {}", initial_value);

        Ok(())
    }
    fn process_increment_counter(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        // Get the counter account to increment
        let counter_account = next_account_info(accounts_iter)?;

        // Security check: Verify this program owns the account
        if counter_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
        let mut counter_data = CounterAccount::try_from_slice(&counter_account.data.borrow())?;
        counter_data.count = counter_data
            .count
            .checked_add(1)
            .ok_or(ProgramError::InvalidInstructionData)?;
        // Serialize and write the data to the account
        let mut account_data = &mut counter_account.data.borrow_mut()[..];
        counter_data.serialize(&mut account_data)?;

        msg!("Counter incremented to: {}", counter_data.count);

        Ok(())
    }
}
