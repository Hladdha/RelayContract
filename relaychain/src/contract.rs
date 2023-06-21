use cosmwasm_std::{DepsMut, Response, StdResult, Uint128, Uint64, BankMsg, coins, entry_point, ensure};
use cosmwasm_std::{Addr, Env, MessageInfo, StdError, Timestamp, Event, Deps, BlockInfo};
use crate::state::{Claims, PendingClaims, TransactionHashLookup, TotalUnclaimedAmount,LatestBlockNumber};
use crate::types::{Claim};

pub fn request_claim(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    block: u64,
    batch_data: Claim,
) -> StdResult<Response> {
    ensure!(LatestBlockNumber.load(deps.storage).unwrap() < block, );
    LatestBlockNumber.save(deps.storage, &block);
    

    Ok(Response::new())

}