use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use super::Instruction;

pub mod execute;
pub mod validate;

#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct InstantiateArgs {
    pub user1: Pubkey,
    pub user2: Pubkey,
}

#[derive(Debug)]
struct InstantiateAccounts<'a> {
    program_state: &'a AccountInfo<'a>,
}

pub struct Instantiate<'a> {
    program_id: Pubkey,
    accounts: InstantiateAccounts<'a>,
    args: InstantiateArgs,
}
impl<'a> Instantiate<'a> {
    pub fn new(
        program_id: Pubkey,
        accounts: &'a [AccountInfo<'a>],
        args: InstantiateArgs,
    ) -> Result<Self, ProgramError> {
        let accounts = &mut accounts.iter();

        let program_state = next_account_info(accounts)?;

        Ok(Instantiate {
            program_id,
            accounts: InstantiateAccounts { program_state },
            args,
        })
    }
}

impl Instruction for Instantiate<'_> {
    fn validate(&self) -> solana_program::entrypoint::ProgramResult {
        self.validate_instruction()
    }

    fn execute(&mut self) -> solana_program::entrypoint::ProgramResult {
        self.execute_instruction()
    }
}
