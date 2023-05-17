contract;

use std::{
    call_frames::msg_asset_id,
    constants::BASE_ASSET_ID,
    context::msg_amount,
    token::transfer_to_address,
};

abi Wallet {
    #[storage(read, write)]
    fn deposit();

    #[storage(read, write)]
    fn send(to: Address, amount: u64);
}

storage {
    balance: u64 = 0,
}

impl Wallet for Contract {
    #[storage(read, write)]
    fn deposit() {
        if msg_asset_id() == BASE_ASSET_ID {
            storage.balance += msg_amount();
        }
    }

    #[storage(read, write)]
    fn send(to: Address, amount: u64) {
        // TODO: require owner
        // TODO: check underflow
        storage.balance -= amount;

        // TODO: error
        transfer_to_address(amount, BASE_ASSET_ID, to);
    }
}
