use cosmwasm_std::{BankMsg, Coin, CosmosMsg, Response, Uint128};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use splyt::wallet::InstantiateMsg;
use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::WalletContract;

#[test]
fn user_can_deposit() {
    let wallet = WalletContract::default();
    let mut deps = mock_dependencies(&[]);
    let info = mock_info("factory", &[]);
    let msg: InstantiateMsg = InstantiateMsg {
        owner: String::from("user1"),
    };
    let _res = wallet.instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let info = mock_info("user", &[Coin {denom: String::from("uluna"), amount: Uint128::from(100000000u128)}]);
    let msg: ExecuteMsg = ExecuteMsg::Deposit;

    let res = wallet.execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(
        Response::new()
            .add_attribute("method", "deposit")
            .add_attribute("amount",  Uint128::from(100000000u128)),
        res)
}

#[test]
fn user_can_withdraw_if_the_user_has_enough_balance() {
    let wallet = WalletContract::default();
    let mut deps = mock_dependencies(&[]);
    let info = mock_info("factory", &[]);
    let msg: InstantiateMsg = InstantiateMsg {
        owner: String::from("user1"),
    };
    let _res = wallet.instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let info = mock_info("user1", &[Coin {denom: String::from("uluna"), amount: Uint128::from(100000000u128)}]);
    let msg: ExecuteMsg = ExecuteMsg::Deposit;
    let _res = wallet.execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let info = mock_info("user1", &[]);
    let msg: ExecuteMsg = ExecuteMsg::Withdraw { amount: Uint128::from(100000u128) };
    let res = wallet.execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(
        Response::new()
            .add_message(CosmosMsg::Bank(BankMsg::Send {
                to_address: String::from("user1"),
                amount: vec![Coin::new(100000u128, "uluna")],
            }))
            .add_attribute("method", "withdraw")
            .add_attribute("amount", Uint128::from(100000u128)),
        res
    )
}

#[test]
fn user_can_not_withdraw_if_the_user_has_not_enough_balance() {
    let wallet = WalletContract::default();
    let mut deps = mock_dependencies(&[]);
    let info = mock_info("factory", &[]);
    let msg: InstantiateMsg = InstantiateMsg {
        owner: String::from("user1"),
    };
    let _res = wallet.instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let info = mock_info("user1", &[Coin {denom: String::from("uluna"), amount: Uint128::from(1000u128)}]);
    let msg: ExecuteMsg = ExecuteMsg::Deposit;
    let _res = wallet.execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let info = mock_info("user1", &[]);
    let msg: ExecuteMsg = ExecuteMsg::Withdraw { amount: Uint128::from(100000u128) };
    let res = wallet.execute(deps.as_mut(), mock_env(), info, msg);

    match res {
        Err(ContractError::NotEnoughBalance {}) => assert!(true),
        _ => panic!("Must return NotEnoughBalance error"),
    }
}