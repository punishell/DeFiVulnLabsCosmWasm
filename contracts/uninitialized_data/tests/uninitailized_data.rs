use cosmwasm_std::Addr;
use cw_unsaved_init_values::contract::{instantiate, execute, query};
use cw_unsaved_init_values::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, MarketsResponse};
use cw_multi_test::{App, ContractWrapper, Executor};

#[test]
    fn uninitialized_data_test() {
        let mut app = App::default();
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));
        let contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { owner: "owner".to_owned(), markets: vec!["some_addr".to_string()] },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        
        let update_response = app.execute_contract(Addr::unchecked("owner"), contract_addr.clone(), &ExecuteMsg::UpdateMarkets { market: "some_market".to_owned() }, &[]);
        println!("{:?}",update_response.unwrap());
   
        }
        