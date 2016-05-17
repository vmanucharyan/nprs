use super::Feature;
use image::Image;
use extract::cser::Incremental;
use structures::{Point, Rect};

#[derive(Debug, Copy, Clone)]
pub struct AspectRatio {
    bounds: Rect
}

impl AspectRatio {
    pub fn value(&self) -> f32 {
        self.bounds.aspect_ratio()
    }

    pub fn from_bounds(r: Rect) -> AspectRatio {
        AspectRatio { bounds: r }
    }
}

impl Incremental for AspectRatio {
    fn init(p: Point) -> AspectRatio {
        AspectRatio {
            bounds: Rect(p, p)
        }
    }

    fn increment(&mut self, p: Point, _: &Image<u8>) {
        self.bounds = self.bounds.expand(Rect(p, p))
    }

    fn merge(&mut self, o: &AspectRatio) {
        self.bounds = self.bounds.expand(o.bounds)
    }
}

impl Feature for AspectRatio {
    fn value(&self, out: &mut Vec<f32>) {
        out.push((self.bounds.width() as f32) / (self.bounds.height() as f32));
    }
}

#[cfg(test)]
mod test {
    pub use image::Image;
    pub use extract::cser::Incremental;
    pub use structures::{Point, Rect};
    pub use super::AspectRatio;

    describe! aspect_ratio {
        describe! init {
            before_each {
                let ar: AspectRatio = AspectRatio::init(Point { x: 6, y: 3 });
            }

            it "should create aspect ratio feature with value `1`" {
                assert_eq!(ar.value(), 1.0f32);
            }

            it "should create aspect ratio with bounds rect containing 1 point"  {
                let expected_rect = Rect(Point { x: 6, y: 3 }, Point { x: 6, y: 3 });
                assert_eq!(ar.bounds, expected_rect);
            }
        }

        describe! increment {
            before_each {
                let mut ar = AspectRatio::from_bounds(Rect(
                    Point { x: 2, y: 2 },
                    Point { x: 5, y: 4 }
                ));
                let img: Image<u8> = Image::from_data(vec![], 0, 0);
            }

            it "should expand correctly when new point is added 1" {

                //
                //    +-------+---+---+
                //    |2,2|   |   |   |
                //    +-------------------+
                //    |   |   |   |   |6,3|
                //    +-------------------+
                //    |   |   |   |5,4|
                //    +---+---+-------+

                let expected_rect = Rect(
                    Point { x: 2, y: 2 },
                    Point { x: 6, y: 4 }
                );

                ar.increment(Point{ x: 6, y: 3 }, &img);

                assert_eq!(ar.bounds, expected_rect);
                assert_eq!(ar.value(), 5.0f32 / 3.0f32);
            }

            it "should expand correctly when new point is added 2" {

                //      +-------+---+---+
                //      |2,2|   |   |   |
                //      +---------------+
                //      |   |   |   |   |
                //      +---------------+
                //      |   |   |   |5,4|
                //  +-------+---+-------+
                //  |1,5|
                //  +---+

                let expected_rect = Rect(
                    Point { x: 1, y: 2 },
                    Point { x: 5, y: 5 }
                );

                ar.increment(Point{ x: 1, y: 5 }, &img);

                assert_eq!(ar.bounds, expected_rect);
                assert_eq!(ar.value(), 5.0f32 / 4.0f32);
            }
        }

        describe! merge {
            before_each {
                let mut ar = AspectRatio::from_bounds(Rect(
                    Point { x: 2, y: 2 },
                    Point { x: 5, y: 4 }
                ));
                let img: Image<u8> = Image::from_data(vec![], 0, 0);
            }

            it "should merge bounds 1" {

                //           +-------+---+---+
                //           |2,2|   |   |   |
                //   +---+---X---+-----------+
                //   |0,3|   |   |   |   |   |
                //   +-----------+-----------+
                //   |   |   |   |   |   |5,4|
                //   +-------X---X---+-------+
                //   |   |   |2,5|
                //   +-------+---+

                let ar2 = AspectRatio::from_bounds(Rect(
                    Point { x: 0, y: 3 },
                    Point { x: 0, y: 5 }
                ));

                let expected_bounds = Rect(
                    Point { x: 0, y: 2 },
                    Point { x: 5, y: 5 }
                );

                ar.merge(&ar2);

                assert_eq!(ar.bounds, expected_bounds);
                assert_eq!(ar.value(), 6.0f32 / 4.0f32);
            }

            it "should merge bounds 2" {

                //                    +-------+
                //                    |6,0|   |
                //                    +-------+
                //                    |   |7,1|
                //    +-------+---+---+-------+
                //    |2,2|   |   |   |
                //    +---------------+
                //    |   |   |   |   |
                //    +---------------+
                //    |   |   |   |5,4|
                //    +---+---+-------+

                let ar2 = AspectRatio::from_bounds(Rect(
                    Point { x: 6, y: 0 },
                    Point { x: 7, y: 1 }
                ));

                let expected_bounds = Rect(
                    Point { x: 2, y: 0 },
                    Point { x: 7, y: 4 }
                );

                ar.merge(&ar2);

                assert_eq!(ar.bounds, expected_bounds);
                assert_eq!(ar.value(), 6.0f32 / 5.0f32);
            }
        }
    }
}
