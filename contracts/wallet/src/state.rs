use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

pub struct WalletContract<'a> {
    pub balance: Item<'a, Uint128>,
    pub config: Item<'a, Config>,
}

impl Default for WalletContract<'static>  {
    fn default() -> Self {
        Self::new(
            "balance",
            "config",
        )
    }
}

impl<'a> WalletContract<'a> {
    fn new(
        balance_key: &'a str,
        config_key: &'a str,
    ) -> Self {
        Self {
            balance: Item::new(balance_key),
            config: Item::new(config_key),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct Config {
    pub owner_addr: Addr,
    pub factory_addr: Addr,
}