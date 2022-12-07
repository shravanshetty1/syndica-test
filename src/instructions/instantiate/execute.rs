use borsh::{BorshSerialize};
use solana_program::entrypoint::ProgramResult;

use crate::state::ProgramState;

use super::Instantiate;

// create pda and set initial program state

impl<'a,'b> Instantiate<'a,'b> {
    pub fn execute_instruction(&self) -> ProgramResult {
        let program_state =  ProgramState {
            state: 0,
            user1: self.args.user1,
            user2: self.args.user2,
        };
        program_state.serialize(&mut &mut self.accounts.program_state.data.borrow_mut()[..])?;

        Ok(())
    }
}
