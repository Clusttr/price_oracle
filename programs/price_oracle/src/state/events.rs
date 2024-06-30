use anchor_lang::prelude::*;
// use anchor_lang::prelude::borsh::{BorshSerialize, BorshDeserialize};
// use borsh::{BorshDeserialize, BorshSerialize};

#[event]
pub struct UpdateAssetEvent {
    pub id: Pubkey,
    pub value: u64,
    pub appreciation_rate: u16,
    pub rent: u32,
    pub cumulative_revenue: u64,
    pub cumulative_maintenance_cost: u64,
    pub block: u64,
}

#[event]
#[derive(Debug, Clone)]//, Serialize, Deserialize)]
pub struct InitAssetEvent {
    pub id: Pubkey,
    pub value: u64,
    pub appreciation_rate: u16,
    pub rent: u32,
    pub cumulative_revenue: u64,
    pub cumulative_maintenance_cost: u64,
    pub block: u64,
}

pub trait EventTrait {
    fn stringify(&self) -> String;
}

impl EventTrait for UpdateAssetEvent {
    fn stringify(&self) -> String {
        format!("{{id: {}, value: {}, appreciation_rate: {}, rent: {}, cumulative_revenue: {}, cumulative_maintenance_cost: {}, block: {}}}",
                self.id,
                self.value,
                self.appreciation_rate,
                self.rent,
                self.cumulative_revenue,
                self.cumulative_maintenance_cost,
                self.block )
    }
}

impl EventTrait for InitAssetEvent {
    fn stringify(&self) -> String {
        format!("{{id: {}, value: {}, appreciation_rate: {}, rent: {}, cumulative_revenue: {}, cumulative_maintenance_cost: {}, block: {}}}",
                self.id,
                self.value,
                self.appreciation_rate,
                self.rent,
                self.cumulative_revenue,
                self.cumulative_maintenance_cost,
                self.block)
    }
}