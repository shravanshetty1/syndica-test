use std::{str::FromStr, thread::sleep, time::Duration};

use syndica_test::{
    instructions::{
        change_state::ChangeStateArgs, instantiate::InstantiateArgs, SyndicaTestInstruction,
    },
    state::ProgramState,
};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    system_instruction,
};
use solana_sdk::{
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};

#[test]
fn basic() -> Result<(), Box<dyn std::error::Error>> {
    let program_id = Pubkey::from_str(std::env::var("PROGRAM_ID")?.as_str())?;
    println!("testing program {program_id}");

    let user1 = Keypair::new();
    let user2 = Keypair::new();
    let user3 = Keypair::new();
    let program_state_acc = Keypair::new();

    let client: RpcClient = RpcClient::new("http://localhost:8899".to_string());
    let sig1 = client.request_airdrop(&user1.pubkey(), LAMPORTS_PER_SOL * 10)?;
    let sig2 = client.request_airdrop(&user2.pubkey(), LAMPORTS_PER_SOL * 10)?;
    let sig3 = client.request_airdrop(&user3.pubkey(), LAMPORTS_PER_SOL * 10)?;
    confirm_transactions(&client, vec![sig1, sig2, sig3])?;
    println!("added sol to user accounts");

    let rent = client.get_minimum_balance_for_rent_exemption(ProgramState::space()? as usize)?;
    let create_program_state_acc = system_instruction::create_account(
        &user1.pubkey(),
        &program_state_acc.pubkey(),
        rent,
        ProgramState::space()?,
        &program_id,
    );

    let instantiate_instruction = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(program_state_acc.pubkey(), false)],
        data: SyndicaTestInstruction::Instantiate(InstantiateArgs {
            user1: user1.pubkey(),
            user2: user2.pubkey(),
        })
        .try_to_vec()?,
    };

    let sig = create_and_send_tx(
        &client,
        vec![create_program_state_acc, instantiate_instruction],
        vec![&user1, &program_state_acc],
        Some(&user1.pubkey()),
    )?;
    confirm_transactions(&client, vec![sig])?;

    let program_state = ProgramState::deserialize(
        &mut client
            .get_account(&program_state_acc.pubkey())?
            .data
            .as_slice(),
    )?;

    assert_eq!(program_state.state, 0);
    assert_eq!(program_state.user1, user1.pubkey());
    assert_eq!(program_state.user2, user2.pubkey());

    println!("instantiated smart contract");

    let change_state_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(program_state_acc.pubkey(), false),
            AccountMeta::new(user1.pubkey(), true),
        ],
        data: SyndicaTestInstruction::ChangeState(ChangeStateArgs { state: 1 }).try_to_vec()?,
    };

    let sig = create_and_send_tx(
        &client,
        vec![change_state_instruction],
        vec![&user1],
        Some(&user1.pubkey()),
    )?;
    confirm_transactions(&client, vec![sig])?;

    let program_state = ProgramState::deserialize(
        &mut client
            .get_account(&program_state_acc.pubkey())?
            .data
            .as_slice(),
    )?;

    assert_eq!(program_state.state, 1);
    assert_eq!(program_state.user1, user1.pubkey());
    assert_eq!(program_state.user2, user2.pubkey());

    println!("user1 changed program state");

    let change_state_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(program_state_acc.pubkey(), false),
            AccountMeta::new(user2.pubkey(), true),
        ],
        data: SyndicaTestInstruction::ChangeState(ChangeStateArgs { state: 2 }).try_to_vec()?,
    };

    let sig = create_and_send_tx(
        &client,
        vec![change_state_instruction],
        vec![&user2],
        Some(&user2.pubkey()),
    )?;
    confirm_transactions(&client, vec![sig])?;

    let program_state = ProgramState::deserialize(
        &mut client
            .get_account(&program_state_acc.pubkey())?
            .data
            .as_slice(),
    )?;

    assert_eq!(program_state.state, 2);
    assert_eq!(program_state.user1, user1.pubkey());
    assert_eq!(program_state.user2, user2.pubkey());

    println!("user2 changed program state");

    let change_state_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(program_state_acc.pubkey(), false),
            AccountMeta::new(user3.pubkey(), true),
        ],
        data: SyndicaTestInstruction::ChangeState(ChangeStateArgs { state: 3 }).try_to_vec()?,
    };

    // transaction simulation will fail
    if create_and_send_tx(
        &client,
        vec![change_state_instruction],
        vec![&user3],
        Some(&user3.pubkey()),
    )
    .is_ok()
    {
        return Err("user3 changed program state".to_string().into());
    } else {
        println!("user3 failed to change program state - success");
    }

    Ok(())
}

fn create_and_send_tx(
    client: &RpcClient,
    instructions: Vec<Instruction>,
    signers: Vec<&dyn Signer>,
    payer: Option<&Pubkey>,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let msg = Message::new(instructions.as_slice(), payer);
    let tx = Transaction::new(&signers, msg, client.get_latest_blockhash()?);
    let sig = client.send_transaction(&tx)?;

    Ok(sig)
}

fn confirm_transactions(
    client: &RpcClient,
    sigs: Vec<Signature>,
) -> Result<(), Box<dyn std::error::Error>> {
    for sig in sigs {
        loop {
            if client.confirm_transaction(&sig)? {
                break;
            }
            sleep(Duration::from_millis(200))
        }
    }

    Ok(())
}
