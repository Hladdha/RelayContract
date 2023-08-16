use cw_storage_plus::{Item, Map};
use cosmwasm_std::Addr; 
use crate::types::{ClaimData, PendingClaim};


pub const Claims: Map<Addr, Vec<ClaimData>> = Map::new("Claims");
pub const PendingClaims: Map<String, PendingClaim> = Map::new("PendingClaims");
pub const TransactionHashLookup: Map<String, u8> = Map::new("TransactionHashLookup");
pub const TotalUnclaimedAmount: Item<u32> = Item::new("TotalUnclaimedAmount");
pub const LatestBlockNumber: Item<u64> = Item::new("LatestBlockNumber");

