contract;

// Topics - Tuples
// - Create, read, update
// - Nested
// - Destructure and "_"

abi MyContract {
    fn test_func() -> (u64, (str[4], bool));
}

impl MyContract for Contract {
    fn test_func() -> (u64, (str[4], bool)) {
        let mut tuple: (u64, bool, u64) = (1, false, 2);
        let x = tuple.0;

        let nested_tuple = (1, ("Fuel", false));
        let s = nested_tuple.1.0;

        let (n, (s, b)) = nested_tuple;
        // Skip variables for 0 and 1.1 
        let (_, (s, _)) = nested_tuple;

        nested_tuple
    }
}
