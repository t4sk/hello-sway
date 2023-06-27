contract;

mod errors;
mod events;

use std::{
    auth::msg_sender,
    call_frames::msg_asset_id,
    context::{
        msg_amount,
        this_balance,
    },
    hash::sha256,
    logging::log,
    token::transfer,
};
use merkle_proof::binary_merkle_proof::{leaf_digest, verify_proof};
use ::errors::{ClaimError, InitError};

abi AirDrop {
    #[storage(read, write), payable]
    fn init(merkle_root: b256, num_leaves: u64);

    #[storage(read, write)]
    fn claim(amount: u64, index: u64, proof: Vec<b256>);
}

abi AirDropInfo {
    #[storage(read)]
    fn asset() -> Option<ContractId>;

    #[storage(read)]
    fn merkle_root() -> Option<b256>;

    #[storage(read)]
    fn num_leaves() -> u64;

    #[storage(read)]
    fn claimed(index: u64) -> bool;
}

storage {
    asset: Option<ContractId> = Option::None,
    merkle_root: Option<b256> = Option::None,
    num_leaves: u64 = 0,
    claims: StorageMap<u64, bool> = StorageMap {},
}

impl AirDrop for Contract {
    #[storage(read, write), payable]
    fn init(merkle_root: b256, num_leaves: u64) {
        require(storage.merkle_root.is_none(), InitError::CannotReinitialize);
        require(msg_amount() > 0, InitError::ZeroTokens);

        let asset = msg_asset_id();
        storage.asset = Option::Some(asset);
        storage.merkle_root = Option::Some(merkle_root);
        storage.num_leaves = num_leaves;

        log(events::InitEvent {
            asset,
            merkle_root,
            num_leaves,
        });
    }

    #[storage(read, write)]
    fn claim(amount: u64, index: u64, proof: Vec<b256>) {
        // Check initializd
        require(storage.merkle_root.is_some(), ClaimError::NotInitialized);

        // Check not claimed
        require(!storage.claims.get(index).unwrap_or(false), ClaimError::AlreadyClaimed);

        // Check merkle proof
        let sender = msg_sender().unwrap();
        require(verify_proof(index, leaf_digest(sha256((sender, amount))), storage.merkle_root.unwrap(), storage.num_leaves, proof), ClaimError::InvalidMerkleProof);

        // Check token balance (TODO: not needed?)
        let asset = storage.asset.unwrap();
        require(this_balance(asset) >= amount, ClaimError::InsufficientBalance);

        // Update claims
        storage.claims.insert(index, true);

        // Transfer token
        transfer(amount, asset, sender);

        // Log
        log(events::ClaimEvent {
            index,
            amount,
            sender,
        });
    }
}

impl AirDropInfo for Contract {
    #[storage(read)]
    fn asset() -> Option<ContractId> {
        storage.asset
    }

    #[storage(read)]
    fn merkle_root() -> Option<b256> {
        storage.merkle_root
    }

    #[storage(read)]
    fn num_leaves() -> u64 {
        storage.num_leaves
    }

    #[storage(read)]
    fn claimed(index: u64) -> bool {
        storage.claims.get(index).unwrap_or(false)
    }
}
