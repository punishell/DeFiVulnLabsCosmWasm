use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Coin, BankMsg
};

use crate::error::{ContractError};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, OwnerResponse, BlacklistResponse};
use crate::state::{Config, read_config, store_config};
use cw2::set_contract_version;



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
        black_list:vec![deps.api.addr_validate(&msg.owner)?]
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
    }
}
fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination: String,
  ) -> Result<Response, ContractError> {
    let config = read_config(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }
    let black_list= config.black_list;
    let destination = Addr::unchecked(destination);
    if black_list.iter().any(|addr| addr==&destination) {
        return Err(ContractError::Unauthorized{});
}
    
    let destination =destination.to_string().to_lowercase();
    let contract_address = env.contract.address;
    let amount = deps.querier.query_all_balances(&contract_address)?;
    Ok(send_tokens(destination, amount, "approve"))
  }

  fn send_tokens(to_address: String, amount: Vec<Coin>, action: &str) -> Response {
    Response::new()
        .add_message(BankMsg::Send {
            to_address: to_address.clone().into(),
            amount,
        })
        .add_attribute("action", action)
        .add_attribute("to", to_address)
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner{} => to_binary(&query_owner(deps)?),
        QueryMsg::BlackList { } => to_binary(&query_blacklist(deps)?),
    }
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let config: Config = read_config(deps.storage)?;
    let addr = config.owner;
    Ok(OwnerResponse{ owner: addr })
}

fn query_blacklist(deps: Deps) -> StdResult<BlacklistResponse> {
    let config: Config = read_config(deps.storage)?;
    let list = config.black_list;
    Ok(BlacklistResponse{ list: list})
}

