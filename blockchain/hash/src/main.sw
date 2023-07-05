contract;

use std::hash::{keccak256, sha256};

abi MyContract {
    fn test_hash() -> b256;
}

struct Point {
    x: u64,
    y: u64,
}

impl MyContract for Contract {
    fn test_hash() -> b256 {
        sha256(u64::max());
        sha256(b256::min());
        sha256((true, 123));
        sha256([1, 2, 3]);

        sha256((Point{x: 1, y: 2}, true, 123, [4, 5, 6]))
    }
}
