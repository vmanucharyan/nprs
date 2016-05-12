use structures::Point;

pub trait Incremental {
    fn init(p: Point) -> Self;
    fn increment(&mut self, p: Point);
    fn merge(&mut self, other: &Self);
}
