use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use cosmwasm_storage::{singleton, singleton_read, Bucket, ReadonlyBucket};

static KEY_CONFIG: &[u8] = b"config";

static PREFIX_REWARD: &[u8] = b"reward";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub token: Addr,
}

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    singleton(storage, KEY_CONFIG).save(config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    singleton_read(storage, KEY_CONFIG).load()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerInfo {
    pub staked_amount: Uint128,

}

/// store stake info
pub fn store_staker_info(
    storage: &mut dyn Storage,
    owner: &Addr,
    staker_info: &StakerInfo,
) -> StdResult<()> {
    Bucket::new(storage, PREFIX_REWARD).save(owner.as_bytes(), staker_info)
}

/// remove staker_info of the given owner
pub fn remove_staker_info(storage: &mut dyn Storage, owner: &Addr) {
    Bucket::<StakerInfo>::new(storage, PREFIX_REWARD).remove(owner.as_bytes())
}

/// returns rewards owned by this owner
/// (read-only version for queries)
pub fn read_staker_info(storage: &dyn Storage, owner: &Addr) -> StdResult<StakerInfo> {
    match ReadonlyBucket::new(storage, PREFIX_REWARD).may_load(owner.as_bytes())? {
        Some(staker_info) => Ok(staker_info),
        None => Ok(StakerInfo {
            staked_amount: Uint128::zero(),

        }),
    }
}