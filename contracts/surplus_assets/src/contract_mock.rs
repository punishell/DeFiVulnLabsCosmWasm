use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Binary, Empty, Response, StdResult, Uint128, CosmosMsg, BankMsg};
use cw_multi_test::{Contract, ContractWrapper};
use cw_storage_plus::Map;


// Simple mocked instantiate with no params so devs can use it easily
#[cw_serde]
pub struct MockInstantiateMsg {}

// Mocked ExecuteMsg with some CW20 related functions, maybe these are needed at all but it gives you a bigger mock to play with.
#[cw_serde]
pub enum MockExecuteMsg {
    Deposit{}
}

// We define a custom struct for each query response
#[cw_serde]
pub enum MockQueryMsg {
    Config {},
}

pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
pub fn contract_pool_mock() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        |_deps, _, _info, msg: MockExecuteMsg| -> StdResult<Response> {
            match msg {  
                MockExecuteMsg::Deposit {} => Ok(Response::new().add_message(CosmosMsg::Bank(BankMsg::Send{ to_address: _info.sender.to_string(), amount: _info.funds }))),
            }
        },
        |_, _, _, _: MockInstantiateMsg| -> StdResult<Response> { Ok(Response::default()) },
        |_, _, msg: MockQueryMsg| -> StdResult<Binary> {
            match msg {
                MockQueryMsg::Config {} => Ok(to_binary("&config_response()")?),
            }
        },
    );
    Box::new(contract)
}

