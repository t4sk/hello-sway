contract;

mod errors;
mod events;

use std::{
    auth::msg_sender,
    b512::B512,
    bytes::Bytes,
    call_frames::{
        contract_id,
    },
    constants::{
        BASE_ASSET_ID,
    },
    context::{
        this_balance,
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
        transfer_to_address,
    },
};
use ::errors::{AccessControlError, InitError, SignatureError};
use ::events::{ExecuteEvent, TransferEvent};

// 2 out of 3 multisig wallet
struct ExecuteParams {
    contract_id: ContractId,
    fn_selector: Bytes,
    data: Bytes,
    single_value_type_arg: bool,
    call_params: CallParams,
}

struct TransferParams {
    to: Identity,
    asset_id: ContractId,
    amount: u64,
}

abi MultiSigWallet {
    #[storage(read, write)]
    fn init(owners: Vec<Identity>);

    #[payable]
    fn deposit();

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

    // Read only functions for debugging
    fn get_execute_tx_hash(params: ExecuteParams, nonce: u64) -> b256;

    fn get_signers(params: ExecuteParams, nonce: u64, sigs: Vec<B512>) -> Vec<Identity>;
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

    #[payable]
    fn deposit() {}

    #[storage(read, write)]
    fn execute(params: ExecuteParams, sigs: Vec<B512>) {
        let sender = msg_sender().unwrap();
        require(storage.is_owner.get(sender).unwrap_or(false), AccessControlError::NotAuthorized);

        let tx_hash = _get_execute_tx_hash(params, storage.nonce);

        // Check approvals
        require(sigs.len() >= MIN_SIGS_REQUIRED, SignatureError::MinSignatures);
        verify(sigs, tx_hash);

        let prev_nonce = storage.nonce;
        storage.nonce = prev_nonce + 1;

        // TODO: return response?
        // Execute tx
        call_with_function_selector(params.contract_id, params.fn_selector, params.data, params.single_value_type_arg, params.call_params);

        log(ExecuteEvent {
            tx_hash,
            nonce: prev_nonce,
        });
    }

    #[storage(read, write)]
    fn transfer(params: TransferParams, sigs: Vec<B512>) {
        let sender = msg_sender().unwrap();
        require(storage.is_owner.get(sender).unwrap_or(false), AccessControlError::NotAuthorized);

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

    fn get_execute_tx_hash(params: ExecuteParams, nonce: u64) -> b256 {
        _get_execute_tx_hash(params, nonce)
    }

    fn get_signers(params: ExecuteParams, nonce: u64, sigs: Vec<B512>) -> Vec<Identity> {
        let tx_hash = _get_execute_tx_hash(params, nonce);

        let mut signers: Vec<Identity> = Vec::new();
        let mut i = 0;
        while i < sigs.len() {
            let signer = ec_recover_address(sigs.get(i).unwrap(), tx_hash).unwrap().value;
            // TODO: can contracts be signer?
            let signer_id = Identity::Address(Address::from(signer));
            i += 1;
            signers.push(signer_id);
        }
        signers
    }
}

fn _get_execute_tx_hash(params: ExecuteParams, nonce: u64) -> b256 {
    sha256((
        contract_id(),
        params.contract_id,
        params.fn_selector.sha256(),
        params.data.sha256(),
        params.single_value_type_arg,
        params.call_params,
        nonce,
    ))
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
