contract;

mod errors;

use std::auth::{msg_sender};
use ::errors::{AccessControlError};


abi MyContract {
    #[storage(read)]
    fn owner() -> Identity;

    #[storage(read, write)]
    fn set_owner(id: Identity);
}

configurable {
    OWNER: Identity = Identity::Address(Address::from(0x0000000000000000000000000000000000000000000000000000000000000000)),
}

storage {
    owner: Identity = OWNER,
}

// TODO: test
impl MyContract for Contract {
    #[storage(read)]
    fn owner() -> Identity {
        storage.owner
    }

    #[storage(read, write)]
    fn set_owner(id: Identity) {
        let sender = msg_sender();
        require(sender.unwrap() == storage.owner, AccessControlError::UnauthorizedError);
        storage.owner = id;
    }
}
