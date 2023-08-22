use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;


#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub pool: String,
    pub allowed_denoms: Vec<String>
    
}


#[cw_serde]
pub enum ExecuteMsg {

    Deposit{},

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
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
}
