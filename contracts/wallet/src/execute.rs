use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo, Response, to_binary, WasmMsg};
use cw2::set_contract_version;
use splyt::wallet::{InstantiateMsg, WalletCallbackMsg};
use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::{Config, WalletContract};

const CONTRACT_NAME: &str = "crates.io:wallet-factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> WalletContract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let config = Config {
            factory_addr: info.sender.clone(),
            owner_addr: deps.api.addr_validate(&msg.owner)?,
        };

        self.config.save(deps.storage, &config)?;

        Ok(Response::new()
            .add_message(
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr:  config.factory_addr.to_string(),
                    msg: to_binary(
                        &ExecuteMsg::Callback(WalletCallbackMsg::Instantiated {
                            owner: config.owner_addr.to_string(),
                        }))?,
                    funds: Vec::new(),
                })
            )
            .add_attribute("method", "instantiate")
            .add_attribute("owner", config.owner_addr))
    }
}