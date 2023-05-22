contract;

use std::{
    call_frames::{msg_asset_id, contract_id},
    context::msg_amount,
    auth::{msg_sender},
    constants::BASE_ASSET_ID,
    token::{
        mint_to_address,
        mint_to_contract,
        burn,
        transfer_to_address,
        force_transfer_to_contract
    }
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

        let id = msg_sender().unwrap();
        match id {
            Identity::Address(addr) => mint_to_address(amount, addr),
            Identity::ContractId(con_id) => mint_to_contract(amount, con_id),
        };
    }

    #[payable]
    fn withdraw() {
        require(msg_asset_id() == contract_id(), "asset id != contract id");

        let amount = msg_amount();
        require(amount > 0, "msg amount = 0");

        burn(amount);

        let id = msg_sender().unwrap();
        match id {
            Identity::Address(addr) => transfer_to_address(amount, BASE_ASSET_ID, addr),
            Identity::ContractId(con_id) => force_transfer_to_contract(amount, BASE_ASSET_ID, con_id),
        };
    }
}
