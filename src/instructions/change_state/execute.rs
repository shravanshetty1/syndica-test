use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::entrypoint::ProgramResult;

use crate::state::ProgramState;

use super::ChangeState;

// create pda and set initial program state

impl ChangeState<'_> {
    pub fn execute_instruction(&self) -> ProgramResult {
        let mut program_state =
            ProgramState::deserialize(&mut &**self.accounts.program_state.data.borrow())?;
        program_state.state = self.args.state;
        program_state.serialize(&mut &mut self.accounts.program_state.data.borrow_mut()[..])?;

        Ok(())
    }
}
