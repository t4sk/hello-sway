contract;

use std::{context::msg_amount, auth::msg_sender, call_frames::{msg_asset_id}};

abi Receiver {
    #[payable]
    fn test_func(x: u64, y: u64) -> (u64, u64);
}

struct LogReceiverEvent {
    sender: Identity,
    asset: ContractId,
    amount: u64
}

impl Receiver for Contract {
    #[payable]
    fn test_func(x: u64, y: u64) -> (u64, u64) {
        log(LogReceiverEvent {
            sender: msg_sender().unwrap(),
            asset: msg_asset_id(),
            amount: msg_amount()
        });

        (x, y)
    }
}
