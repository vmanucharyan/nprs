use structures::{Point, Rect};
use super::feature::Feature;
use super::incremental::{Incremental, ExtremalRegion};

static PEAK_THRESHOLD: f32 = 0.05f32;

#[derive(Debug, Clone)]
pub struct Region<A: Incremental + Feature + Clone> {
    features: A,
    bounds: Rect,
    points: Vec<Point>,
    weight: f32,
    peaks: Vec<Region<A>>,
    prev_weight: f32
}

impl<A: Incremental + Feature + Clone> Region<A> {
    fn is_peak(&self, new_weight: f32) -> bool {
        self.weight - self.prev_weight > PEAK_THRESHOLD &&
        self.weight - new_weight > PEAK_THRESHOLD
    }
}

impl<A: Incremental + Feature + Clone> Incremental for Region<A> {
    fn init(p: Point) -> Self {
        Region {
            features: A::init(p),
            bounds: Rect(p, p),
            points: vec![p],
            weight: -1f32,
            prev_weight: -1f32,
            peaks: vec![]
        }
    }

    fn increment(&mut self, p: Point) {
        self.features.increment(p);
        self.bounds = self.bounds.expand(Rect(p, p));
        self.weight += 0.1;
        self.points.push(p);

        let new_weight = ((self.points.len() % 50) as f32) / 50.0f32;

        if self.is_peak(new_weight) {
            let clone = Region {
                features: self.features.clone(),
                bounds: self.bounds.clone(),
                points: vec![],
                weight: self.weight,
                peaks: vec![],
                prev_weight: self.prev_weight,
            };
            self.peaks.push(clone);
        }

        self.prev_weight = self.weight;
        self.weight = new_weight;
    }

    fn merge(&mut self, r: &Self) {
        self.bounds = self.bounds.expand(r.bounds);
        self.points.extend_from_slice(&r.points[..]);
        self.features.merge(&r.features);

        let new_weight = ((self.points.len() % 50) as f32) / 50.0f32;

        if self.is_peak(new_weight) {
            let clone = Region {
                features: self.features.clone(),
                bounds: self.bounds.clone(),
                points: vec![],
                weight: self.weight,
                peaks: vec![],
                prev_weight: self.prev_weight,
            };
            self.peaks.push(clone);
        }

        self.prev_weight = self.weight;
        self.weight = new_weight;
    }
}

impl<A: Incremental + Feature + Clone> ExtremalRegion for Region<A> {
    fn points<'a> (&'a self) -> &'a [Point] {
        &self.points[..]
    }

    fn weight(&self) -> f32 {
        ((self.points.len() % 10) as f32) / 10.0f32
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn peaks<'a> (&'a self) -> &'a [Self] {
        &self.peaks[..]
    }
}

#[cfg(test)]
mod test {
    pub use super::*;
    pub use extract::cser::Incremental;
    pub use structures::{Point, Rect};
    pub use super::super::feature::Feature;

    #[derive(Clone)]
    pub struct FakeFeature {
        incremented: i32,
        merged: i32,
        init_point: Point,
    }

    impl Incremental for FakeFeature {
        fn init(p: Point) -> Self {
            FakeFeature { init_point: p, incremented: 0, merged: 0 }
        }
        fn increment(&mut self, _: Point) { self.incremented += 1; }
        fn merge(&mut self, _: &FakeFeature) { self.merged += 1; }
    }

    impl Feature for FakeFeature {
        fn value(&self, out: &mut Vec<f32>) { }
    }

    describe! region {
        describe! init {
            before_each {
                let region: Region<FakeFeature> = Incremental::init(Point { x: 6, y: 3 });
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
                let mut region: Region<FakeFeature> = Incremental::init(Point { x: 6, y: 3 });
                region.increment(Point { x: 6, y: 4 });
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
                let r1p1 = Point { x: 6, y: 3 };
                let r1p2 = Point { x: 6, y: 4 };
                let mut r1: Region<FakeFeature> = Incremental::init(r1p1);
                r1.increment(r1p2);

                let r2p = Point { x:7, y: 3 };
                let mut r2: Region<FakeFeature> = Incremental::init(r2p);

                r1.merge(&r2);
            }

            it "should merge features" {
                assert_eq!(r1.features.merged, 1);
            }
        }
    }
}
