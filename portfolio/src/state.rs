use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, ContractInfo, Storage, Uint128};
use secret_toolkit::storage::{Item, Keymap};

pub static CONFIG_KEY: &[u8] = b"config";

#[cw_serde]
pub struct Config {
    pub factory: ContractInfo,
    pub admin: Addr,
    pub accepted_deposit_tokens: Vec<ContractInfo>,
    pub portfolio: Portfolio,
}

#[cw_serde]
pub struct PortfolioConfig {
    pub percent: u128,
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
pub const KEY_FEES: &[u8] = b"fees";
pub const KEY_VIEWING_KEY: &[u8] = b"viewing_key";
pub static CONFIG: Item<Config> = Item::new(KEY_CONFIG);
pub static FEES: Keymap<Addr, Uint128> = Keymap::new(KEY_FEES);
pub static VIEWING_KEY: Item<String> = Item::new(KEY_VIEWING_KEY);
