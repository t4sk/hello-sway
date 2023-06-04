contract;

mod errors;
mod events;

use std::{
    b512::B512,
    bytes::Bytes,
    call_frames::{
        contract_id,
    },
    ecr::{
        ec_recover_address,
    },
    hash::{
        sha256,
    },
    low_level_call::{
        call_with_function_selector,
        CallParams,
    },
    storage::{
        StorageMap,
        StorageVec,
    },
};
use ::errors::{ExecuteError, InitError, SignatureError};
use ::events::{ExecuteEvent};

// 2 out of 3 multisig wallet
struct ExecuteParams {
    contract_id: ContractId,
    fn_selector: Bytes,
    data: Bytes,
    single_value_type_arg: bool,
    call_params: CallParams,
    // pub struct CallParams {
    //     coins: u64,
    //     asset_id: ContractId,
    //     gas: u64,
    // }
}

abi MultiSigWallet {
    #[storage(read, write)]
    fn init(owners: Vec<Identity>);

    #[storage(read, write)]
    fn execute(params: ExecuteParams, sigs: Vec<B512>);
}

abi WalletInfo {
    #[storage(read)]
    fn nonce() -> u64;

    // #[storage(read)]
    // fn owners() -> Vec<Identity>;
}

configurable {
    MAX_OWNERS: u64 = 3,
    MIN_SIGS_REQUIRED: u64 = 2,
}

// TODO: events

storage {
    // TODO: owner can be contract?
    owners: StorageVec<Identity> = StorageVec {},
    is_owner: StorageMap<Identity, bool> = StorageMap {},
    nonce: u64 = 0,
}

impl MultiSigWallet for Contract {
    #[storage(read, write)]
    fn init(owners: Vec<Identity>) {
        require(storage.owners.len() == 0, InitError::CannotReinitialize);

        let num_owners = owners.len();
        require(num_owners > 0, InitError::ZeroOwners);
        require(num_owners <= MAX_OWNERS, InitError::MaxOwners);

        let mut i = 0;
        while i < num_owners {
            let owner = owners.get(i).unwrap();
            // TODO: check owner not zero?
            require(!storage.is_owner.get(owner).unwrap_or(false), InitError::DuplicateOwner);
            storage.is_owner.insert(owner, true);
            storage.owners.push(owner);
            i += 1;
        }
    }

    #[storage(read, write)]
    fn execute(params: ExecuteParams, sigs: Vec<B512>) {
        // check initialized
        let tx_hash = get_tx_hash(contract_id(), params, storage.nonce);

        // get approval count
        verify(sigs, tx_hash);
        require(sigs.len() >= MIN_SIGS_REQUIRED, ExecuteError::MinSignatures);

        let prev_nonce = storage.nonce;
        storage.nonce = prev_nonce + 1;

        // execute tx
        call_with_function_selector(params.contract_id, params.fn_selector, params.data, params.single_value_type_arg, params.call_params);

        // log
        log(ExecuteEvent {
            tx_hash,
            nonce: prev_nonce,
        });
    }

    // TODO: deposit, withdraw, transfer?
}

impl WalletInfo for Contract {
    // #[storage(read)]
    // fn owners() -> Vec<Identity> {
    //     storage.owners
    // }
    #[storage(read)]
    fn nonce() -> u64 {
        storage.nonce
    }
}

fn get_tx_hash(id: ContractId, params: ExecuteParams, nonce: u64) -> b256 {
    sha256((id, params, nonce))
}

fn recover_signer(sig: B512, hash: b256) -> b256 {
    // TODO: signature malleability?
    // TODO: recover contract id?
    ec_recover_address(sig, hash).unwrap().value
}

#[storage(read)]
fn verify(sigs: Vec<B512>, tx_hash: b256) {
    let mut prev_signer = b256::min();
    let mut i = 0;
    while i < sigs.len() {
        let signer = recover_signer(sigs.get(i).unwrap(), tx_hash);
        // TODO: log index inside error
        let signer_id = Identity::Address(Address::from(signer));
        require(storage.is_owner.get(signer_id).unwrap_or(false), SignatureError::NotOwner);
        require(prev_signer < signer, SignatureError::IncorrectSignerOrdering);

        prev_signer = signer;
        i += 1;
    }
}
