use structures::Point;

pub trait Feature {
    fn init(p: Point) -> Self;
    fn increment(&self, p: Point) -> Self;
    fn merge(&self, other: &Self) -> Self;
    fn value(&self) -> f32;
}
