use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub chocolate_bars: u64,
    pub water_bottles: u64,
    pub chips_packets: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    GetItem { category: String },
    Refill { number: u64 },
}

#[cw_serde]
pub struct Items{
    pub chocolate_bars: u64,
    pub water_bottles: u64,
    pub chips_packets: u64,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Items)]
    ItemsCount {},
}