use crate::state::{ContractStatus, PortfolioConfig, RouteKey, SwapContract};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, Binary, ContractInfo, DepsMut, StdError, StdResult, Uint128, Uint256,
};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub viewing_key: String,
    pub swap_factory: ContractInfo,
    pub withdraw_fee: Uint128,
    pub create_fee: Uint128,
    pub snip20_code_id: i32,
    pub portfolio_code_id: i32,
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
    Deposit {
        portfolio_snip20: Addr,
    },
}

#[cw_serde]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetConfig {},
    GetState {},
    GetUnupdated {},
    Prices { assets: Vec<Addr>, key: String },
    Route { route: RouteKey, key: String },
}

#[cw_serde]
pub struct Price {
    pub asset: Addr,
    pub price: Uint128,
}

#[cw_serde]
pub struct Route {
    pub key: RouteKey,
    pub route: Vec<SwapContract>,
    pub router_contract: ContractInfo,
}

#[cw_serde]
pub enum QueryAnswer {
    Prices { prices: Vec<Price> },
    Route { route: Route },
}

#[cw_serde]
pub enum ExecuteResponse {}

pub fn query_prices(
    deps: &DepsMut,
    contract: ContractInfo,
    assets: Vec<Addr>,
    key: String,
) -> Result<Vec<Price>, StdError> {
    match deps.querier.query(&cosmwasm_std::QueryRequest::Wasm(
        cosmwasm_std::WasmQuery::Smart {
            contract_addr: contract.address.into_string(),
            code_hash: contract.code_hash,
            msg: to_binary(&QueryMsg::Prices { assets, key })?,
        },
    ))? {
        QueryAnswer::Prices { prices } => Ok(prices),
        _ => Err(StdError::generic_err("Query prices error")),
    }
}

pub fn query_route(
    deps: &DepsMut,
    contract: ContractInfo,
    route: RouteKey,
    key: String,
) -> Result<Route, StdError> {
    match deps.querier.query(&cosmwasm_std::QueryRequest::Wasm(
        cosmwasm_std::WasmQuery::Smart {
            contract_addr: contract.address.into_string(),
            code_hash: contract.code_hash,
            msg: to_binary(&QueryMsg::Route { route, key })?,
        },
    ))? {
        QueryAnswer::Route { route } => Ok(route),
        _ => Err(StdError::generic_err("Query route error")),
    }
}
