use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
//changing State to Config 
pub struct Config {
    pub admin: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll{
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64)>,
}

pub const CONFIG: Item<Config> = Item::new("config");
