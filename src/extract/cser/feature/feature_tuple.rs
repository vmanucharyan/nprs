use image::Image;
use structures::Point;
use extract::cser::Incremental;

use super::Feature;

impl<A: Incremental, B: Incremental> Incremental for (A, B) {
    fn init(p: Point) -> Self {
        (A::init(p), B::init(p))
    }

    fn increment(&mut self, p: Point, img: &Image<u8>) {
        self.0.increment(p, img);
        self.1.increment(p, img);
    }

    fn merge(&mut self, other: &Self) {
        self.0.merge(&other.0);
        self.1.merge(&other.1);
    }
}

impl<A: Incremental + Feature, B: Incremental + Feature> Feature for (A, B) {
    fn value(&self, out: &mut Vec<f32>) {
        self.0.value(out);
        self.1.value(out);
    }
}

impl<A: Incremental, B: Incremental, C: Incremental> Incremental for (A, B, C) {
    fn init(p: Point) -> Self {
        (A::init(p), B::init(p), C::init(p))
    }

    fn increment(&mut self, p: Point, img: &Image<u8>) {
        self.0.increment(p, img);
        self.1.increment(p, img);
        self.2.increment(p, img);
    }

    fn merge(&mut self, other: &Self) {
        self.0.merge(&other.0);
        self.1.merge(&other.1);
        self.2.merge(&other.2);
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
    fn init(p: Point) -> Self {
        (A::init(p), B::init(p), C::init(p), D::init(p))
    }

    fn increment(&mut self,    p: Point, img: &Image<u8>) {
        self.0.increment(p, img);
        self.1.increment(p, img);
        self.2.increment(p, img);
        self.3.increment(p, img);
    }

    fn merge(&mut self, other: &Self) {
        self.0.merge(&other.0);
        self.1.merge(&other.1);
        self.2.merge(&other.2);
        self.3.merge(&other.3);
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
