use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub black_list: Option<Vec<Addr>>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Withdraw{
        destination: String,
    },
 
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns a human-readable representation of the arbiter.
    #[returns(OwnerResponse)]
    Owner {},
    #[returns(BlacklistResponse)]
    BlackList {},
}

#[cw_serde]
pub struct OwnerResponse {
    pub owner: Addr,
}
#[cw_serde]
pub struct BlacklistResponse {
    pub list: Vec<Addr>
}

