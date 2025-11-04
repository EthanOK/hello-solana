use solana_program::{
    declare_id, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey,
};

mod entrypoint;

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

declare_id!("He11oC24LKntiBzYP6w3uBgzWsjLj4fvf7AhrajtVaHd");

pub fn check_program_account(program_id: &Pubkey) -> ProgramResult {
    if program_id != &id() {
        msg!(
            "Program ID mismatch: expected {:?}, got {:?}",
            id(),
            program_id
        );
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}
