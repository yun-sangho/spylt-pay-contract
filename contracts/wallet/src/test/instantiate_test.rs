use cosmwasm_std::{CosmosMsg, Response, to_binary, WasmMsg};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use splyt::wallet::{InstantiateMsg, WalletCallbackMsg};
use crate::msg::ExecuteMsg;
use crate::state::WalletContract;

#[test]
fn instantiate_test() {
    let wallet = WalletContract::default();

    let mut deps = mock_dependencies(&[]);
    let info = mock_info("factory", &[]);

    let msg: InstantiateMsg = InstantiateMsg {
        owner: String::from("user1"),
    };

    let res = wallet.instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(
        Response::new()
            .add_message(
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr:  "factory".to_string(),
                    msg: to_binary(
                        &ExecuteMsg::Callback(WalletCallbackMsg::Instantiated {
                            owner: "user1".to_string(),
                        })).unwrap(),
                    funds: vec![],
                })
            )
            .add_attribute("method", "instantiate")
            .add_attribute("owner", "user1"),
        res
    )
}