use cosmwasm_std::{DepsMut, Response, StdResult, BankMsg, coins, entry_point, ensure};
use cosmwasm_std::{MessageInfo, StdError, Addr};
use crate::state::{Claims, PendingClaims, TransactionHashLookup, TotalUnclaimedAmount, LatestBlockNumber};
use crate::types::{Claim, PendingClaim, ClaimData};
use crate::error::ContractError;
use sha3::{Digest, Keccak256};


const  PREFIX: &'static [u8] = b"Pay ENJ2 to the account:";

pub fn request_claim(
    deps: DepsMut,
    block: u64,
    batch_data: Vec<Claim>,
) -> Result<Response, ContractError> {
    ensure!(LatestBlockNumber.load(deps.storage).unwrap() < block, ContractError::WrongTimeStamp {});
    LatestBlockNumber.save(deps.storage, &block)?;
    for claim in batch_data.iter() {
        let amount = claim.amount;
        let account = claim.account.clone();
        ensure!(!(amount == 0), ContractError::ZeroAmount {});
        ensure!(!(TransactionHashLookup.load(deps.storage, claim.tx_hash.clone())? == 1), ContractError::HashAlreadyExists {});
        let data = PendingClaim {
            account,
            amount,
            is_early_bird : claim.is_early_bird,
            is_efi: claim.is_efi,
            start_block_number: block,
        };
        PendingClaims.save(deps.storage, claim.tx_hash.clone(), &data)?;
        TransactionHashLookup.save(deps.storage, claim.tx_hash.clone(), &1)?;
    }
    Ok(Response::new())
}

pub fn approve_claims(
    deps: DepsMut,
    approve: Vec<String>,
) -> Result<Response, ContractError> {
    for string in approve.iter(){
        let mut sum: u32 = 0;
        let pending_data = PendingClaims.load(deps.storage, string.to_string())?;
        let mut amount = pending_data.amount;
		let start_block_number = pending_data.start_block_number;
        let is_early_bird = pending_data.is_early_bird;
        if pending_data.is_efi {
            amount = 1_000_000_000u32*amount;
        }
        let data = ClaimData {
            tx_hash: Some(string.to_string()),
            amount,
            start_block_number,
            is_early_bird,
        };
        Claims.update(deps.storage, pending_data.account, |x: Option<Vec<ClaimData>>|-> StdResult<Vec<ClaimData>>{
            match x {
                Some(one) => Ok([one, vec![data]].concat()),
                None => Ok(vec![data]),
            }
        })?;
        sum = sum
			.checked_add(amount)
			.ok_or(ContractError::AritheMaticOverflow)?;
		if !(sum == 0) {
			TotalUnclaimedAmount.update(deps.storage, |t: u32| -> StdResult<u32>  {
				Ok(t + sum)
			})?;
		}
    }
    Ok(Response::new())
}

pub fn mint_claim (
    deps: DepsMut,
    address: Addr,
    amount: u32,
) -> Result<Response, ContractError> {
    ensure!(!(amount == 0), ContractError::ZeroAmount {});
    TotalUnclaimedAmount.update(deps.storage, |t: u32| -> StdResult<u32>  {
        Ok(t + amount)
    })?;
    let block  = LatestBlockNumber.load(deps.storage)?;
    let data = ClaimData {
        tx_hash: None,
        amount,
        start_block_number : block, 
        is_early_bird : false,
    };
    Claims.update(deps.storage, address, |x: Option<Vec<ClaimData>>|-> StdResult<Vec<ClaimData>>{
        match x {
            Some(one) => Ok([one, vec![data]].concat()),
            None => Ok(vec![data]),
        } 
    })?;
    Ok(Response::new())
}

pub fn claim (
    deps: DepsMut,
    _info: MessageInfo,
    signature: [u8; 65],
) -> Result<Response , ContractError> {
    let dest = _info.sender;
    let data = dest.;

    let claims = Claims.load(deps.storage, _info.sender)?;
    let mut amount: u32  =  0;
    for claim_data in claims.iter(){
        amount += claim_data.amount;
        if claim_data.is_early_bird{
            amount = amount
                .checked_mul(2)
                .ok_or(ContractError::AritheMaticOverflow)?;
        }
    }
    Ok(Response::new())
}

fn to_ascii_hex(data: &[u8]) -> Vec<u8> {
	let mut r = Vec::with_capacity(data.len() * 2);
	let mut push_nibble = |n| r.push(if n < 10 { b'0' + n } else { b'a' - 10 + n });
	for &b in data.iter() {
		push_nibble(b / 16);
		push_nibble(b % 16);
	}
	r
}

fn ethereum_signable_message(what: &[u8], extra: &[u8]) -> Vec<u8> {
    let mut l = PREFIX.len() + what.len() + extra.len();
    let mut rev = Vec::with_capacity(21);
    while l > 0 {
        rev.push(b'0' + (l % 10) as u8);
        l /= 10;
    }
    let v_pref = b"\x19Ethereum Signed Message:\n";
    let mut v = Vec::with_capacity(
        v_pref.len() + rev.len() + PREFIX.len() + what.len() + extra.len(),
    );
    v.extend_from_slice(v_pref);
    v.extend(rev.into_iter().rev());
    v.extend_from_slice(PREFIX);
    v.extend_from_slice(what);
    v.extend_from_slice(extra);
    v
}

fn eth_recover(s: &[u8; 65], what: &[u8], extra: &[u8]) -> Option<String> {
    let mut hasher = Keccak256::new();
    hasher.update(ethereum_signable_message(what, extra));
    let msg = hasher.finalize();

    let mut res = String::new();
    
    Some(res)
}

