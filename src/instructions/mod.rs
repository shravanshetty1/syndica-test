use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::entrypoint::ProgramResult;

use self::{change_state::ChangeStateArgs, instantiate::InstantiateArgs};

pub mod change_state;
pub mod instantiate;

pub trait Instruction {
    fn validate(&self) -> ProgramResult;
    fn execute(&mut self) -> ProgramResult;
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum SyndicaTestInstruction {
    Instantiate(InstantiateArgs),
    ChangeState(ChangeStateArgs),
}
