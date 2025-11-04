#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CounterError {
    IncorrectProgramId,
    InvalidAccountData,
    InvalidInstructionData,
    InvalidInstruction,
}
