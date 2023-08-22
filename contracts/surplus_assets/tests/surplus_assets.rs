use cosmwasm_std::{Addr, coin, Response, Coin, QueryRequest, BankQuery, BalanceResponse, to_binary, Querier, from_binary};
use cw_unwrap_or::contract::{instantiate, execute, query, reply};
use cw_unwrap_or::contract_mock::{contract_pool_mock, MockInstantiateMsg};
use cw_unwrap_or::msg::{InstantiateMsg, ExecuteMsg};
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
    fn surplus_assets_test() {
        let mut app = App::default();

        let pool_code = contract_pool_mock();
        let pool_code_id = app.store_code(pool_code);
        let pool_addr = app.instantiate_contract(pool_code_id, Addr::unchecked("owner"), &MockInstantiateMsg{}, &[], "pool", None).unwrap();
        
        let code = ContractWrapper::new(execute, instantiate, query);
        let code = code.with_reply(reply);
        let code_id = app.store_code(Box::new(code));
        let contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { owner: "owner".to_owned(),  pool: pool_addr.to_string(), allowed_denoms: vec!["uatom".to_string(),"uosmo".to_string()] },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        mint_native(&mut app, "user".to_string(), "uatom".to_string(), 100);
        mint_native(&mut app, "user".to_string(), "uosmo".to_string(), 100);

        let deposit_response = app.execute_contract(Addr::unchecked("user"), contract_addr.clone(), &ExecuteMsg::Deposit { }, &[coin(100, "uatom"),coin(100, "uosmo")]);
        print!("{:?}",deposit_response);
        
        //The mock contract sends back assets to info.sender, this could happen in pool rebase etc.
        assert_eq!(coin(100u128,"uosmo"), query_balance_native(&app,&contract_addr,"uosmo"));
   
        }
        