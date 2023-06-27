contract;

storage {
    count: u64 = 0
}

abi Counter {
    #[storage(read)]
    fn get() -> u64;

    #[storage(read, write)]
    fn inc() -> u64;

    #[storage(read, write)]
    fn dec() -> u64;
}

impl Counter for Contract {
    #[storage(read)]
    fn get() -> u64 {
        storage.count
    }

    #[storage(read, write)]
    fn inc() -> u64 {
        storage.count += 1;
        storage.count
    }

    #[storage(read, write)]
    fn dec() -> u64 {
        storage.count -= 1;
        storage.count
    }
}

// #[test]
// fn test_inc() {
//     let counter = abi(Counter, CONTRACT_ID);
//     let count = counter.inc();
//     assert(count == 1);
// }

// #[test(should_revert)]
// fn test_dec_underflow() {
//     let counter = abi(Counter, CONTRACT_ID);
//     counter.dec();
// }