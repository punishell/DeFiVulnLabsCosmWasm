use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub asset_infos: Vec<crate::assets::AssetInfo>
}


#[cw_serde]
pub enum ExecuteMsg {

    Receive(Cw20ReceiveMsg),
}


#[cw_serde]
pub enum Cw20HookMsg {
    Deposit {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns a human-readable representation of the arbiter.
    #[returns(ConfigResponse)]
    Config {},
    #[returns(ConfigResponse)]
    UserInfo{ user: String},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
}
#[cw_serde]
pub struct UserResponse {
    pub ammount: Uint128,
}

