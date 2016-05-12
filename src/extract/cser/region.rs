use structures::{Point, Rect};
use super::feature::{AspectRatio};
use super::incremental::Incremental;

#[derive(Debug, Clone)]
pub struct Region {
    aspect_ratio: AspectRatio,
    bounds: Rect,
    points: Vec<Point>
}

impl Region {
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio.value()
    }
}

impl Incremental for Region {
    fn init(p: Point) -> Region {
        Region {
            aspect_ratio: AspectRatio::init(p),
            bounds: Rect(p, p),
            points: vec![p]
        }
    }

    fn increment(&mut self, p: Point) {
        self.aspect_ratio.increment(p);
        self.bounds = self.bounds.expand(Rect(p, p));
        self.points.push(p);
    }

    fn merge(&mut self, r: &Region) {
        self.aspect_ratio.merge(&r.aspect_ratio);
        self.bounds = self.bounds.expand(r.bounds);
        self.points.extend_from_slice(&r.points[..]);
    }

    fn points<'a> (&'a self) -> Option<&'a[Point]> {
        Some(&self.points[..])
    }
}

#[cfg(test)]
mod test {
    pub use extract::cser::Incremental;
    pub use super::*;
    pub use structures::{Point, Rect};

    describe! region {
        describe! init {
            before_each {
                let region: Region = Incremental::init(Point { x: 6, y: 3 });
            }

            it "should create Region with one point bounds" {
                assert_eq!(region.aspect_ratio(), 1.0f32);
            }

            it "should create Region that contains one point" {
                let expected_points = vec![Point { x: 6, y: 3 }];
                assert_eq!(region.points, expected_points);
            }
        }

        describe! increment {
            before_each {
                let mut region: Region = Incremental::init(Point { x: 6, y: 3 });
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
        }
    }
}
