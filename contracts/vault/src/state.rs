use cosmwasm_schema::{cw_serde};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use crate::assets::{VaultInfo};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub vault_info: VaultInfo,

}

#[cw_serde]
pub struct UserInfo {
    pub amount: Uint128,

}


pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const USER_INFO_KEY: &str = "user_info";
pub const USER_INFO: Map<&Addr, UserInfo> = Map::new(USER_INFO_KEY);
