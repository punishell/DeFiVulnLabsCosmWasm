use std::error::Error;

use cosmwasm_std::{
    entry_point, to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, StdError, from_binary, Uint128,
};

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, QueryMsg, ExecuteMsg, ConfigResponse, Cw20HookMsg, UserResponse};
use crate::state::{Config, CONFIG, USER_INFO, UserInfo};
use crate::assets::{VaultInfo, AssetInfo};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};

// Version info, for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-merkle-airdrop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    if msg.asset_infos.len() != 2 {
        return Err(StdError::generic_err("asset_infos must contain exactly two elements").into());
    }

    if msg.asset_infos[0] == msg.asset_infos[1] {
        return Err(ContractError::DoublingAssets {});
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: deps.api.addr_validate(&msg.owner)?,
        vault_info: VaultInfo {
            contract_addr: env.contract.address.clone(),
            liquidity_token: Addr::unchecked(""),
            asset_infos: msg.asset_infos.clone(),
        },
    };

    CONFIG.save(deps.storage, &config)?;

    // Instnatinate the LP token contract  here:
    // TODO


    Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        }
    }

    pub fn receive_cw20(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        cw20_msg: Cw20ReceiveMsg,
    ) -> Result<Response, ContractError> {
        match from_binary(&cw20_msg.msg) {
            Ok(Cw20HookMsg::Deposit {}) => {
                let mut authorized = false;
                let cfg = CONFIG.load(deps.storage)?;
                for asset in cfg.vault_info.asset_infos {
                    if let AssetInfo::Token { contract_addr, .. } = &asset{
                        if contract_addr == &info.sender
                        {
                            authorized = true;
                        }
                    } 
                }
                if !authorized{
                    return Err(ContractError::Unauthorized {}); 
                }
                let cw20_sender = deps.api.addr_validate(&cw20_msg.sender)?;

                //Bad implementation
                USER_INFO.save(deps.storage, &cw20_sender, &UserInfo {amount: cw20_msg.amount })?;
                //Good inplementation
                // USER_INFO.update(deps.storage, &cw20_sender, |user_info:Option<UserInfo>| -> StdResult<_>{
                //     let mut info = user_info.unwrap_or(UserInfo { amount: 0u128.into()});
                //     info.amount = info.amount + cw20_msg.amount;
                //     Ok(info)
                // })?; 
                
                Ok(Response::new()) 
          
            }
            Err(_) => Err(ContractError::MissingData {}),
        }
    }


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config{} => to_binary(&query_config(deps)?),
        QueryMsg::UserInfo{user} => {
            let user_addr = deps.api.addr_validate(&user)?;
            to_binary(&query_user(deps, user_addr)?)},
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let addr = config.owner;
    Ok(ConfigResponse{ owner: addr })
}

fn query_user(deps: Deps, user_addr:Addr) -> StdResult<UserResponse> {

    let user_info = USER_INFO.load(deps.storage, &user_addr)?;
    let ammout = user_info.amount;
    Ok(UserResponse{ ammount: ammout })
}