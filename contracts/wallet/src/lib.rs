mod state;
mod msg;
mod error;
mod execute;

#[cfg(test)]
mod test;

#[cfg(not(feature = "library"))]
pub mod entry {
    use cosmwasm_std::{DepsMut, entry_point, Env, MessageInfo, Response};
    use splyt::wallet::InstantiateMsg;
    use crate::error::ContractError;
    use crate::state::WalletContract;

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let contract = WalletContract::default();
        contract.instantiate(deps, env, info, msg)
    }
}