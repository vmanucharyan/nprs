use structures::{Point, Rect};
use super::feature::{AspectRatio};
use super::incremental::Incremental;

#[derive(Debug)]
pub struct Region {
    aspect_ratio: AspectRatio,
    bounds: Rect
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
            bounds: Rect(p, p)
        }
    }

    fn increment(&self, p: Point) -> Region {
        Region {
            aspect_ratio: self.aspect_ratio.increment(p),
            bounds: self.bounds.expand(Rect(p, p))
        }
    }

    fn merge(&self, r: &Region) -> Region {
        Region {
            aspect_ratio: self.aspect_ratio.merge(&r.aspect_ratio),
            bounds: self.bounds.expand(r.bounds)
        }
    }
}

#[cfg(test)]
mod test {
    pub use extract::cser::Incremental;
    pub use super::*;
    pub use structures::Point;

    describe! region {
        describe! init {
            it "should create Region with one point" {
                let region: Region = Incremental::init(Point { x: 6, y: 3 });
                assert_eq!(region.aspect_ratio(), 1.0f32);
            }
        }
    }
}
