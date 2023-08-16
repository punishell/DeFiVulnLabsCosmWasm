use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;


#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub markets: Vec<String>
    
}


#[cw_serde]
pub enum ExecuteMsg {

    Deposit{},
    UpdateMarkets{ market: String}
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
    #[returns(MarketsResponse)]
    Markets {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
}


#[cw_serde]
pub struct MarketsResponse {
    pub markets: Vec<String>,
}