use cosmwasm_std::{Addr, coin};
use cw_unwrap_or::contract::{instantiate, execute, query};
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


#[test]
    fn uninitialized_data_test() {
        let mut app = App::default();
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));
        let contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { owner: "owner".to_owned(), oracle:"oracle".to_owned() },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        mint_native(&mut app, "user".to_string(), "uatom".to_string(), 100);
     
        let update_response = app.execute_contract(Addr::unchecked("user"), contract_addr.clone(), &ExecuteMsg::Deposit { denom: "uatom".to_owned() }, &[coin(100, "uatom")]);
        assert_eq!(update_response.unwrap().events[1].attributes[1].value, "0");
   
        }
        