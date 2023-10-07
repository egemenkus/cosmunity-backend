use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GroupData, InitMsg, ListResponse, QueryMsg};
use crate::state::{Config, Group, CONFIG, ENTRY_SEQ, GROUPS};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
    Uint128,
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;
use std::ops::Add;

const CONTRACT_NAME: &str = "crates.io:cosmunity";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let owner = msg
        .owner
        .and_then(|addr_string| deps.api.addr_validate(addr_string.as_str()).ok())
        .unwrap_or(info.sender);

    let config = Config {
        owner: owner.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    ENTRY_SEQ.save(deps.storage, &0u64)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateGroup {
            group_name,
            token_amount,
        } => create_group(deps, info, group_name, token_amount),
    }
}

fn create_group(
    deps: DepsMut,
    info: MessageInfo,
    group_name: String,
    token_amount: Uint128,
) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }
    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;
    let new_group = Group {
        id,
        group_name,
        token_amount,
    };
    GROUPS.save(deps.storage, id, &new_group)?;
    Ok(Response::new()
        .add_attribute("method", "create_group")
        .add_attribute("new_group_id", id.to_string()))
}

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetGroup { id } => to_binary(&query_group(deps, id)?),
        QueryMsg::GroupList { start_after, limit } => {
            to_binary(&query_group_list(deps, start_after, limit)?)
        }
    }
}

fn query_group(deps: Deps, id: u64) -> StdResult<GroupData> {
    let group = GROUPS.load(deps.storage, id)?;
    Ok(GroupData {
        id: group.id,
        group_name: group.group_name,
        token_amount: group.token_amount,
    })
}

fn query_group_list(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<ListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    let entries: StdResult<Vec<_>> = GROUPS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .collect();

    let result = ListResponse {
        groups: entries?.into_iter().map(|l| l.1).collect(),
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary, Addr};
    use std::collections::hash_map::Entry;
    use std::vec::Vec;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let msg = InitMsg { owner: None };
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        let state = CONFIG.load(&deps.storage).unwrap();
        assert_eq!(
            state,
            Config {
                owner: Addr::unchecked("creator".to_string()),
            }
        );

        let msg = InitMsg {
            owner: Some("specified_owner".to_string()),
        };
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let state = CONFIG.load(&deps.storage).unwrap();
        assert_eq!(
            state,
            Config {
                owner: Addr::unchecked("specified_owner".to_string()),
            }
        );
    }
    #[test]
    fn create_group() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = InitMsg { owner: None };
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        let msg = ExecuteMsg::CreateGroup {
            group_name: "Cosmunity".to_string(),
            token_amount: Uint128::from(1907u128),
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![attr("method", "create_group"), attr("new_group_id", "1")]
        );

        let res = query(deps.as_ref(), env.clone(), QueryMsg::GetGroup { id: 1 }).unwrap();
        let entry: GroupData = from_binary(&res).unwrap();
        assert_eq!(
            GroupData {
                id: 1,
                group_name: "Cosmunity".to_string(),
                token_amount: Uint128::from(1907u128),
            },
            entry
        );

        let msg = ExecuteMsg::CreateGroup {
            group_name: "Cosmunity Official".to_string(),
            token_amount: Uint128::from(1907u128),
        };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![attr("method", "create_group"), attr("new_group_id", "2")]
        );

        let res = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::GroupList {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
        let list: ListResponse = from_binary(&res).unwrap();
        assert_eq!(
            Vec::from([
                Group {
                    id: 1,
                    group_name: "Cosmunity".to_string(),
                    token_amount: Uint128::from(1907u128)
                },
                Group {
                    id: 2,
                    group_name: "Cosmunity Official".to_string(),
                    token_amount: Uint128::from(1907u128)
                }
            ]),
            list.groups
        );
    }
}
