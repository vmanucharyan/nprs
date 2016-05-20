use image::Image;
use structures::Point;
use extract::cser::Incremental;

use super::Feature;

impl<A: Incremental, B: Incremental> Incremental for (A, B) {
    fn init(p: Point, reg_idx: usize) -> Self {
        (A::init(p, reg_idx), B::init(p, reg_idx))
    }

    fn increment(&mut self, p: Point, thres: i32, img: &Image<u8>,  reg_img: &Image<Option<usize>>) {
        self.0.increment(p, thres, img, reg_img);
        self.1.increment(p, thres, img, reg_img);
    }

    fn merge(&mut self, other: &Self, img: &Image<u8>, reg_image: &Image<Option<usize>>) {
        self.0.merge(&other.0, img, reg_image);
        self.1.merge(&other.1, img, reg_image);
    }
}

impl<A: Incremental + Feature, B: Incremental + Feature> Feature for (A, B) {
    fn value(&self, out: &mut Vec<f32>) {
        self.0.value(out);
        self.1.value(out);
    }
}

impl<A: Incremental, B: Incremental, C: Incremental> Incremental for (A, B, C) {
    fn init(p: Point, reg_idx: usize) -> Self {
        (A::init(p, reg_idx), B::init(p, reg_idx), C::init(p, reg_idx))
    }

    fn increment(&mut self, p: Point, thres: i32,   img: &Image<u8>,  reg_img: &Image<Option<usize>>) {
        self.0.increment(p, thres, img, reg_img);
        self.1.increment(p, thres, img, reg_img);
        self.2.increment(p, thres, img, reg_img);
    }

    fn merge(&mut self, other: &Self, img: &Image<u8>, reg_image: &Image<Option<usize>>) {
        self.0.merge(&other.0, img, reg_image);
        self.1.merge(&other.1, img, reg_image);
        self.2.merge(&other.2, img, reg_image);
    }
}

impl<A: Incremental + Feature,
     B: Incremental + Feature,
     C: Incremental + Feature
> Feature for (A, B, C) {
    fn value(&self, out: &mut Vec<f32>) {
        self.0.value(out);
        self.1.value(out);
        self.2.value(out);
    }
}

impl<A: Incremental,
     B: Incremental,
     C: Incremental,
     D: Incremental
> Incremental for (A, B, C, D) {
    fn init(p: Point, reg_idx: usize) -> Self {
        (A::init(p, reg_idx), B::init(p, reg_idx), C::init(p, reg_idx), D::init(p, reg_idx))
    }

    fn increment(&mut self, p: Point, thres: i32, img: &Image<u8>,  reg_img: &Image<Option<usize>>) {
        self.0.increment(p, thres, img, reg_img);
        self.1.increment(p, thres, img, reg_img);
        self.2.increment(p, thres, img, reg_img);
        self.3.increment(p, thres, img, reg_img);
    }

    fn merge(&mut self, other: &Self, img: &Image<u8>, reg_image: &Image<Option<usize>>) {
        self.0.merge(&other.0, img, reg_image);
        self.1.merge(&other.1, img, reg_image);
        self.2.merge(&other.2, img, reg_image);
        self.3.merge(&other.3, img, reg_image);
    }
}

impl<A: Incremental + Feature,
     B: Incremental + Feature,
     C: Incremental + Feature,
     D: Incremental + Feature
> Feature for (A, B, C, D) {
    fn value(&self, out: &mut Vec<f32>) {
        self.0.value(out);
        self.1.value(out);
        self.2.value(out);
        self.3.value(out);
    }
}

impl<A: Incremental + Feature,
     B: Incremental + Feature,
     C: Incremental + Feature,
     D: Incremental + Feature,
     E: Incremental + Feature
> Feature for (A, B, C, D, E) {
    fn value(&self, out: &mut Vec<f32>) {
        self.0.value(out);
        self.1.value(out);
        self.2.value(out);
        self.3.value(out);
        self.4.value(out);
    }
}
