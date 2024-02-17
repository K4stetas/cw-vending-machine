use cosmwasm_std::{Addr};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub chocolates: u64,
    pub water: u64,
    pub chips: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddMembers { admins: Vec<String> },
    Leave {},
}

#[cw_serde]
pub struct GreetResp {
    pub message: String,
}

#[cw_serde]
pub struct AdminsListResp {
    pub admins: Vec<Addr>,
}

#[cw_serde]
pub struct ItemsCount {
    pub chocolates: u64,
    pub water: u64,
    pub chips: u64,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GreetResp)]
    Greet {},
    #[returns(AdminsListResp)]
    AdminsList {},
    #[returns(ItemsCount)]
    Items {},
}