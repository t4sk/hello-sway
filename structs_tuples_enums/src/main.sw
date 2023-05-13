script;

struct Point {
    x: u64,
    y: u64,
}

// Nested struct
struct Line {
    p0: Point,
    p1: Point,
}

// Tuple in struct
struct TupleInStruct {
    nested_tuple: (u64, (bool, str[4])),
}

enum Color {
    Red: (),
    Blue: (),
    Green: (),
}

// Enums of structs
enum Geometry {
    Point: Point,
    Line: Line
}

enum Shape {
    Circle: Color,
    Triangle: Color
}

fn main() {
    // Initiate
    let mut p0 = Point {
        x: 1,
        y: 2,
    };

    // Update
    p0.x = 11;

    // Shorthand
    let x: u64 = 123;
    let y: u64 = 123;

    let p1 = Point {x, y};

    // Line
    let line = Line {p0, p1};

    // Destructure
    let Line {
        p0: Point { x: x0, y: y0},
        p1: Point { x: x1, y: y1},
    } = line;

    let sum = x0 + x1 + y0 + y1;

    // Tuple in struct
    let t = TupleInStruct {
        nested_tuple: (123, (true, "sway"))
    };

    // Destructure
    let TupleInStruct {
        nested_tuple: (a, (b, s))
    } = t;

    // Structs have zero memory overhead

    // Tuples
    let mut tuple: (u64, bool, u64) = (1, false, 2);
    let x = tuple.0;

    let nested_tuple = (1, ("Fuel", false));
    let s = nested_tuple.1.0;

    let (_, (s, _)) = nested_tuple;

    // Enums
    let blue = Color::Blue;

    let geometry = Geometry::Line(Line {
        p0: Point{x: 1, y: 2},
        p1: Point{x: 1, y: 2}
    });

    let shape = Shape::Circle(Color::Red);
}