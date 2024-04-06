use crate::state::{ContractStatus, Portfolio, PortfolioConfig};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, ContractInfo, Uint128, Uint256};

#[cw_serde]
pub struct InstantiateMsg {
    pub factory: ContractInfo,
    pub snip20: ContractInfo,
    pub accepted_deposit_tokens: Vec<ContractInfo>,
    pub viewing_key: String,
    pub portfolio: Portfolio,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Responsible for rebalancing portfolio
    // Will reset UNUPDATED_LIST if found empty
    Update {},
    Withdraw {},

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
    GetState {},
    GetUnupdated {},
}

#[cw_serde]
pub enum QueryResponse {}

#[cw_serde]
pub enum ExecuteResponse {}
