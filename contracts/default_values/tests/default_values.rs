use cosmwasm_std::{Addr, Coin, coin, Querier};
use cosmwasm_std::{QueryRequest, BankQuery, to_binary, from_binary, BalanceResponse};
use cw_default_values::contract::{execute, instantiate, query};
use cw_default_values::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, OwnerResponse};
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

#[test]
    fn default_values_test() {
        let mut app = App::default();
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));
        // The contract has no real life functionality
        let contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { owner: None },
                &[],
                "Contract",
                None,
            )
            .unwrap();

            let msg = QueryMsg::Owner {  };
            // the contract was deployed with default values 
            let owner_response : OwnerResponse = app.wrap().query_wasm_smart(contract_addr.clone(), &msg).unwrap();
            println!("{:?}", owner_response);
            mint_native(&mut app, contract_addr.to_string(), "ATOM".to_string(), 100);
            //this should fail
            let withdraw_res = app.execute_contract(Addr::unchecked("owner"), contract_addr.clone(), &ExecuteMsg::Withdraw{destination: "attacker".to_string()}, &[]);
            println!("{:?}", withdraw_res.as_ref());
            assert!(withdraw_res.is_err());
        }
        
