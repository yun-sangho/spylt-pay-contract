use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use serde::{Serialize, Deserialize};
use schemars::{JsonSchema};

pub struct WalletFactoryContract<'a> {
    pub config: Item<'a, Config>,
    pub wallets: Map<'a, Addr, Addr>,
}

impl Default for WalletFactoryContract<'static> {
    fn default() -> Self {
        Self::new(
            "config",
            "wallets",
        )
    }
}

impl <'a> WalletFactoryContract<'a> {
    fn new(
        config_key: &'a str,
        wallets_key: &'a str,
    ) -> Self {
        Self {
            config: Item::new(config_key),
            wallets: Map::new(wallets_key),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct Config {
    pub wallet_contract_code_id: u64,
    pub wallet_contract_label: String,
}