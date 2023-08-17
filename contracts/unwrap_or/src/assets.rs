use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Uint128};

#[cw_serde]
pub struct PriceResponse {
    pub price: Coin,
}

impl Default for PriceResponse {
    fn default() -> Self {
        Self {
            price: Coin {
                denom: "USD".to_string(),
                amount: Uint128::zero(),
            },
        }
    }
}


#[cw_serde]
pub enum OracleQueryMsg{
    Price {denom:String},
}