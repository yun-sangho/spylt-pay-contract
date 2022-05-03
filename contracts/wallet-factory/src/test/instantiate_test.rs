use cosmwasm_std::from_binary;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use crate::msg::{InstantiateMsg, QueryMsg};
use crate::state::{Config, WalletFactoryContract};

#[test]
fn instantiate_test() {
    let factory = WalletFactoryContract::default();

    let mut deps = mock_dependencies(&[]);

    let instantiate_msg = InstantiateMsg {
        wallet_contract_code_id: 1,
        wallet_contract_label: String::from("wallet"),
    };

    let info = mock_info("owner", &[]);

    let _res = factory.instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();

    let config: Config = from_binary(
        &factory.query(deps.as_ref(), QueryMsg::Config {}).unwrap()
    ).unwrap();

    assert_eq!(String::from("wallet"), config.wallet_contract_label);
    assert_eq!(1, config.wallet_contract_code_id);
}