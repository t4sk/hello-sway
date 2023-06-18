contract;

mod errors;

use std::auth::msg_sender;
use std::{
    call_frames::msg_asset_id,
    constants::BASE_ASSET_ID,
    context::msg_amount,
    token::transfer_to_address,
};
use ::errors::{AccessControlError, InitError};

abi MyWallet {
    #[storage(read, write)]
    fn init();

    #[storage(read, write), payable]
    fn deposit();

    #[storage(read, write)]
    fn send(to: Address, amount: u64);

    #[storage(read)]
    fn owner() -> Option<Identity>;

    #[storage(read)]
    fn balance() -> u64;
}

storage {
    owner: Option<Identity> = Option::None,
    balance: u64 = 0,
}

impl MyWallet for Contract {
    #[storage(read)]
    fn owner() -> Option<Identity> {
        storage.owner
    }

    #[storage(read)]
    fn balance() -> u64 {
        storage.balance
    }

    #[storage(read, write)]
    fn init() {
        require(storage.owner.is_none(), InitError::CannotReinitialize);
        let sender = msg_sender().unwrap();
        storage.owner = Option::Some(sender);
    }

    #[storage(read, write), payable]
    fn deposit() {
        if msg_asset_id() == BASE_ASSET_ID {
            storage.balance += msg_amount();
        }
    }

    #[storage(read, write)]
    fn send(to: Address, amount: u64) {
        let sender = msg_sender();
        require(sender.unwrap() == storage.owner.unwrap(), AccessControlError::UnauthorizedError);
        // TODO: check underflow
        storage.balance -= amount;
        // TODO: test

        transfer_to_address(amount, BASE_ASSET_ID, to);
    }
}