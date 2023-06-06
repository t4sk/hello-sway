contract;

abi TestContract {
    fn test_function() -> bool;
}

impl TestContract for Contract {
    fn test_function() -> bool {
        true
    }
}
