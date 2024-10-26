use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::{Quest, Realm};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
    pub realms: Vec<Realm>,
    pub quests: Vec<Quest>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    AddRealm { realm: Realm },
    AddQuest { quest: Quest },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
    #[returns(Realm)]
    GetRealm { id: String },
    #[returns(Quest)]
    GetQuest { id: String },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
