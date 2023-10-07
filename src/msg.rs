use crate::state::Group;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub enum ExecuteMsg {
    CreateGroup {
        group_name: String,
        token_amount: Uint128,
    },
}

#[cw_serde]
pub struct InitMsg {
    pub owner: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GroupData)]
    GetGroup { id: u64 },
    #[returns(ListResponse)]
    GroupList {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct GroupData {
    pub id: u64,
    pub group_name: String,
    pub token_amount: Uint128,
}

#[cw_serde]
pub struct ListResponse {
    pub groups: Vec<Group>,
}
