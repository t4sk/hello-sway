contract;

mod errors;

use std::storage::StorageVec;
use ::errors::{InitError};

// 2 out of 3 multisig wallet

abi MultiSigWallet {
    #[storage(read, write)]
    fn init(owners: Vec<Identity>);

    #[storage(read, write)]
    fn execute();
}

abi WalletInfo {
    #[storage(read)]
    fn nonce() -> u64;

    // #[storage(read)]
    // fn owners() -> Vec<Identity>;
}

configurable {
    MAX_OWNERS: u64 = 3,
    MIN_SIGS_REQUIRED: u64 = 2
}

// TODO: events

storage {
    owners: StorageVec<Identity> = StorageVec {},
    nonce: u64 = 1,
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
            // TODO: check unique owner?s
            // TODO: check owner not zero?
            storage.owners.push(owner);
            i += 1;
        }
    }

    #[storage(read, write)]
    fn execute() {
        // TODO: need generic_call
        // https://github.com/FuelLabs/sway-applications/issues/22
    }
}

impl WalletInfo for  Contract {
    // #[storage(read)]
    // fn owners() -> Vec<Identity> {
    //     storage.owners
    // }

    #[storage(read)]
    fn nonce() -> u64 {
        storage.nonce
    }
}