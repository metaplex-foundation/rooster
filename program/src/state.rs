use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct Rooster {
    bump: u8,
}

impl Rooster {
    pub fn new(bump: u8) -> Self {
        Self { bump }
    }
}
