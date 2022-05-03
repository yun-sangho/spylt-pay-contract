use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use splyt::wallet::WalletCallbackMsg;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub wallet_contract_code_id: u64,
    pub wallet_contract_label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateWallet {},
    Callback(WalletCallbackMsg)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config,
    GetWallet { owner: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WalletResponse {
    pub wallet_address: String,
}