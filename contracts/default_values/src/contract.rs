use cosmwasm_std::{
    entry_point, to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, OwnerResponse};
use crate::state::{Config, CONFIG};
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
    let config = Config {
        // address is not validated
        // usage of default values
        owner: Addr::unchecked(&msg.owner.unwrap_or_default()),
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(deps.storage, &config)?;
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
        ExecuteMsg::UpdateConfig {owner } => execute_update(deps, env, info, owner),
    }
}
fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination: String,
  ) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != Addr::unchecked(&config.owner.to_string()) {
        return Err(ContractError::Unauthorized {});
      };
    
    let destination = deps.api.addr_validate(destination.as_str())?;
    let contract_address = env.contract.address;
    let amount = deps.querier.query_all_balances(&contract_address)?;
    Ok(send_tokens(destination, amount, "approve"))
  }

  fn execute_update(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
  ) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    if info.sender != Addr::unchecked(&config.owner.to_string()) {
        return Err(ContractError::Unauthorized {});
      };
    
      let new_config = Config {
        owner: deps.api.addr_validate(&owner)?,
    };
    CONFIG.save(deps.storage, &new_config)?;
    let resp= Response::new()
    .add_attribute("action", "UpdateConfig")
    .add_attribute("Owner", owner.to_string());
    Ok(resp)
  }

// this is a helper to move the tokens, so the business logic is easy to read
fn send_tokens(to_address: Addr, amount: Vec<Coin>, action: &str) -> Response {
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
    }
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let config = CONFIG.load(deps.storage)?;
    let addr = config.owner;
    Ok(OwnerResponse{ owner: addr })
}
