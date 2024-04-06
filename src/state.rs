use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, ContractInfo, Storage};
use secret_toolkit::storage::Item;
use secret_toolkit::{serialization::Json, storage::Keymap};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub swap_factory: ContractInfo,
    pub fee: i32,
    pub viewing_key: String,
    pub snip20_code_id: i32,
}

pub struct PortfolioConfig {
    pub percent: u32,
    pub asset: ContractInfo,
}

pub struct Portfolio {
    pub config: Vec<PortfolioConfig>,
    pub creator: Addr,
}

pub const KEY_CONFIG: &[u8] = b"config";
pub const KEY_PORTFOLIO: &[u8] = b"portfolio";
pub static CONFIG: Item<Config> = Item::new(KEY_CONFIG);
// Hash map of snip20 protfolio token and the portfolio information
pub static PORTFOLIO: Keymap<Addr, Portfolio> = Keymap::new(KEY_PORTFOLIO);
