use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Uint128, from_binary, CosmosMsg, WasmMsg
};

use crate::error::{ContractError};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, OwnerResponse, StakeResponse, Cw20HookMsg};
use crate::state::{Config, StakerInfo, store_staker_info, read_staker_info, read_config, store_config};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};


// Version info, for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-merkle-airdrop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    store_config(deps.storage, &Config {
        owner:  deps.api.addr_validate(&msg.owner)?,
        token: deps.api.addr_validate(&msg.token)?,
    })?;
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

        ExecuteMsg::Withdraw { destination } => execute_withdraw(deps, env, info, destination),
        ExecuteMsg::UpdateConfig {owner, token } => execute_update(deps, env, info, owner, token),
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
    }
}
fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination: String,
  ) -> Result<Response, ContractError> {
    let config: Config = read_config(deps.storage)?;
    let staker_info: StakerInfo = read_staker_info(deps.storage, &info.sender)?;
    //SOME COMPUTATION SHOULD BE HERE BUT THIS  LATER   
    let amount = staker_info.staked_amount;
    Ok(Response::new()
    .add_messages(vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: info.sender.to_string(),
            amount,
        })?,
        funds: vec![],
    })])
    .add_attributes(vec![
        ("action", "withdraw"),
        ("owner", info.sender.as_str()),
        ("amount", amount.to_string().as_str()),
    ]))
  }

  fn execute_update(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
    token: String,
  ) -> Result<Response, ContractError> {

    let config = read_config(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }

    store_config(deps.storage, &Config {
        owner: deps.api.addr_validate(&owner)?,
        token: deps.api.addr_validate(&token)?,
    })?;

    let resp= Response::new()
    .add_attribute("action", "UpdateConfig")
    .add_attribute("Owner", owner.to_string())
    .add_attribute("Token", token.to_string());
    Ok(resp)
    
  }

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Stake {}) => {
            // only owner can create allocation
            // if config.token != info.sender {
            //     return Err(ContractError::Unauthorized {});
            // }

            let cw20_sender = deps.api.addr_validate(&cw20_msg.sender)?;
            stake(deps, env, cw20_sender, cw20_msg.amount)
        }
        Err(_) => Err(ContractError::MissingData {}),
    }
}

pub fn stake(deps: DepsMut, env: Env, sender_addr: Addr, amount: Uint128) -> Result<Response, ContractError>  {
    let sender_addr: Addr = deps.api.addr_validate(sender_addr.as_str())?;
    let mut staker_info: StakerInfo = read_staker_info(deps.storage, &sender_addr)?;
    staker_info.staked_amount += amount;
    store_staker_info(deps.storage, &sender_addr, &staker_info)?;
    Ok(Response::new().add_attributes(vec![
        ("action", "stake"),
        ("owner", sender_addr.as_str()),
        ("amount", amount.to_string().as_str()),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner{} => to_binary(&query_owner(deps)?),
        QueryMsg::StakerInfo{staker} => to_binary(&query_staked(deps, staker)?),
    }
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let config: Config = read_config(deps.storage)?;
    let addr = config.owner;
    Ok(OwnerResponse{ owner: addr })
}

fn query_staked(deps: Deps, staker: String) -> StdResult<StakeResponse> {
    let staker_addr = deps.api.addr_validate(&staker)?;
    let staker_info = read_staker_info(deps.storage, &staker_addr)?;
    let amount = staker_info.staked_amount;
    Ok(StakeResponse{ staker: staker_addr, amount })
}

