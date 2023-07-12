contract;

mod my_lib;
use ::my_lib::{min, Color};

abi MyContract {
    fn test_min(x: u64, y: u64) -> u64;
    fn test_color_eq(a: Color, b: Color) -> bool;
}

impl MyContract for Contract {
    fn test_min(x: u64, y: u64) -> u64 {
        // my_lib::min(x, y)
        min(x, y)
    }

    fn test_color_eq(a: Color, b: Color) -> bool {
        return a == b;
    }
}
