use super::*;

pub fn find_rooster_pda(authority: &Pubkey) -> (Pubkey, u8) {
    let seeds = &[b"rooster", authority.as_ref()];
    Pubkey::find_program_address(seeds, &crate::ID)
}
