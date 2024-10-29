use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub granter: Addr,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");

pub const GRANTEES: Map<Addr, bool> = Map::new("grantees");
