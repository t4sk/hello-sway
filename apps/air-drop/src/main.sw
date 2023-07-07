contract;

mod errors;
mod events;

use std::{
    auth::msg_sender,
    call_frames::msg_asset_id,
    context::{
        msg_amount,
    },
    hash::sha256,
    logging::log,
    token::transfer,
};
use merkle_proof::binary_merkle_proof::{leaf_digest, verify_proof};
use ::errors::{ClaimError, DepositError};

abi AirDrop {
    #[payable]
    fn deposit();

    #[storage(read, write)]
    fn claim(amount: u64, index: u64, proof: Vec<b256>);
}

abi AirDropInfo {
    #[storage(read)]
    fn asset() -> ContractId;

    #[storage(read)]
    fn merkle_root() -> b256;

    #[storage(read)]
    fn num_leaves() -> u64;

    #[storage(read)]
    fn claimed(index: u64) -> bool;
}

const ZERO: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;

configurable {
    ASSET: ContractId = ContractId::from(ZERO),
    MERKLE_ROOT: b256 = ZERO,
    NUM_LEAVES: u64 = 0,
}

storage {
    claims: StorageMap<u64, bool> = StorageMap {},
}

impl AirDrop for Contract {
    #[payable]
    fn deposit() {
        require(msg_amount() > 0, DepositError::ZeroAmount);
        require(msg_asset_id() == ASSET, DepositError::NotAirDropAsset);
    }

    #[storage(read, write)]
    fn claim(amount: u64, index: u64, proof: Vec<b256>) {
        // Check not claimed
        require(!storage.claims.get(index).unwrap_or(false), ClaimError::AlreadyClaimed);

        // Check merkle proof
        let sender = msg_sender().unwrap();
        require(verify_proof(index, leaf_digest(sha256((sender, amount))), MERKLE_ROOT, NUM_LEAVES, proof), ClaimError::InvalidMerkleProof);

        // Update claims
        storage.claims.insert(index, true);

        // Transfer token
        transfer(amount, ASSET, sender);

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
    fn asset() -> ContractId {
        ASSET
    }

    #[storage(read)]
    fn merkle_root() -> b256 {
        MERKLE_ROOT
    }

    #[storage(read)]
    fn num_leaves() -> u64 {
        NUM_LEAVES
    }

    #[storage(read)]
    fn claimed(index: u64) -> bool {
        storage.claims.get(index).unwrap_or(false)
    }
}
