use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};


/// PendingData isstruct to represent pending hashes tha has to be approved before claiming
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct PendingClaim {
	pub account: Addr,
	/// The amount and balance being claimed
	pub amount: u32,
    /// is eligible for earlybird reward
    pub is_early_bird : bool,

    pub is_efi : bool,
	/// time when claim request is made
	pub start_block_number: u32,
}

/// A claim from of ENJ2 from Ethereum
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Claim {
	pub account: Addr,
    /// hash of the transaction 
    pub tx_hash: String,
	/// The amount and balance being claimed
	pub amount: u32,

    pub is_efi : bool,
    /// is eligible for earlybird reward
    pub is_early_bird : bool,
}


/// Struct to store claims data for users
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct ClaimData {
	/// hash of burn transaction
	pub tx_hash: Option<String>,
	/// The amount and balance being claimed
	pub amount: u32,
	/// time when claim request is made
	pub start_block_number: u32,
}

