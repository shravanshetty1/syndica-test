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
struct ChangeStateAccounts<'a, 'b> {
    program_state: &'a AccountInfo<'b>,
    signer: &'a AccountInfo<'b>,
}

pub struct ChangeState<'a, 'b> {
    program_id: Pubkey,
    accounts: ChangeStateAccounts<'a, 'b>,
    args: ChangeStateArgs,
}
impl<'a, 'b> ChangeState<'a, 'b> {
    pub fn new(
        program_id: Pubkey,
        accounts: &'a [AccountInfo<'b>],
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

impl<'a, 'b> Instruction for ChangeState<'a, 'b> {
    fn validate(&self) -> solana_program::entrypoint::ProgramResult {
        self.validate_instruction()
    }

    fn execute(&mut self) -> solana_program::entrypoint::ProgramResult {
        self.execute_instruction()
    }
}
