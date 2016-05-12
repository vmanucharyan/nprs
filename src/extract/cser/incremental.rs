use structures::Point;

pub trait Incremental {
    fn init(p: Point) -> Self;
    fn increment(&mut self, p: Point);
    fn merge(&mut self, other: &Self);
    fn points<'a> (&'a self) -> Option<&'a[Point]> { None }
}
