use crate::state::{Config, Portfolio, PortfolioConfig};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, ContractInfo, Uint128, Uint256};
use rebalancer_factory::state::SwapContract;

#[cw_serde]
pub struct InstantiateMsg {
    pub factory: ContractInfo,
    pub accepted_deposit_tokens: Vec<ContractInfo>,
    pub viewing_key: String,
    pub portfolio: Portfolio,
    pub admin: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    Update {
        tolerance_percent: u128,
    },
    UpdateKey {
        viewing_key: String,
    },
    Withdraw {
        share: Uint128,
        receiver: Addr,
        fee: u128,
    },

    //Receiver interface
    Receive {
        sender: Addr,
        from: Addr,
        amount: Uint256,
        #[serde(default)]
        msg: Option<Binary>,
    },
}

#[cw_serde]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetConfig {},
    GetFees {},
    GetBalances {},
}

#[cw_serde]
pub struct Fee {
    pub asset: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct BalanceItem {
    pub asset: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub enum QueryAnswer {
    GetConfig { config: Config },
    GetFees { fees: Vec<Fee> },
    GetBalances { balances: Vec<BalanceItem> },
}

#[cw_serde]
pub enum ExecuteAnswer {
    Withdraw {
        withdraw_assets: Vec<WithdrawAction>,
    },
    Update {
        actions: Vec<UpdateAction>,
    },
}

#[cw_serde]
pub struct UpdateAction {
    pub from_asset: ContractInfo,
    pub to_asset: ContractInfo,
    pub sell_amount: Uint256,
    pub expected_return: Uint256,
}

#[cw_serde]
pub struct WithdrawAction {
    pub snip20_addr: Addr,
    pub amount: Uint128,
}

pub struct PositionDetails {
    pub position: PortfolioConfig,
    pub value: Uint256,
    pub price: Uint128,
}

pub struct PositionCorrection {
    pub position: PortfolioConfig,
    pub correction: Uint256,
    pub price: Uint128,
}

#[cw_serde]
pub struct SwapTokensForExact {
    pub expected_return: Uint128,
    pub path: Vec<SwapContract>,
}

#[cw_serde]
pub struct RouterMsg {
    pub swap_tokens_for_exact: SwapTokensForExact,
}
