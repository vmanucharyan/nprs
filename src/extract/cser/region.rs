use structures::{Point, Rect};
use image::Image;
use super::feature::Feature;
use super::incremental::{Incremental};
use extract::ExtremalRegion;

static PEAK_THRESHOLD: f32 = 0.05f32;

#[derive(Debug, Clone)]
pub struct Region<A: Incremental + Feature + Clone> {
    features: A,
    bounds: Rect,
    points: Vec<Point>,
    weight: f32,
    peaks: Vec<(Rect, A)>,
    prev_weight: f32
}

impl<A: Incremental + Feature + Clone> Region<A> {
    fn is_peak(&self, new_weight: f32) -> bool {
        self.weight - self.prev_weight > PEAK_THRESHOLD &&
        self.weight - new_weight > PEAK_THRESHOLD
    }
}

impl<A: Incremental + Feature + Clone> Incremental for Region<A> {
    fn init(p: Point, reg_idx: usize) -> Self {
        Region {
            features: A::init(p, reg_idx),
            bounds: Rect(p, p),
            points: vec![p],
            weight: -1f32,
            prev_weight: -1f32,
            peaks: vec![]
        }
    }

    fn increment(&mut self, p: Point, thres: i32, img: &Image<u8>,  reg_img: &Image<Option<usize>>) {
        self.features.increment(p, thres, img, reg_img);
        self.bounds = self.bounds.expand(Rect(p, p));
        self.weight += 0.1;
        self.points.push(p);

        let new_weight = ((self.points.len() % 50) as f32) / 50.0f32;

        if self.is_peak(new_weight) {
            self.peaks.push((self.bounds, self.features.clone()));
        }

        self.prev_weight = self.weight;
        self.weight = new_weight;
    }

    fn merge(&mut self, r: &Self, img: &Image<u8>, reg_image: &Image<Option<usize>>) {
        self.bounds = self.bounds.expand(r.bounds);
        self.points.extend_from_slice(&r.points[..]);
        self.features.merge(&r.features, img, reg_image);

        let new_weight = ((self.points.len() % 50) as f32) / 50.0f32;

        if self.is_peak(new_weight) {
            self.peaks.push((self.bounds, self.features.clone()));
        }

        self.prev_weight = self.weight;
        self.weight = new_weight;
    }
}

impl<A: Incremental + Feature + Clone> ExtremalRegion for Region<A> {
    type F = A;

    fn points<'a> (&'a self) -> &'a [Point] {
        &self.points[..]
    }

    fn weight(&self) -> f32 {
        ((self.points.len() % 10) as f32) / 10.0f32
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn peaks<'a> (&'a self) -> &'a [(Rect, A)] {
        &self.peaks[..]
    }
}

#[cfg(test)]
mod test {
    pub use super::*;
    pub use image::Image;
    pub use extract::cser::Incremental;
    pub use extract::cser::feature::Feature;
    pub use structures::{Point, Rect};

    #[derive(Clone)]
    pub struct FakeFeature {
        incremented: i32,
        merged: i32,
        init_point: Point,
    }

    impl Incremental for FakeFeature {
        fn init(p: Point, _: usize) -> Self {
            FakeFeature { init_point: p, incremented: 0, merged: 0 }
        }

        fn increment(&mut self, _: Point, thres: i32,   _: &Image<u8>,  reg_img: &Image<Option<usize>>) {
            self.incremented += 1;
        }

        fn merge(&mut self, _: &Self, _: &Image<u8>, reg_img: &Image<Option<usize>>) {
            self.merged += 1;
        }
    }

    impl Feature for FakeFeature {
        fn value(&self, out: &mut Vec<f32>) { }
    }

    describe! region {
        describe! init {
            before_each {
                let region: Region<FakeFeature> = Incremental::init(Point { x: 6, y: 3 }, 0);
            }

            it "should create Region with one point bounds" {
                assert_eq!(region.bounds, Rect(Point { x: 6, y: 3 }, Point { x: 6, y: 3 }));
            }

            it "should create Region that contains one point" {
                let expected_points = vec![Point { x: 6, y: 3 }];
                assert_eq!(region.points, expected_points);
            }

            it "should initialize features" {
                assert_eq!(region.features.init_point, Point { x: 6, y: 3 });
            }
        }

        describe! increment {
            before_each {
                let img: Image<u8> = Image::from_data(vec![], 0, 0);
                let reg_img: Image<Option<usize>> = Image::from_data(vec![], 0, 0);

                let mut region: Region<FakeFeature> = Incremental::init(Point { x: 6, y: 3 }, 0);
                region.increment(Point { x: 6, y: 4 }, 0, &img, &reg_img);
            }

            it "should add point to region" {
                let expected_points = vec![
                    Point { x: 6, y: 3 },
                    Point { x: 6, y: 4 }
                ];
                assert_eq!(region.points, expected_points);
            }

            it "should expand bounds of region" {
                let expected_bounds = Rect(
                    Point { x: 6, y: 3 },
                    Point { x: 6, y: 4}
                );
                assert_eq!(region.bounds, expected_bounds);
            }

            it "should increment features" {
                assert_eq!(region.features.incremented, 1);
            }
        }

        describe! merge {
            before_each {
                let img: Image<u8> = Image::from_data(vec![], 0, 0);
                let reg_img: Image<Option<usize>> = Image::from_data(vec![], 0, 0);

                let r1p1 = Point { x: 6, y: 3 };
                let r1p2 = Point { x: 6, y: 4 };
                let mut r1: Region<FakeFeature> = Incremental::init(r1p1, 0);
                r1.increment(r1p2, 0, &img, &reg_img);

                let r2p = Point { x:7, y: 3 };
                let mut r2: Region<FakeFeature> = Incremental::init(r2p, 1);

                r1.merge(&r2, &img, &reg_img);
            }

            it "should merge features" {
                assert_eq!(r1.features.merged, 1);
            }
        }
    }
}
