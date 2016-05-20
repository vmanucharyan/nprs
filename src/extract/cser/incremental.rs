use image::Image;
use structures::Point;

pub trait Incremental {
    fn init(p: Point, reg_idx: usize, thres: i32) -> Self;
    fn increment(&mut self, p: Point, thres: i32, _: &Image<u8>,  reg_img: &Image<Option<usize>>);
    fn merge(&mut self, other: &Self, thres: i32, _: &Image<u8>, _: &Image<Option<usize>>);
}
