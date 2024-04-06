use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, ContractInfo, Uint128, Uint256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::PortfolioConfig;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub swap_factory: ContractInfo,
    pub fee: i32,
    pub snip20_code_id: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    //Receiver interface
    Receive {
        sender: Addr,
        from: Addr,
        amount: Uint256,
        #[serde(default)]
        msg: Option<Binary>,
    },
    Update {
        batch_amount: Uint128,
    },
}

#[cw_serde]
pub enum ReceiveMsg {
    CreatePortfolio { config: Vec<PortfolioConfig> },
    Withdraw {},
    Deposit {},
}

#[cw_serde]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetConfig {},
    GetState {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct CountResponse {
    pub count: i32,
}
