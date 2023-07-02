contract;

use std::auth::msg_sender;

// Events
struct TransferEvent {
    from: Option<Identity>,
    to: Option<Identity>,
    amount: u64
}

abi MyContract {
    fn test_func(amount: u64);
}

impl MyContract for Contract {
    fn test_func(amount: u64) {
        let sender = msg_sender().unwrap();

        // Mint
        log(TransferEvent {
            from: Option::None,
            to: Option::Some(sender),
            amount,
        });

        // Burn
        log(TransferEvent {
            from: Option::Some(sender),
            to: Option::None,
            amount,
        })
    }
}
