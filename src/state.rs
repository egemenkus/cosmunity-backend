use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
}

#[cw_serde]
pub struct Group {
    pub id: u64,
    pub group_name: String,
    pub token_amount: Uint128,
}

pub const GROUPS: Map<u64, Group> = Map::new("groups");
pub const ENTRY_SEQ: Item<u64> = Item::new("entry_seq");
pub const CONFIG: Item<Config> = Item::new("config");
