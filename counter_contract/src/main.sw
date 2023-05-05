contract;

storage {
    counter: u64 = 0,
}

abi Counter {
    #[storage(read, write)]
    fn increment();

    #[storage(read)]
    fn counter() -> u64;
}

impl Counter for Contract {
    #[storage(read)]
    fn counter() -> u64 {
        return storage.counter.read();
    }

    #[storage(read, write)]
    fn increment() {
        let count = storage.counter.read() + 1;
        storage.counter.write(count);
    }
}
