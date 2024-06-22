use anchor_lang::prelude::*;

/// Financial Information on Clusttr's RWAs
#[account]
pub struct Asset {
    /// Asset address on-chain
    pub id: Pubkey,
    /// Price of asset in usd
    /// note: figure is in two decimals; 100 == $1
    pub price: u64,
    /// Expected appreciation in a year
    pub appreciation_rate: u32,
    /// Annual rental value
    pub rent: u8,
    /// Total revenue generated from asset
    pub cumulative_revenue: u64,
    /// Total cost used in maintaining asset
    pub total_maintenance_cost: u64
}

impl Asset {
    pub const SIZE: usize = std::mem::size_of::<Asset>();

    /// Create new asset record for a given token, registering initial price, appreciation_rate & rent
    pub fn new(id: Pubkey, price: u64, appreciation_rate: u32, rent: u8) -> Asset {
        Asset {
            id,
            price,
            appreciation_rate,
            rent,
            cumulative_revenue: 0,
            total_maintenance_cost: 0
        }
    }
}