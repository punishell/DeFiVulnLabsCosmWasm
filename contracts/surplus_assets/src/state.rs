use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;


#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub allowed_denoms: Vec<String>
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const POOL_KEY: &str = "POOL";
pub const POOL: Item<Addr> = Item::new(POOL_KEY);
