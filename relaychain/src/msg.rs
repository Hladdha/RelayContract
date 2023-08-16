use cosmwasm_std::Addr;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum QueryMsg {
    PendingClaims {},
}