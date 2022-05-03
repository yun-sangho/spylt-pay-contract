use cosmwasm_std::{Binary, Deps, StdResult, to_binary};
use crate::msg::QueryMsg;
use crate::state::WalletFactoryContract;

impl<'a> WalletFactoryContract<'a> {
    pub fn query(&self, deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Config => to_binary(&self.config.load(deps.storage)?),
            QueryMsg::GetWallet { owner } => to_binary(&self.wallets.may_load(deps.storage, deps.api.addr_validate(&owner)?)?)
        }
    }
}