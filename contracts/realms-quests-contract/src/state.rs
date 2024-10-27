use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

// The Realm state for storing all the realms
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Realm {
    pub id: String,
    pub name: String,
    pub description: String,
    pub reward_nft: Addr,
}

// QuestTypes for storing the different types of quests
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum QuestType {
    CodeCompletion,
}

// Quest state for storing all the quests
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Quest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub quest_type: QuestType,
    pub verification_data: String,
}

pub const STATE: Item<State> = Item::new("state");
pub const REALMS: Map<String, Realm> = Map::new("realms");
pub const QUESTS: Map<String, Quest> = Map::new("quests");
