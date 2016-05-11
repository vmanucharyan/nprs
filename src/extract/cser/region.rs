use structures::Point;
use super::feature::{Feature, AspectRatio};

pub struct Region {
    aspect_ratio: AspectRatio,
}

impl Region {
    pub fn init(p: Point) -> Region {
        Region {
            aspect_ratio: Feature::init(p)
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio.value()
    }
}

#[cfg(test)]
mod test {
    pub use super::*;
    pub use structures::Point;

    describe! region {
        describe! init {
            it "should create Region with one point" {
                let region: Region = Region::init(Point { x: 6, y: 3 });
                assert_eq!(region.aspect_ratio(), 1.0f32);
            }
        }
    }
}
