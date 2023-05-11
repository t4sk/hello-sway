script;

fn eq(x: u64, y: u64) -> bool {
    x == y
}

fn inc(ref mut num: u64) {
    num += 1;
}

fn swap(ref mut pair: (u64,u64)) {
    let tmp = pair.0;
    pair.0 = pair.1;
    pair.1 = tmp;
}

fn main() {
    eq(11, 11);
    eq(11, 12);

    let mut num: u64 = 123;
    inc(num);
    assert(num == 123 + 1);

    let mut pair = (12, 13);
    swap(pair);
    assert(pair.0 == 13);
    assert(pair.1 == 12);
}