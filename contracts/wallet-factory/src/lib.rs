mod msg;
mod state;
mod query;
mod execute;
mod error;

#[cfg(test)]
mod test;

#[cfg(not(feature = "library"))]
pub mod entry {
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use crate::state::WalletFactoryContract;

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let contract = WalletFactoryContract::default();
        contract.instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let contract = WalletFactoryContract::default();
        contract.execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(
        deps: Deps,
        _env: Env,
        msg: QueryMsg,
    ) -> StdResult<Binary> {
        let contract = WalletFactoryContract::default();
        contract.query(deps, msg)
    }
}