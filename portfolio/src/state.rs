use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, ContractInfo, Storage, Uint128};
use secret_toolkit::storage::Item;
use secret_toolkit::{serialization::Json, storage::Keymap};

pub static CONFIG_KEY: &[u8] = b"config";

#[cw_serde]
pub enum ContractStatus {
    ACTIVE,    // full funcitonality
    FROZEN,    // no functionality
    PROTECTED, // withdraw functionality only
}

#[cw_serde]
pub struct Config {
    pub factory: ContractInfo,
    pub snip20: ContractInfo,
    pub accepted_deposit_tokens: Vec<ContractInfo>,
    pub portfolio: Portfolio,
}

#[cw_serde]
pub struct PortfolioConfig {
    pub percent: u32,
    pub asset: ContractInfo,
}

#[cw_serde]
pub struct Portfolio {
    pub config: Vec<PortfolioConfig>,
    pub creator: Addr,
    pub name: String,
    pub snip20: ContractInfo,
}

pub const KEY_CONFIG: &[u8] = b"config";
pub const KEY_REGISTERED_ASSETS: &[u8] = b"registered_assets";
pub const KEY_VIEWING_KEY: &[u8] = b"viewing_key";
pub static CONFIG: Item<Config> = Item::new(KEY_CONFIG);
pub static REGISTERED_ASSETS: Map<Addr, Snip20Asset> = Map::new(KEY_REGISTERED_ASSETS);
pub static VIEWING_KEY: Item<String> = Item::new(KEY_VIEWING_KEY);
