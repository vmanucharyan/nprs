use structures::{Point, Rect};

pub trait Incremental {
    fn init(p: Point) -> Self;
    fn increment(&mut self, p: Point);
    fn merge(&mut self, other: &Self);
}

pub trait ExtremalRegion : Sized {
    fn points<'a> (&'a self) -> &'a [Point];
    fn bounds(&self) -> Rect;
    fn peaks<'a>(&'a self) -> &'a [Self];
    fn weight(&self) -> f32;
}
