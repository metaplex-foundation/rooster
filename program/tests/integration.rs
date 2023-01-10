#![cfg(feature = "test-bpf")]

use assert_matches::*;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_sdk::{signature::Signer, transaction::Transaction};

#[test]
fn test_validator_transaction() {
    solana_logger::setup_with_default("solana_program_runtime=debug");
    let program_id = Pubkey::new_unique();
}
