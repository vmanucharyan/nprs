use structures::{Rect, Point, Quad};

pub struct Symbol {
    bound: Rect
}

pub struct NumberPlate {
    bound: Quad,
    symbols: Vec<Point>
}
