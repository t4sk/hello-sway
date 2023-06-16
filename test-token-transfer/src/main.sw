contract;

use std::{
    call_frames::{contract_id},
    constants::{BASE_ASSET_ID},
};

abi MyContract {
    #[payable]
    fn deposit();
    fn transfer(amount: u64, to: Identity);
    fn get_balance_of_contract() -> u64;
}

impl MyContract for Contract {
    #[payable]
    fn deposit() {}

    fn transfer(amount: u64, to: Identity) {
        std::token::transfer(amount, BASE_ASSET_ID, to);
    }

    fn get_balance_of_contract() -> u64 {
        std::context::balance_of(contract_id(), BASE_ASSET_ID)
    }
}
