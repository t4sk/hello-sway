contract;

use std::{auth::{AuthError, msg_sender}, hash::sha256, logging::log};

struct Sent {
    from: Address,
    to: Address,
    amount: u64,
}

abi Token {
    #[storage(read, write)]
    fn mint(receiver: Address, amount: u64);

    #[storage(read, write)]
    fn send(receiver: Address, amount: u64);
}

const MINTER = Address::from(0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b);

storage {
    balances: StorageMap<Address, u64> = StorageMap {},
}

impl Token for Contract {
    #[storage(read, write)]
    fn mint(receiver: Address, amount: u64) {
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                assert(addr == MINTER);
                addr
            },
            _ => revert(0),
        };

        storage.balances.insert(receiver, storage.balances.get(receiver).try_read().unwrap_or(0) + amount);
    }

    #[storage(read, write)]
    fn send(receiver: Address, amount: u64) {
        let sender = msg_sender().unwrap();
        let sender = match sender {
            Identity::Address(addr) => addr,
            _ => revert(0),
        };

        let sender_amount = storage.balances.get(sender).try_read().unwrap_or(0);
        assert(sender_amount > amount);
        storage.balances.insert(sender, sender_amount - amount);

        let receiver_bal = storage.balances.get(receiver).try_read().unwrap_or(0);
        storage.balances.insert(receiver, receiver_bal + amount);

        log(Sent {
            from: sender,
            to: receiver,
            amount,
        })
    }
}