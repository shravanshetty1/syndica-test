use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct ProgramState {
    pub state: u32,
    pub user1: Pubkey,
    pub user2: Pubkey,
}

impl ProgramState {
    pub fn space() -> Result<u64, ProgramError> {
        Ok(ProgramState {
            state: u32::MAX,
            user1: Pubkey::default(),
            user2: Pubkey::default(),
        }
        .try_to_vec()?
        .len() as u64)
    }
}
