use image::Image;
use structures::{Point, Rect};

use super::Feature;

pub trait Incremental {
    fn init(p: Point, reg_idx: usize) -> Self;
    fn increment(&mut self, p: Point, thres: i32, _: &Image<u8>,  reg_img: &Image<Option<usize>>);
    fn merge(&mut self, other: &Self, _: &Image<u8>, _: &Image<Option<usize>>);
}
