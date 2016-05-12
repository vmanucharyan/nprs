use structures::{Point, Rect};
use super::feature::{AspectRatio};
use super::incremental::Incremental;

#[derive(Debug)]
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
}

#[cfg(test)]
mod test {
    pub use extract::cser::Incremental;
    pub use super::*;
    pub use structures::Point;

    describe! region {
        describe! init {
            it "should create Region with one point bounds" {
                let region: Region = Incremental::init(Point { x: 6, y: 3 });
                assert_eq!(region.aspect_ratio(), 1.0f32);
            }

            it "should create Region that contains one point" {
                let region: Region = Incremental::init(Point { x: 6, y: 3 });
                let expected_points = vec![Point { x: 6, y: 3 }];
                assert_eq!(region.points, expected_points);
            }
        }
    }
}
