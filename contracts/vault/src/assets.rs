use cosmwasm_std::Addr;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct VaultInfo {
    pub contract_addr: Addr,
    pub liquidity_token: Addr,
    pub asset_infos: Vec<AssetInfo>,
}

#[cw_serde]
#[derive(Hash, Eq)]
pub enum AssetInfo {
    /// Non-native Token
    Token { contract_addr: Addr },
    /// Native token
    NativeToken { denom: String },
}