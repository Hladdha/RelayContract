use cw_storage_plus::{Item, Map};
use cosmwasm_std::Addr; 
use crate::types::{Claim, PendingClaim};


pub const Claims: Map<Addr, Vec<Claim>> = Map::new("Claims");
pub const PendingClaims: Map<String, PendingClaim> = Map::new("PendingClaims");
pub const TransactionHashLookup: Item<String> = Item::new("TransactionHashLookup");
pub const TotalUnclaimedAmount: Item<u32> = Item::new("TotalUnclaimedAmount");
pub const LatestBlockNumber: Item<u64> = Item::new("LatestBlockNumber");
