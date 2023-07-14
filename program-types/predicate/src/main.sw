predicate;

use std::hash::{sha256};

configurable {
    HASH: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000,
}

fn main(a: u64) -> bool {
    sha256(a) == HASH
}
