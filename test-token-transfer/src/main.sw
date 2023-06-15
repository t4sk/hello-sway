contract;

use std::{
    constants::{BASE_ASSET_ID},
};

abi MyContract {
    #[payable]
    fn deposit();

    fn transfer(amount: u64, to: Identity);
}

impl MyContract for Contract {
    #[payable]
    fn deposit() {}

    fn transfer(amount: u64, to: Identity) {
        std::token::transfer(amount, BASE_ASSET_ID, to);
    }
}
