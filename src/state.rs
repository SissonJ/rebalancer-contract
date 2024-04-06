use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, ContractInfo, Storage};
use secret_toolkit::storage::Item;
use secret_toolkit::{serialization::Json, storage::Keymap};

pub static CONFIG_KEY: &[u8] = b"config";

pub enum ContractStatus {
    ACTIVE,    // full funcitonality
    FROZEN,    // no functionality
    PROTECTED, // withdraw functionality only
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub swap_factory: ContractInfo,
    pub withdraw_fee: i32,
    pub create_fee: i32,
    pub snip20_code_id: i32,
    pub accepted_deposit_tokens: Vec<ContractInfo>,
    pub contract_status: ContractStatus,
}

pub struct PortfolioConfig {
    pub percent: u32,
    pub asset: ContractInfo,
}

pub struct Portfolio {
    pub config: Vec<PortfolioConfig>,
    pub creator: Addr,
    pub name: String,
    pub snip20: ContractInfo,
}

pub struct RouteKey(Addr, Addr);

pub struct SwapContract {
    addr: Addr,
    code_hash: String,
}

pub const KEY_CONFIG: &[u8] = b"config";
pub const KEY_PORTFOLIO_LIST: &[u8] = b"portfolio_list";
pub const KEY_UNUPDATED_LIST: &[u8] = b"unupdated_list";
pub const KEY_REGISTERED_ASSETS: &[u8] = b"registered_assets";
pub const KEY_VIEWING_KEY: &[u8] = b"viewing_key";
pub const KEY_PORTFOLIO: &[u8] = b"portfolio";
pub const KEY_ROUTE_CACHE: &[u8] = b"route_cache";
pub static CONFIG: Item<Config> = Item::new(KEY_CONFIG);
// List of all known portfolios
pub static PORTFOLIO_LIST: Item<Vec<Addr>> = Item::new(KEY_PORTFOLIO_LIST);
// List of all portfolios pending update
pub static UNUPDATED_LIST: Item<Vec<Addr>> = Item::new(KEY_UNUPDATED_LIST);
pub static REGISTERED_ASSTS: Item<Vec<Addr>> = Item::new(KEY_REGISTERED_ASSETS);
pub static VIEWING_KEY: Item<String> = Item::new(KEY_VIEWING_KEY);
// Hash map of snip20 protfolio token and the portfolio information
pub static PORTFOLIO: Keymap<Addr, Portfolio> = Keymap::new(KEY_PORTFOLIO);
pub static ROUTE_CACHE: Keymap<RouteKey, Vec<SwapContract>> = Keymap::new(KEY_ROUTE_CACHE);
