use cosmwasm_std::Uint128;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use splyt::wallet::WalletCallbackMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Callback(WalletCallbackMsg),

    Deposit,

    Withdraw { amount: Uint128 },
}
