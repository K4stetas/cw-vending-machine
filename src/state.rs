use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use cosmwasm_schema::{cw_serde};

#[cw_serde]
pub struct MachineItems {
    pub chocolates: u64,
    pub water_bottles: u64,
    pub chips: u64,
}

pub const OWNER: Item<Addr> = Item::new("owner");
pub const MACHINE: Item<MachineItems> = Item::new("machine_items");