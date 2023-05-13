script;

// methods - first param is self
// associated funcs - regular funcs

struct Rectangle {
    height: u64,
    width: u64,
    x: u64,
    y: u64
}

impl Rectangle {
    fn area(self) -> u64 {
        self.height * self.width
    }

    fn new(height: u64, width: u64) -> Self {
        Self {
            height,
            width,
            x: 0,
            y: 0
        }
    }

    fn move(ref mut self, dx: u64, dy: u64) {
        self.x += dx;
        self.y += dy;
    } 
}

fn main() {
    let mut rect = Rectangle::new(100, 200);
    rect.area();
    rect.move(10, 20);
}