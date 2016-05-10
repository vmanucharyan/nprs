pub struct Point {
    x: i32,
    y: i32
}

pub struct Rect(Point, Point);
pub struct Quad(Point, Point, Point, Point);

pub struct Symbol {
    bound: Rect
}

pub struct NumberPlate {
    bound: Quad,
    symbols: Vec<Point>
}
