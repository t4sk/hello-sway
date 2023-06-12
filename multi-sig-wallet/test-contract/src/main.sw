contract;

abi TestContract {
    #[storage(read, write)]
    fn test_function();
    #[storage(read)]
    fn get_count() -> u64;
}

storage {
    count: u64 = 0
}

impl TestContract for Contract {
    #[storage(read, write)]
    fn test_function() {
        storage.count += 1;
    }

    #[storage(read)]
    fn get_count() -> u64 {
        storage.count
    }
}
