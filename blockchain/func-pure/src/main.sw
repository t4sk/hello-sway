contract;

storage {
    num: u64 = 0
}

abi MyContract {
    fn add(x: u64, y: u64) -> u64;

    #[storage(read)]
    fn add_with_storage_val() -> u64;
}

impl MyContract for Contract {
    // Pure function - does not read from storage
    fn add(x: u64, y: u64) -> u64 {
        x + y
    }

    // Impure function - reads from storage
    #[storage(read)]
    fn add_with_storage_val() -> u64 {
        1 + storage.num
    }
}
