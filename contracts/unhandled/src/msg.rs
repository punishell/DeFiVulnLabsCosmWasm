use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Withdraw{
        destination: String,
    },
    UpdateConfig{
        new_owner: String,
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns a human-readable representation of the arbiter.
    #[returns(OwnerResponse)]
    Owner {},
}

#[cw_serde]
pub struct OwnerResponse {
    pub owner: Addr,
}
