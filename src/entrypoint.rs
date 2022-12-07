use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

use crate::instructions::{
    change_state::ChangeState, instantiate::Instantiate, Instruction, SyndicaTestInstruction,
};

entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: &[u8],
) -> ProgramResult {
    msg!("creating instruction object!");
    let instruction_type = SyndicaTestInstruction::try_from_slice(args)?;

    let mut instruction: Box<dyn Instruction> = match instruction_type {
        SyndicaTestInstruction::Instantiate(args) => {
            Box::new(Instantiate::new(*program_id, accounts, args)?)
        }
        SyndicaTestInstruction::ChangeState(args) => {
            Box::new(ChangeState::new(*program_id, accounts, args)?)
        }
    };

    msg!("validating instruction");
    instruction.validate()?;
    msg!("executing instruction");
    instruction.execute()?;
    msg!("finished executing instruction");

    Ok(())
}
