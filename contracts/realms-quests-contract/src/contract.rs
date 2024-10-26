#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, Realm, Quest, STATE, REALMS, QUESTS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmic-quest";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    for realm in msg.realms {
        REALMS.save(deps.storage, realm.id.clone(), &realm)?;
    }

    for quest in msg.quests {
        QUESTS.save(deps.storage, quest.id.clone(), &quest)?;
    }

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => execute::increment(deps),
        ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
        ExecuteMsg::AddRealm { realm } => try_add_realm(deps, realm),
        ExecuteMsg::AddQuest { quest } => try_add_quest(deps, quest),
    
    }
}

pub fn try_add_realm(deps: DepsMut, realm: Realm) -> Result<Response, ContractError> {
    REALMS.save(deps.storage, realm.id.clone(), &realm)?;
    Ok(Response::new().add_attribute("method", "add_realm").add_attribute("realm_id", realm.id))
}

pub fn try_add_quest(deps: DepsMut, quest: Quest) -> Result<Response, ContractError> {
    QUESTS.save(deps.storage, quest.id.clone(), &quest)?;
    Ok(Response::new().add_attribute("method", "add_quest").add_attribute("quest_id", quest.id))
}

pub mod execute {
    use super::*;

    pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count += 1;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }

    pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.count = count;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_json_binary(&query::count(deps)?),
        QueryMsg::GetRealm { id } => to_json_binary(&query::realm(deps, id)?),
        QueryMsg::GetQuest { id } => to_json_binary(&query::quest(deps, id)?),
    
    }
}

pub mod query {
    use super::*;

    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count })
    }

    pub fn realm(deps: Deps, id: String) -> StdResult<Realm> {
        let realm = REALMS.load(deps.storage, id)?;
        Ok(realm)
    }

    pub fn quest(deps: Deps, id: String) -> StdResult<Quest> {
        let quest = QUESTS.load(deps.storage, id)?;
        Ok(quest)
    }
}

#[cfg(test)]
mod tests {
    use crate::state::QuestType;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, from_json};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { 
            count: 17,
            realms: vec![], // Provide appropriate values here
            quests: vec![], // Provide appropriate values here
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { 
            count: 17,
            realms: vec![], // Provide appropriate values here
            quests: vec![], // Provide appropriate values here
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { 
            count: 17,
            realms: vec![], // Provide appropriate values here
            quests: vec![], // Provide appropriate values here
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(5, value.count);
    }

    #[test]
    fn test_query_quest() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        // Initialize the contract with some realms and quests
        let msg = InstantiateMsg {
            count: 0,
            realms: vec![],
            quests: vec![
                Quest {
                    id: "quest1".to_string(),
                    name: "Quest 1".to_string(),
                    description: "First quest".to_string(),
                    quest_type: QuestType::CodeCompletion,
                    verification_data: "data1".to_string(),
                },
                Quest {
                    id: "quest2".to_string(),
                    name: "Quest 2".to_string(),
                    description: "Second quest".to_string(),
                    quest_type: QuestType::CodeCompletion,
                    verification_data: "data2".to_string(),
                },
            ],
        };

        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Query the first quest
        let query_msg = QueryMsg::GetQuest { id: "quest1".to_string() };
        let binary_response = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let quest: Quest = from_json(&binary_response).unwrap();

        assert_eq!(quest.id, "quest1");
        assert_eq!(quest.name, "Quest 1");
        assert_eq!(quest.description, "First quest");
        assert_eq!(quest.quest_type, QuestType::CodeCompletion);
        assert_eq!(quest.verification_data, "data1");

        // Query the second quest
        let query_msg = QueryMsg::GetQuest { id: "quest2".to_string() };
        let binary_response = query(deps.as_ref(), env, query_msg).unwrap();
        let quest: Quest = from_json(&binary_response).unwrap();

        assert_eq!(quest.id, "quest2");
        assert_eq!(quest.name, "Quest 2");
        assert_eq!(quest.description, "Second quest");
        assert_eq!(quest.quest_type, QuestType::CodeCompletion);
        assert_eq!(quest.verification_data, "data2");
    }
    
}
