contract;

// payable
// msg_asset_id
// msg_amount
// BASE_ASSET_ID
// mint, burn, transfer
use std::{
    auth::{
        msg_sender,
    },
    call_frames::{
        contract_id,
        msg_asset_id,
    },
    constants::BASE_ASSET_ID,
    context::{msg_amount, balance_of},
    token::{
        mint_to,
        mint,
        burn,
        transfer
    },
};

abi WrappedToken {
    #[payable]
    fn deposit();
    #[payable]
    fn withdraw();
}

impl WrappedToken for Contract {
    #[payable]
    fn deposit() {
        require(msg_asset_id() == BASE_ASSET_ID, "not base asset");

        let amount = msg_amount();
        require(amount > 0, "msg amount = 0");

        // mint_to(amount, msg_sender().unwrap());
        mint(amount);
        transfer(amount, contract_id(), msg_sender().unwrap());
    }

    #[payable]
    fn withdraw() {
        require(msg_asset_id() == contract_id(), "asset id != contract id");

        let amount = msg_amount();
        require(amount > 0, "msg amount = 0");

        burn(amount);
        transfer(amount, BASE_ASSET_ID, msg_sender().unwrap());
    }
}
