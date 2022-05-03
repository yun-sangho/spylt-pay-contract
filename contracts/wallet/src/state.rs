use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

pub struct WalletContract<'a> {
    pub config: Item<'a, Config>,
}

impl Default for WalletContract<'static>  {
    fn default() -> Self {
        Self::new(
            "config",
        )
    }
}

impl<'a> WalletContract<'a> {
    fn new(
        config_key: &'a str,
    ) -> Self {
        Self {
            config: Item::new(config_key),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct Config {
    pub owner_addr: Addr,
    pub factory_addr: Addr,
}