use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub chocolates: u64,
    pub water_bottles: u64,
    pub chips: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    GetItem { category: String },
    Refill { number: u64 },
}

#[cw_serde]
pub struct Items{
    pub chocolates: u64,
    pub water_bottles: u64,
    pub chips: u64,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Items)]
    ItemsCount {},
}