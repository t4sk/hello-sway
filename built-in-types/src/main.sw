script;

struct Point {
    x: u64,
    y: u64,
}

// Primitive types
// - Unsigned integers
// - Fixed length string
// - Boolean
// - 256 bits = 32 bytes

// Compound types
// - Tuples

fn main() {
    // Unsigned integers
    // 0 <= u8 <= 2**8 - 1
    let u_8: u8 = 123;
    // 0 <= u16 <= 2**16 - 1
    let u_16: u16 = 123;
    // 0 <= u32 <= 2**32 - 1
    let u_32: u32 = 123;
    // 0 <= u64 <= 2**64 - 1
    let u_64: u64 = 123;

    let u_64_max = u64::max();

    // Fixed length string
    let s: str[4] = "fuel";
    // Boolean
    let boo: bool = true;
    // 256 bits = 32 bytes
    let b_256: b256 = 0x1111111111111111111111111111111111111111111111111111111111111111;

    // Tuples
    let t: (u64, bool) = (42, true);
    // Access tuple value
    assert(t.0 == 42);
    assert(t.1);

    // Struct
    let p = Point {x: 1, y: 2};
    // Access struct fields
    assert(p.x == 1);
    assert(p.x == 2);

    // Array
    let u_arr: [u64; 5] = [1, 2, 3, 4, 5];

    let struct_arr: [Point; 2] = [
        Point{x: 1, y: 2},
        Point{x: 11, y: 22},
    ];

    // Mutating array
    let mut mut_arr: [bool; 2] = [true, false];
    mut_arr[1] = true;
}