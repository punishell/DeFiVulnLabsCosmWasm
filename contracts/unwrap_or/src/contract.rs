
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult 
};

use crate::assets::{PriceResponse, OracleQueryMsg};
use crate::error::ContractError;
use crate::msg::{InstantiateMsg, QueryMsg, ExecuteMsg, ConfigResponse};
use crate::state::{Config, CONFIG, ORACLE };
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
    ORACLE.save(deps.storage, &deps.api.addr_validate(&msg.oracle)?)?;
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
        ExecuteMsg::Deposit {denom} =>  deposit_funds(deps, info, denom),
        }
    }

pub fn deposit_funds(
        deps: DepsMut,
        info: MessageInfo,
        denom: String,
    ) -> Result<Response, ContractError> {
        if info.funds.len() != 1 {
            return Err(ContractError::OnlyOneCoin {});
        }
        if info.funds[0].denom != denom {
            return Err(ContractError::FundsError {});
        }
     

        let usd_price = deps.querier
                .query_wasm_smart::<PriceResponse>(
                    ORACLE.load(deps.storage)?,
                    &OracleQueryMsg::Price { denom: denom },
                )
                .unwrap_or_default()
                .price
                .amount;
        let price_in_usd = info.funds[0].amount * usd_price;
        //rest of the code
             
       
        let response = Response::new();
        Ok(response.add_attribute("calculated value", price_in_usd))
    }



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config{} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let addr = config.owner;
    Ok(ConfigResponse{ owner: addr })
}



