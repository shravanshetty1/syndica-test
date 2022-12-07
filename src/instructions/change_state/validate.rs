use borsh::BorshDeserialize;
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError};

use crate::state::ProgramState;

use super::ChangeState;

impl ChangeState<'_> {
    pub fn validate_instruction(&self) -> ProgramResult {
        if !self.accounts.signer.is_signer {
            solana_program::msg!("signer account has not signed the transaction");
            return Err(ProgramError::InvalidAccountData);
        }

        let program_state =
            ProgramState::deserialize(&mut &**self.accounts.program_state.data.borrow())?;

        if *self.accounts.signer.key != program_state.user1
            && *self.accounts.signer.key != program_state.user2
        {
            solana_program::msg!("signer account is not one of the 2 authorized user");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}
