use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub token: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Withdraw{
        destination: String,
    },
    UpdateConfig{
        owner: String,
        token: String,
    },
    Receive(Cw20ReceiveMsg),
}

#[cw_serde]
pub enum Cw20HookMsg {
    Stake {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns a human-readable representation of the arbiter.
    #[returns(OwnerResponse)]
    Owner {},
    #[returns(StakeResponse)]
    StakerInfo {staker: String},
}

#[cw_serde]
pub struct OwnerResponse {
    pub owner: Addr,
}
#[cw_serde]
pub struct StakeResponse {
    pub staker: Addr,
    pub amount: Uint128,
}

