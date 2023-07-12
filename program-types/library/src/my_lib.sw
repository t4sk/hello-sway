library;

pub fn min(x: u64, y: u64) -> u64 {
    if (x <= y) {
        return x;
    }
    y
}

pub enum Color {
    Red: (),
    Green: (),
    Blue: (),
}

impl core::ops::Eq for Color {
    fn eq(self, other: Self) -> bool {
        match self {
            Red => match other {
                Red => true,
                _ => false,
            },
            Green => match other {
                Green => true,
                _ => false,
            },
            Blue => match other {
                Blue => true,
                _ => false,
            },
        }
    }
}