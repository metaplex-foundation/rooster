use crate::{error::Crows, instruction::RoosterCommand};
use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::instruction::{
    builders::TransferBuilder, InstructionBuilder, TransferArgs,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program_error::ProgramError,
    program_memory::sol_memcpy,
    pubkey,
    pubkey::Pubkey,
};

pub mod assertions;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod pda;
pub mod processor;
pub mod state;

pub use mpl_token_metadata::processor::AuthorizationData;

solana_program::declare_id!("MyProgram1111111111111111111111111111111111");

pub const SPL_TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const SPL_ATA_TOKEN_PROGRAM_ID: Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
pub const MPL_TOKEN_AUTH_RULES_PROGRAM_ID: Pubkey =
    pubkey!("auth9SigNpDKz4sJJ1DfCTuZrZNSAgh9sFD3rboVmgg");
