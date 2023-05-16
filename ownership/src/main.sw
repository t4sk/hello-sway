contract;

use std::auth::{ msg_sender};

abi MyContract {
    #[storage(read)]
    fn owner() -> Identity;

    #[storage(read, write)]
    fn set_owner(id: Identity);
}

configurable {
    OWNER: Identity = Identity::Address(
        // TODO: config from wallet address
        Address::from(0x0000000000000000000000000000000000000000000000000000000000000000)
    )
}

storage {
    owner: Identity = OWNER,
}

impl MyContract for Contract {
    #[storage(read)]
    fn owner() -> Identity {
        storage.owner
    }

    #[storage(read, write)]
    fn set_owner(id: Identity) {
        let sender = msg_sender();
        require(sender.unwrap() == storage.owner, "not authorized");
        storage.owner = id;
    }
}
