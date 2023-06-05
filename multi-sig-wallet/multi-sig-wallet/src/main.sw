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
    token::{
        transfer,
    },
};
use ::errors::{InitError, SignatureError};
use ::events::{ExecuteEvent, TransferEvent};

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

struct TransferParams {
    to: Identity,
    asset_id: ContractId,
    amount: u64,
}

abi MultiSigWallet {
    #[storage(read, write)]
    fn init(owners: Vec<Identity>);

    #[storage(read, write)]
    fn execute(params: ExecuteParams, sigs: Vec<B512>);

    // TODO: transfer assets with execute
    #[storage(read, write)]
    fn transfer(params: TransferParams, sigs: Vec<B512>);
}

abi WalletInfo {
    #[storage(read)]
    fn owners() -> Vec<Identity>;

    #[storage(read)]
    fn nonce() -> u64;
}

configurable {
    MAX_OWNERS: u64 = 3,
    MIN_SIGS_REQUIRED: u64 = 2,
}

storage {
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
        let tx_hash = sha256((contract_id(), params, storage.nonce));

        // get approval count
        verify(sigs, tx_hash);
        require(sigs.len() >= MIN_SIGS_REQUIRED, SignatureError::MinSignatures);

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
    #[storage(read, write)]
    fn transfer(params: TransferParams, sigs: Vec<B512>) {
        let tx_hash = sha256((contract_id(), params, storage.nonce));

        // get approval count
        verify(sigs, tx_hash);
        require(sigs.len() >= MIN_SIGS_REQUIRED, SignatureError::MinSignatures);

        let prev_nonce = storage.nonce;
        storage.nonce = prev_nonce + 1;

        // execute tx
        transfer(params.amount, params.asset_id, params.to);

        // log
        log(TransferEvent {
            tx_hash,
            nonce: prev_nonce,
        });
    }
}

impl WalletInfo for Contract {
    #[storage(read)]
    fn owners() -> Vec<Identity> {
        let mut owners: Vec<Identity> = Vec::new();

        let mut i = 0;
        while i < storage.owners.len() {
            owners.push(storage.owners.get(i).unwrap());
            i += 1;
        }
        return owners;
    }

    #[storage(read)]
    fn nonce() -> u64 {
        storage.nonce
    }
}

#[storage(read)]
fn verify(sigs: Vec<B512>, tx_hash: b256) {
    let mut prev_signer = b256::min();
    let mut i = 0;
    while i < sigs.len() {
        let signer = ec_recover_address(sigs.get(i).unwrap(), tx_hash).unwrap().value;
        // TODO: can contracts be signer?
        let signer_id = Identity::Address(Address::from(signer));
        require(storage.is_owner.get(signer_id).unwrap_or(false), SignatureError::NotOwner);
        require(prev_signer < signer, SignatureError::IncorrectSignerOrdering);

        prev_signer = signer;
        i += 1;
    }
}
