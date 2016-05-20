use image::Image;
use structures::{Point, Rect};

use super::Feature;

pub trait Incremental {
    fn init(p: Point, reg_idx: usize) -> Self;
    fn increment(&mut self, p: Point,  _: &Image<u8>, reg_img: &Image<Option<usize>>);
    fn merge(&mut self, other: &Self, _: &Image<u8>, _: &Image<Option<usize>>);
}

pub trait ExtremalRegion : Sized {
    type F: Feature + Sized;

    fn points<'a> (&'a self) -> &'a [Point];
    fn bounds(&self) -> Rect;
    fn peaks<'a>(&'a self) -> &'a [(Rect, Self::F)];
    fn weight(&self) -> f32;
}
