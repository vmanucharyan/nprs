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

    fn from_bounds(r: Rect) -> AspectRatio {
        AspectRatio { bounds: r }
    }
}

impl Incremental for AspectRatio {
    fn init(p: Point) -> AspectRatio {
        AspectRatio {
            bounds: Rect(p, p)
        }
    }

    fn increment(&self, p: Point) -> AspectRatio {
        AspectRatio {
            bounds: self.bounds.expand(Rect(p, p))
        }
    }

    fn merge(&self, o: &AspectRatio) -> AspectRatio {
        AspectRatio {
            bounds: self.bounds.expand(o.bounds)
        }
    }
}

#[cfg(test)]
mod test {
    pub use extract::cser::Incremental;
    pub use super::AspectRatio;
    pub use structures::{Point, Rect};

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
                let ar = AspectRatio::from_bounds(Rect(
                    Point { x: 2, y: 2 },
                    Point { x: 5, y: 4 }
                ));
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

                let new_ar = ar.increment(Point{ x: 6, y: 3 });

                assert_eq!(new_ar.bounds, expected_rect);
                assert_eq!(new_ar.value(), 5.0f32 / 3.0f32);
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

                let new_ar = ar.increment(Point{ x: 1, y: 5 });

                assert_eq!(new_ar.bounds, expected_rect);
                assert_eq!(new_ar.value(), 5.0f32 / 4.0f32);
            }
        }

        describe! merge {
            before_each {
                let ar = AspectRatio::from_bounds(Rect(
                    Point { x: 2, y: 2 },
                    Point { x: 5, y: 4 }
                ));
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

                let merged = ar.merge(&ar2);

                assert_eq!(merged.bounds, expected_bounds);
                assert_eq!(merged.value(), 6.0f32 / 4.0f32);
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

                let merged = ar.merge(&ar2);

                assert_eq!(merged.bounds, expected_bounds);
                assert_eq!(merged.value(), 6.0f32 / 5.0f32);
            }
        }
    }
}
