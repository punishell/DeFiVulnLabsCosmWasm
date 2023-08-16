use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;


#[cw_serde]
pub struct Config {
    pub owner: Addr,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const MARKET_KEY: &str = "MARKET";
pub const MARKETS: Item<Vec<String>> = Item::new(MARKET_KEY);
