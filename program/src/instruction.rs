use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::{pda::find_token_record_account, processor::AuthorizationData};
use shank::ShankInstruction;

use super::*;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct WithdrawArgs {
    pub auth_data: AuthorizationData,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct DelegateArgs {
    pub amount: u64,
    pub authority: Pubkey,
    pub bump: u8,
}

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum RoosterCommand {
    /// Initialize a new rooster
    #[account(0, writable, signer, name="authority", desc="Account owner")]
    #[account(1, writable, name="rooster_pda", desc = "Rooster PDA account")]
    #[account(2, name="system_program", desc = "The system program")]
    Init,

    /// Withdraw the token from the rooster by CPIing into Token Metadata 'Transfer'
    #[account(0, writable, signer, name="authority", desc="Account owner")]
    #[account(1, writable, name="rooster_pda", desc = "Rooster PDA account")]
    #[account(2, writable, name="token", desc = "Token account for rooster PDA")]
    #[account(3, name="destination_owner", desc = "Owner of the destination token account")]
    #[account(4, writable, name="destination", desc = "Destination token account")]
    #[account(5, name="mint", desc = "Token mint")]
    #[account(6, writable, name="metadata", desc = "Token metadata account")]
    #[account(7, name="edition", desc = "Token edition account")]
    #[account(8, name="token_record", desc = "Token record account")]
    #[account(9, name="token_metadata_program", desc = "The token metadata program")]
    #[account(10, name="system_program", desc = "The system program")]
    #[account(11, name="sysvar_instructions", desc = "The sysvar instructions")]
    #[account(12, name="spl_token_program", desc = "The token program")]
    #[account(13, name="spl_ata_program", desc = "The spl ata program")]
    #[account(14, name="authorization_rules_program", desc = "The authorization rules program")]
    #[account(15, name="authorization_rules", desc = "The authorization rules PDA account")]
    Withdraw(WithdrawArgs),

    /// Create delegate via Token Metadata CPI
    #[account(0, writable, signer, name="delegate", desc="Delegate account")]
    #[account(1, writable, name="rooster_pda", desc = "Rooster PDA account")]
    #[account(2, writable, name="token", desc = "Token account for rooster PDA")]
    #[account(3, name="mint", desc = "Token mint")]
    #[account(4, writable, name="metadata", desc = "Token metadata account")]
    #[account(5, name="edition", desc = "Token edition account")]
    #[account(6, name="delegate_record", desc = "Collection delegate record account")]
    #[account(7, name="token_metadata_program", desc = "The token metadata program")]
    #[account(8, name="system_program", desc = "The system program")]
    #[account(9, name="sysvar_instructions", desc = "The sysvar instructions")]
    #[account(10, name="spl_token_program", desc = "The token program")]
    Delegate(DelegateArgs),
    
}

pub fn init(authority: Pubkey, rooster_pda: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(rooster_pda, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: RoosterCommand::Init.try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn withdraw(
    authority: Pubkey,
    rooster_pda: Pubkey,
    token: Pubkey,
    destination_owner: Pubkey,
    destination: Pubkey,
    mint: Pubkey,
    metadata: Pubkey,
    edition: Pubkey,
    rule_set: Pubkey,
    args: WithdrawArgs,
) -> Instruction {
    let (token_record, _) = find_token_record_account(&mint, &rooster_pda);

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(rooster_pda, false),
            AccountMeta::new(token, false),
            AccountMeta::new_readonly(destination_owner, false),
            AccountMeta::new(destination, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(metadata, false),
            AccountMeta::new(edition, false),
            AccountMeta::new(token_record, false),
            AccountMeta::new_readonly(mpl_token_metadata::ID, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(solana_program::sysvar::instructions::id(), false),
            AccountMeta::new_readonly(SPL_TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(SPL_ATA_TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(MPL_TOKEN_AUTH_RULES_PROGRAM_ID, false),
            AccountMeta::new_readonly(rule_set, false),
        ],
        data: RoosterCommand::Withdraw(args).try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn delegate(
    delegate: Pubkey,
    rooster_pda: Pubkey,
    token: Pubkey,
    mint: Pubkey,
    metadata: Pubkey,
    edition: Pubkey,
    args: DelegateArgs,
) -> Instruction {
    let (token_record, _bump) = find_token_record_account(&mint, &rooster_pda);

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(delegate, true),
            AccountMeta::new(rooster_pda, false),
            AccountMeta::new(token, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(metadata, false),
            AccountMeta::new_readonly(edition, false),
            AccountMeta::new(token_record, false),
            AccountMeta::new_readonly(mpl_token_metadata::ID, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(solana_program::sysvar::instructions::id(), false),
            AccountMeta::new_readonly(SPL_TOKEN_PROGRAM_ID, false),
            AccountMeta::new_readonly(MPL_TOKEN_AUTH_RULES_PROGRAM_ID, false),
        ],
        data: RoosterCommand::Delegate(args).try_to_vec().unwrap(),
    }
}
