use solana_program_test::{processor, tokio, BanksClient};
use syndica_test::{
    entrypoint::process_instruction,
    instructions::{
        change_state::ChangeStateArgs, instantiate::InstantiateArgs, SyndicaTestInstruction,
    },
    state::ProgramState,
};

use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    system_instruction,
};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

#[tokio::test]
async fn functional_test() -> Result<(), Box<dyn std::error::Error>> {
    let program_id = syndica_test::id();
    let pt = solana_program_test::ProgramTest::new(
        "syndica_test",
        program_id,
        processor!(process_instruction),
    );
    let (client, mint, _) = pt.start().await;

    let user1 = Keypair::new();
    let user2 = Keypair::new();
    let user3 = Keypair::new();
    let program_state_acc = Keypair::new();

    request_airdrop(client.clone(), &mint, &user1, LAMPORTS_PER_SOL * 10).await?;
    request_airdrop(client.clone(), &mint, &user2, LAMPORTS_PER_SOL * 10).await?;
    request_airdrop(client.clone(), &mint, &user3, LAMPORTS_PER_SOL * 10).await?;
    println!("added sol to user accounts");

    instantiate_sc(
        client.clone(),
        program_id,
        &user1,
        &user2,
        &program_state_acc,
    )
    .await?;
    println!("instantiated smart contract");

    change_state(client.clone(), program_id, &user1, &program_state_acc, 1).await?;
    println!("user1 changed program state");

    change_state(client.clone(), program_id, &user2, &program_state_acc, 2).await?;
    println!("user2 changed program state");

    if change_state(client.clone(), program_id, &user3, &program_state_acc, 3)
        .await
        .is_ok()
    {
        return Err("user3 changed program state".to_string().into());
    } else {
        println!("user3 failed to change program state - success");
    }

    Ok(())
}

async fn change_state(
    mut client: BanksClient,
    program_id: Pubkey,
    signer: &Keypair,
    program_state_acc: &Keypair,
    state: u32,
) -> syndica_test::Result<()> {
    let change_state_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(program_state_acc.pubkey(), false),
            AccountMeta::new(signer.pubkey(), true),
        ],
        data: SyndicaTestInstruction::ChangeState(ChangeStateArgs { state }).try_to_vec()?,
    };

    create_and_send_tx(
        client.clone(),
        vec![change_state_instruction],
        vec![signer],
        Some(&signer.pubkey()),
    )
    .await?;

    let program_state = ProgramState::deserialize(
        &mut client
            .get_account(program_state_acc.pubkey())
            .await?
            .ok_or("could not find program state acc")?
            .data
            .as_slice(),
    )?;

    assert_eq!(program_state.state, state);
    Ok(())
}

async fn instantiate_sc(
    mut client: BanksClient,
    program_id: Pubkey,
    user1: &Keypair,
    user2: &Keypair,
    program_state_acc: &Keypair,
) -> syndica_test::Result<()> {
    let rent = client
        .get_rent()
        .await?
        .minimum_balance(ProgramState::space()? as usize);
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

    create_and_send_tx(
        client.clone(),
        vec![create_program_state_acc, instantiate_instruction],
        vec![user1, program_state_acc],
        Some(&user1.pubkey()),
    )
    .await?;

    let program_state = ProgramState::deserialize(
        &mut client
            .get_account(program_state_acc.pubkey())
            .await?
            .ok_or("could not find program state account")?
            .data
            .as_slice(),
    )?;

    assert_eq!(program_state.state, 0);
    assert_eq!(program_state.user1, user1.pubkey());
    assert_eq!(program_state.user2, user2.pubkey());

    Ok(())
}

async fn request_airdrop(
    client: BanksClient,
    mint: &Keypair,
    user: &Keypair,
    lamports: u64,
) -> syndica_test::Result<()> {
    let ix = system_instruction::transfer(&mint.pubkey(), &user.pubkey(), lamports);
    create_and_send_tx(client, vec![ix], vec![mint], Some(&mint.pubkey())).await?;
    Ok(())
}
async fn create_and_send_tx(
    mut client: BanksClient,
    instructions: Vec<Instruction>,
    signers: Vec<&dyn Signer>,
    payer: Option<&Pubkey>,
) -> Result<(), Box<dyn std::error::Error>> {
    let msg = Message::new(instructions.as_slice(), payer);
    let tx = Transaction::new(&signers, msg, client.get_latest_blockhash().await?);
    Ok(client.process_transaction(tx).await?)
}
