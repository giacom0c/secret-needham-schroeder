use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::UserInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Increment {},
    Register { s_key: String }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
    Search { address: HumanAddr}
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

/// success or failure response
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum ResponseStatus {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    /// AuctionInfo query response
    GetCount {
        count: i32
    },
    Search {
        contact: UserInfo
        //stringa: String,
        //booleano: bool
    }
}