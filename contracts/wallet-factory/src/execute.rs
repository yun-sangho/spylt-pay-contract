use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo, Response, to_binary, WasmMsg};
use cw2::set_contract_version;
use splyt::wallet::{WalletCallbackMsg, InstantiateMsg as WalletInstantiateMsg};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Config, WalletFactoryContract};

const CONTRACT_NAME: &str = "crates.io:wallet-factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> WalletFactoryContract<'a>  {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let config = Config {
            wallet_contract_code_id: msg.wallet_contract_code_id,
            wallet_contract_label: msg.wallet_contract_label.clone(),
        };

        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        self.config.save(deps.storage, &config)?;

        Ok(Response::new()
            .add_attribute("method", "instantiate")
            .add_attribute("wallet_contract_code_id", msg.wallet_contract_code_id.to_string())
            .add_attribute("wallet_contract_label", msg.wallet_contract_label))
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::InitWallet {} => self.init_wallet(deps, env, info),
            ExecuteMsg::Callback(callback_msg) => self.handle_wallet_callback(deps, env, info, callback_msg),
        }
    }
}

impl<'a> WalletFactoryContract<'a>  {
    pub fn init_wallet(&self, deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        let config = self.config.load(deps.storage)?;

        Ok(Response::new().add_message(
            CosmosMsg::Wasm(WasmMsg::Instantiate {
                admin: Some(info.sender.to_string()),
                code_id: config.wallet_contract_code_id,
                msg: to_binary(&WalletInstantiateMsg{
                    owner: info.sender.to_string(),
                })?,
                funds: Vec::new(),
                label: config.wallet_contract_label,
            })
        ))
    }

    pub fn handle_wallet_callback(&self, deps: DepsMut, _env: Env, info: MessageInfo, msg: WalletCallbackMsg) -> Result<Response, ContractError> {
        let wallet_contract_addr = info.sender;

        match msg {
            WalletCallbackMsg::Instantiated {owner} => {
                let owner_addr = deps.api.addr_validate(&owner)?;
                self.wallets.save(deps.storage, owner_addr, &wallet_contract_addr)?;

                Ok(Response::new()
                    .add_attribute("method", "instantiated_callback")
                    .add_attribute("owner", owner)
                    .add_attribute("wallet_address", &wallet_contract_addr))
            },
        }
    }
}