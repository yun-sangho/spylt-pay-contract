use cosmwasm_std::{Coin, Response, Uint128};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use splyt::wallet::InstantiateMsg;
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

fn user_can_withdraw_if_the_user_has_enough_balance() {

}

fn user_can_not_withdraw_if_the_user_has_not_enough_balance() {

}