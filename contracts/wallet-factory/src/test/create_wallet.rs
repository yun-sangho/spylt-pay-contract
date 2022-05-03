use cosmwasm_std::{CosmosMsg, from_binary, Response, to_binary, WasmMsg};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use splyt::wallet::{InstantiateMsg as WalletInstantiateMsg, WalletCallbackMsg};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::WalletFactoryContract;

#[test]
fn send_wallet_instantiate_msg() {
    let factory = WalletFactoryContract::default();

    let mut deps = mock_dependencies(&[]);

    let instantiate_msg = InstantiateMsg {
        wallet_contract_code_id: 1,
        wallet_contract_label: String::from("wallet"),
    };

    let info = mock_info("user1", &[]);

    let _res = factory.instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

    let msg = ExecuteMsg::CreateWallet {};

    let res = factory.execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(Response::new().add_message(
        CosmosMsg::Wasm(WasmMsg::Instantiate {
            admin: Some(String::from("user1")),
            code_id: 1,
            msg: to_binary( &WalletInstantiateMsg {
                owner: String::from("user1"),
            }).unwrap(),
            funds: Vec::new(),
            label: String::from("wallet"),
        })
    ), res)
}

#[test]
fn handle_callback() {
    let factory = WalletFactoryContract::default();

    let mut deps = mock_dependencies(&[]);

    let instantiate_msg = InstantiateMsg {
        wallet_contract_code_id: 1,
        wallet_contract_label: String::from("wallet"),
    };

    let info = mock_info("user1", &[]);
    let _res = factory.instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

    // beneficiary can release it
    let info = mock_info("wallet_1", &[]);
    let msg = ExecuteMsg::Callback(WalletCallbackMsg::Instantiated {
        owner: String::from("user1"),
    });

    let res = factory.execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(Response::new()
                   .add_attribute("method", "instantiated_callback")
                   .add_attribute("owner", String::from("user1"))
                   .add_attribute("wallet_address", String::from("wallet_1")
                   ), res);

    let res = factory.query(deps.as_ref(), QueryMsg::GetWallet { owner: String::from("user1")}).unwrap();
    let value: String = from_binary(&res).unwrap();
    assert_eq!(String::from("wallet_1"), value);

    let res = factory.query(deps.as_ref(), QueryMsg::GetWallet { owner: String::from("user2")}).unwrap();
    let value: Option<String> = from_binary(&res).unwrap();
    assert_eq!(None, value);
}