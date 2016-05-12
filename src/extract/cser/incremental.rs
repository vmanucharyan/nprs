use structures::Point;

pub trait Incremental {
    fn init(p: Point) -> Self;
    fn increment(&self, p: Point) -> Self;
    fn merge(&self, other: &Self) -> Self;
}
