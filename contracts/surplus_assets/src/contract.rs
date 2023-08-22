
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, WasmMsg, SubMsg, StdError, Reply 
};

use crate::assets::{PriceResponse, OracleQueryMsg};
use crate::error::ContractError;
use crate::msg::{InstantiateMsg, QueryMsg, ExecuteMsg, ConfigResponse};
use crate::state::{Config, CONFIG, POOL };
use cw2::set_contract_version;


// Version info, for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-merkle-airdrop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const DEPOSIT_REPLY_ID: u64 = 0;

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
        allowed_denoms: msg.allowed_denoms
    };
    CONFIG.save(deps.storage, &config)?;
    POOL.save(deps.storage, &deps.api.addr_validate(&msg.pool)?)?;
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
        }
    }

pub fn deposit_funds(
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {


        let config = CONFIG.load(deps.storage)?;
        let pool = POOL.load(deps.storage)?;

        if info.funds.len() != 2 {
            return Err(ContractError::OnlyOneCoin {});
        }
        

        if !config.allowed_denoms.contains(&info.funds[0].denom) || !config.allowed_denoms.contains(&info.funds[1].denom){
            return Err(ContractError::FundsError {});
        }

        Ok(Response::default().add_submessage(SubMsg::reply_on_success(
            WasmMsg::Execute {
                contract_addr: pool.to_string(),
                msg: to_binary(&ExecuteMsg::Deposit { })?,
                funds: info.funds,
            },
            DEPOSIT_REPLY_ID,
        )))

        
     

    
     
    }


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError>{
    match reply.id {
        DEPOSIT_REPLY_ID => Ok(Response::new()),
        id => Err(ContractError::Std(StdError::generic_err(format!(
            "unknown reply id {}",
            id
        )))),
    }
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



