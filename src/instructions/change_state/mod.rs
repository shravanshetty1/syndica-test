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
pub struct ChangeStateArgs {
    pub state: u32,
}

#[derive(Debug)]
struct ChangeStateAccounts<'a> {
    program_state: &'a AccountInfo<'a>,
    signer: &'a AccountInfo<'a>,
}

pub struct ChangeState<'a> {
    program_id: Pubkey,
    accounts: ChangeStateAccounts<'a>,
    args: ChangeStateArgs,
}
impl<'a> ChangeState<'a> {
    pub fn new(
        program_id: Pubkey,
        accounts: &'a [AccountInfo<'a>],
        args: ChangeStateArgs,
    ) -> Result<Self, ProgramError> {
        let accounts = &mut accounts.iter();

        let program_state = next_account_info(accounts)?;
        let signer = next_account_info(accounts)?;

        Ok(ChangeState {
            program_id,
            accounts: ChangeStateAccounts {
                program_state,
                signer,
            },
            args,
        })
    }
}

impl Instruction for ChangeState<'_> {
    fn validate(&self) -> solana_program::entrypoint::ProgramResult {
        self.validate_instruction()
    }

    fn execute(&mut self) -> solana_program::entrypoint::ProgramResult {
        self.execute_instruction()
    }
}
