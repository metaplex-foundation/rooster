use mpl_token_metadata::instruction::builders::DelegateBuilder;
use solana_program::program::invoke_signed;

use crate::{
    assertions::assert_rooster_pda,
    instruction::{DelegateArgs, WithdrawArgs},
    state::Rooster,
};

use super::*;

pub struct Processor;
impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: RoosterCommand = RoosterCommand::try_from_slice(instruction_data)?;
        match instruction {
            RoosterCommand::Init => init(program_id, accounts),
            RoosterCommand::Withdraw(args) => withdraw(program_id, accounts, args),
            RoosterCommand::Delegate(args) => delegate(program_id, accounts, args),
        }
    }
}

fn init(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Rooster: Init");

    let account_iter = &mut accounts.iter();
    let authority_info = next_account_info(account_iter)?;
    let rooster_pda_info = next_account_info(account_iter)?;
    let system_program_info = next_account_info(account_iter)?;

    if !authority_info.is_signer {
        return Err(Crows::NotASigner.into());
    }

    let bump = assert_rooster_pda(rooster_pda_info, authority_info)?;
    let rooster = Rooster::new(bump);
    let rooster_signer_seeds = &[b"rooster", authority_info.key.as_ref(), &[bump]];

    let serialized_rooster = rooster.try_to_vec()?;
    let data_len = serialized_rooster.len();

    mpl_utils::create_or_allocate_account_raw(
        *program_id,
        rooster_pda_info,
        system_program_info,
        authority_info,
        data_len,
        rooster_signer_seeds,
    )?;

    msg!("Writing state");
    sol_memcpy(
        &mut rooster_pda_info.data.borrow_mut(),
        serialized_rooster.as_slice(),
        data_len,
    );

    Ok(())
}

pub fn withdraw(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: WithdrawArgs,
) -> ProgramResult {
    msg!("Rooster: Withdraw");

    let account_iter = &mut accounts.iter();
    let authority_info = next_account_info(account_iter)?;
    let rooster_pda_info = next_account_info(account_iter)?;
    let token_info = next_account_info(account_iter)?;
    let destination_owner_info = next_account_info(account_iter)?;
    let destination_info = next_account_info(account_iter)?;
    let mint_info = next_account_info(account_iter)?;
    let metadata_info = next_account_info(account_iter)?;
    let edition_info = next_account_info(account_iter)?;
    let token_metadata_program_info = next_account_info(account_iter)?;
    let system_program_info = next_account_info(account_iter)?;
    let sysvar_instructions_info = next_account_info(account_iter)?;
    let spl_token_program_info = next_account_info(account_iter)?;
    let spl_ata_program_info = next_account_info(account_iter)?;
    let mpl_token_auth_rules_program_info = next_account_info(account_iter)?;
    let rule_set_info = next_account_info(account_iter)?;

    let bump = assert_rooster_pda(rooster_pda_info, authority_info)?;
    let signer_seeds = &[b"rooster", authority_info.key.as_ref(), &[bump]];

    let transfer_args = TransferArgs::V1 {
        authorization_data: Some(args.auth_data),
        amount: 1,
    };

    let mut builder = TransferBuilder::new();
    builder
        .authority(*rooster_pda_info.key)
        .token_owner(*rooster_pda_info.key)
        .token(*token_info.key)
        .destination_owner(*destination_owner_info.key)
        .destination(*destination_info.key)
        .mint(*mint_info.key)
        .metadata(*metadata_info.key)
        .edition(*edition_info.key)
        .authorization_rules(*rule_set_info.key)
        .payer(*authority_info.key);

    let build_result = builder.build(transfer_args);

    let instruction = match build_result {
        Ok(transfer) => transfer.instruction(),
        Err(err) => {
            msg!("Error building transfer instruction: {:?}", err);
            return Err(Crows::TransferBuilderFailed.into());
        }
    };

    let account_infos = [
        token_info.clone(),
        rooster_pda_info.clone(),
        destination_info.clone(),
        destination_owner_info.clone(),
        mint_info.clone(),
        metadata_info.clone(),
        edition_info.clone(),
        authority_info.clone(),
        token_metadata_program_info.clone(),
        system_program_info.clone(),
        sysvar_instructions_info.clone(),
        spl_token_program_info.clone(),
        spl_ata_program_info.clone(),
        mpl_token_auth_rules_program_info.clone(),
        rule_set_info.clone(),
    ];

    invoke_signed(&instruction, &account_infos, &[signer_seeds]).unwrap();

    Ok(())
}

pub fn delegate(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: DelegateArgs,
) -> ProgramResult {
    msg!("Rooster: Delegate");
    let DelegateArgs {
        amount,
        authority,
        bump,
    } = args;

    let account_iter = &mut accounts.iter();
    let delegate_info = next_account_info(account_iter)?;
    let rooster_pda_info = next_account_info(account_iter)?;
    let token_info = next_account_info(account_iter)?;
    let mint_info = next_account_info(account_iter)?;
    let metadata_info = next_account_info(account_iter)?;
    let edition_info = next_account_info(account_iter)?;
    let delegate_record_info = next_account_info(account_iter)?;
    let token_metadata_program_info = next_account_info(account_iter)?;
    let system_program_info = next_account_info(account_iter)?;
    let sysvar_instructions_info = next_account_info(account_iter)?;
    let spl_token_program_info = next_account_info(account_iter)?;
    let _mpl_token_auth_rules_program_info = next_account_info(account_iter)?;

    let signer_seeds = &[b"rooster", authority.as_ref(), &[bump]];

    let transfer_args = mpl_token_metadata::instruction::DelegateArgs::TransferV1 {
        amount,
        authorization_data: None,
    };

    let mut builder = DelegateBuilder::new();
    builder
        .approver(*rooster_pda_info.key)
        .delegate(*delegate_info.key)
        .delegate_record(*delegate_record_info.key)
        .token(*token_info.key)
        .mint(*mint_info.key)
        .metadata(*metadata_info.key)
        .master_edition(*edition_info.key)
        .payer(*delegate_info.key);

    let build_result = builder.build(transfer_args);

    let instruction = match build_result {
        Ok(delegate) => delegate.instruction(),
        Err(err) => {
            msg!("Error building transfer instruction: {:?}", err);
            return Err(Crows::DelegateBuilderFailed.into());
        }
    };

    let account_infos = [
        delegate_info.clone(),
        delegate_record_info.clone(),
        token_info.clone(),
        rooster_pda_info.clone(),
        mint_info.clone(),
        metadata_info.clone(),
        edition_info.clone(),
        token_metadata_program_info.clone(),
        system_program_info.clone(),
        sysvar_instructions_info.clone(),
        spl_token_program_info.clone(),
    ];

    invoke_signed(&instruction, &account_infos, &[signer_seeds]).unwrap();

    Ok(())
}
