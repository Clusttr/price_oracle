use anchor_lang::prelude::*;

#[account]
pub struct Asset {
    pub id: Pubkey,
    pub price: u64,
    pub appreciation_rate: u32,
    pub rent: u8
}

impl Asset {
    pub const SIZE: usize = std::mem::size_of::<Asset>();
}