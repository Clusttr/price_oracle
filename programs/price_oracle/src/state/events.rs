use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::UnixTimestamp;

#[event]
pub struct UpdateAssetEvent {
    pub id: Pubkey,
    pub price: u64,
    pub appreciation_rate: u32,
    pub rent: u8,
    pub timestamp: UnixTimestamp
}

#[event]
pub struct InitAssetEvent {
    pub id: Pubkey,
    pub price: u64,
    pub appreciation_rate: u32,
    pub rent: u8,
    pub timestamp: UnixTimestamp
}