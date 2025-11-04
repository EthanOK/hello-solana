use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstruction {
    SayHello,
    InitializeCounter { initial_value: u64 },
    IncrementCounter,
}
