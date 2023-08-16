
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, CosmosMsg, WasmMsg, Uint128, coin, 
};

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, QueryMsg, ExecuteMsg, ConfigResponse, MarketsResponse};
use crate::state::{Config, CONFIG, MARKETS };
use cw2::set_contract_version;


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

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: deps.api.addr_validate(&msg.owner)?,
    };
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
        ExecuteMsg::Deposit {} =>  deposit_funds(deps, info),
        ExecuteMsg::UpdateMarkets {market} =>  update_markets(deps, info, market),
        }
    }

pub fn deposit_funds(
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        if info.funds.len() != 1 {
            return Err(ContractError::OnlyOneCoin {});
        }
        let markets = MARKETS.load(deps.storage)?; 
        let mut response = Response::new();
        let amount = info.funds[0].amount/Uint128::from( markets.len() as u128);
        let denom = &info.funds[0].denom;

        for market in markets{
            response = response.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: market,
                msg: to_binary(&"SOME_MSG")?,
                funds: vec![coin(amount.into(), denom)],
            }));
        };
        Ok(response)
    }

    pub fn update_markets(
        deps: DepsMut,
        info: MessageInfo,
        market: String
    ) -> Result<Response, ContractError> {
        if info.funds.len() != 0 {
            return Err(ContractError::FundsError{});
        }
        MARKETS.update(deps.storage, |mut markets| -> StdResult<Vec<String>>
        {
            markets.push(market);
            Ok(markets)
        }
        )?;

        Ok(Response::new())
    }
    

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config{} => to_binary(&query_config(deps)?),
        QueryMsg::Markets{} => to_binary(&query_markets(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let addr = config.owner;
    Ok(ConfigResponse{ owner: addr })
}

fn query_markets(deps: Deps) -> StdResult<MarketsResponse> {
    let markets = MARKETS.load(deps.storage)?;
    Ok(MarketsResponse { markets: markets})
}


