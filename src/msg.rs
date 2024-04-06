use crate::state::{ContractStatus, PortfolioConfig};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, ContractInfo, Uint128, Uint256};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub viewing_key: String,
    pub swap_factory: ContractInfo,
    pub withdraw_fee: i32,
    pub create_fee: Uint128,
    pub snip20_code_id: i32,
    pub accepted_deposit_tokens: Option<Vec<ContractInfo>>,
}

#[cw_serde]
pub enum ExecuteMsg {
    // ADMIN
    UpdateConfig {
        admin: Option<Addr>,
        swap_factory: Option<ContractInfo>,
        withdraw_fee: Option<i32>,
        create_fee: Option<i32>,
        snip20_code_id: Option<i32>,
        accepted_deposit_tokens: Option<Vec<ContractInfo>>,
        contract_status: Option<ContractStatus>,
    },
    RegisterAssets {
        assets: Vec<ContractInfo>,
    },
    // Responsible for rebalancing protfolios
    // Will reset UNUPDATED_LIST if found empty
    Update {
        batch_amount: Option<Uint128>,
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
pub enum ReceiveMsg {
    CreatePortfolio {
        config: Vec<PortfolioConfig>,
        name: String,
    },
    Withdraw {},
    Deposit {},
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
