script;

configurable {
    MY_STRING: str[4] = "fuel",
    ARRAY: [u32; 3] = [1, 2, 3],
}

fn main() -> u64 {
    // Variables are immutable by default
    // u64 is default numeric type
    // mut - mutable
    let mut foo = 5;
    foo = 6;

    // Type annotations
    let x: u32 = 5;
    let s: str[4] = "sway";
    let b: bool = true;

    // Configuration-time constants
    let config_string = MY_STRING;

    return 1;
}
