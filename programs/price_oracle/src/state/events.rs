use anchor_lang::prelude::*;
// use anchor_lang::solana_program::clock::UnixTimestamp;

#[event]
pub struct UpdateAssetEvent {
    pub id: Pubkey,
    pub value: u64,
    pub appreciation_rate: u16,
    pub rent: u32,
    pub cumulative_revenue: u64,
    pub cumulative_maintenance_cost: u64,
    // pub timestamp: UnixTimestamp,
}

#[event]
pub struct InitAssetEvent {
    pub id: Pubkey,
    pub value: u64,
    pub appreciation_rate: u16,
    pub rent: u32,
    pub cumulative_revenue: u64,
    pub cumulative_maintenance_cost: u64,
    // pub timestamp: UnixTimestamp
}