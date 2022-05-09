use std::ops::{Add, Sub};
use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, Uint128, WasmMsg};
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
        self.balance.save(deps.storage, &Uint128::zero())?;

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

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Deposit {} => self.deposit(deps, env, info),
            ExecuteMsg::Withdraw { amount } => self.withdraw(deps, env, info, amount),
            _ => Err(ContractError::Unauthorized {}),
        }
    }
}

impl<'a> WalletContract<'a>  {
    pub fn deposit(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let amount = info
            .funds.iter()
            .find(|c| c.denom == "uluna")
            .map(|c| c.amount)
            .unwrap_or_else(Uint128::zero);

        self.balance.update(deps.storage, |c| -> StdResult<_> {
            Ok(c.add(amount))
        })?;

        Ok(Response::new()
            .add_attribute("method", "deposit")
            .add_attribute("amount", amount))
    }

    pub fn withdraw(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        let config = self.config.load(deps.storage)?;
        if config.owner_addr != info.sender {
            return Err(ContractError::Unauthorized {});
        };

        let balance = self.balance.load(deps.storage)?;
        if amount >= balance {
            return Err(ContractError::NotEnoughBalance {});
        };

        let remain = balance.sub(amount);

        self.balance.save(deps.storage, &remain)?;

        Ok(Response::new()
            .add_message(CosmosMsg::Bank(BankMsg::Send {
                to_address: config.owner_addr.to_string(),
                amount: vec![Coin::new(amount.u128(), "uluna")],
            }))
            .add_attribute("method", "withdraw")
            .add_attribute("amount", amount))
    }
}