contract;

mod events;
mod errors;

use std::auth::msg_sender;
use ::errors::{TokenError};

abi NFT {
    // Read
    #[storage(read)]
    fn owner_of(token_id: u64) -> Option<Identity>;
    #[storage(read)]
    fn balance_of(owner: Identity) -> u64;
    #[storage(read)]
    fn approvals(token_id: u64) -> Option<Identity>;
    #[storage(read)]
    fn is_approved_for_all(owner: Identity, operator: Identity) -> bool;

    // Write
    #[storage(read, write)]
    fn mint(to: Identity) -> u64;
    #[storage(read, write)]
    fn burn(token_id: u64);
    #[storage(read, write)]
    fn approve(spender: Identity, token_id: u64);
    #[storage(read, write)]
    fn set_approval_for_all(operator: Identity, approved: bool);
    #[storage(read, write)]
    fn transfer_from(from: Identity, to: Identity, token_id: u64);
}

storage {
    token_id: u64 = 0,
    owner_of: StorageMap<u64, Identity> = StorageMap {},
    balance_of: StorageMap<Identity, u64> = StorageMap {},
    approvals: StorageMap<u64, Identity> = StorageMap {},
}

impl NFT for Contract {
    // Read
    #[storage(read)]
    fn owner_of(token_id: u64) -> Option<Identity> {
        storage.owner_of.get(token_id)
    }

    #[storage(read)]
    fn balance_of(owner: Identity) -> u64 {
        storage.balance_of.get(owner).unwrap_or(0)
    }

    #[storage(read)]
    fn approvals(token_id: u64) -> Option<Identity> {
        storage.approvals.get(token_id)
    }

    #[storage(read)]
    fn is_approved_for_all(owner: Identity, operator: Identity) -> bool {
        false
    }

    // Write
    #[storage(read, write)]
    fn mint(to: Identity) -> u64 {
        storage.token_id += 1;
        let token_id = storage.token_id;
        let owner = msg_sender().unwrap();

        storage.owner_of.insert(token_id, owner);
        let bal = storage.balance_of.get(owner).unwrap_or(0);
        storage.balance_of.insert(owner, bal + 1);

        log(events::TransferEvent {
            token_id,
            from: Option::None,
            to: Option::Some(owner)
        });

        token_id
    }

    #[storage(read, write)]
    fn burn(token_id: u64) {
        let owner = storage.owner_of.get(token_id).unwrap();
        let sender = msg_sender().unwrap();
        require(owner == sender, TokenError::NotOwner);

        // NOTE: token exists, so check is optional
        require(storage.owner_of.remove(token_id), TokenError::DoesNotExist);

        let bal = storage.balance_of.get(sender).unwrap_or(0);
        // TODO: check underflow?
        storage.balance_of.insert(sender, bal - 1);

        // TODO: clear approvals

        log(events::TransferEvent {
            token_id,
            from: Option::Some(sender),
            to: Option::None
        });
    }

    #[storage(read, write)]
    fn approve(spender: Identity, token_id: u64) {
        let owner = storage.owner_of.get(token_id).unwrap();
        let sender = msg_sender().unwrap();
        // TODO: check is approved for all
        require(owner == sender, TokenError::NotAuthorized);

        storage.approvals.insert(token_id, spender);

        log(events::ApprovalEvent{
            owner,
            spender,
            token_id
        });
    }

    #[storage(read, write)]
    fn set_approval_for_all(operator: Identity, approved: bool) {}

    #[storage(read, write)]
    fn transfer_from(from: Identity, to: Identity, token_id: u64) {}
}
