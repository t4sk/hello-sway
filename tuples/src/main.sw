script;

// Topics - Tuples
// - Create, read, update
// - Nested
// - Destructure and "_"

fn main() {
    let mut tuple: (u64, bool, u64) = (1, false, 2);
    let x = tuple.0;

    let nested_tuple = (1, ("Fuel", false));
    let s = nested_tuple.1.0;

    let (n, (s, b)) = nested_tuple;
    // Skip variables for 0 and 1.1 
    let (_, (s, _)) = nested_tuple;
}