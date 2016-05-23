use std::cmp;

pub use super::Point;

#[derive(Debug, PartialEq, Eq, Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct Rect(pub Point, pub Point);

impl Rect {
    pub fn expand(&self, o: Rect) -> Rect {
        let x_min = cmp::min(self.0.x, o.0.x);
        let x_max = cmp::max(self.1.x, o.1.x);
        let y_min = cmp::min(self.0.y, o.0.y);
        let y_max = cmp::max(self.1.y, o.1.y);

        Rect(Point { x: x_min, y: y_min }, Point { x: x_max, y: y_max })
    }

    pub fn width(&self) -> i32 {
        self.1.x - self.0.x + 1
    }

    pub fn height(&self) -> i32 {
        self.1.y - self.0.y + 1
    }

    pub fn aspect_ratio(&self) -> f32 {
        (self.width() as f32) / (self.height() as f32)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expand_1() {
        let r1 = Rect(Point { x: 2, y: 2 }, Point { x: 5, y: 5 });
        let r2 = Rect(Point { x: 5, y: 3 }, Point { x: 6, y: 3 });
        let expected = Rect(Point { x: 2, y: 2 }, Point { x: 6, y: 5 });

        let actual = r1.expand(r2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn expand_2() {
        let r1 = Rect(Point { x: 2, y: 2 }, Point { x: 5, y: 5 });
        let r2 = Rect(Point { x: 1, y: 3 }, Point { x: 2, y: 4 });
        let expected = Rect(Point { x: 1, y: 2 }, Point { x: 5, y: 5 });

        let actual = r1.expand(r2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn expand_3() {
        let r1 = Rect(Point { x: 2, y: 2 }, Point { x: 5, y: 5 });
        let r2 = Rect(Point { x: 0, y: 3 }, Point { x: 3, y: 6 });
        let expected = Rect(Point { x: 0, y: 2 }, Point { x: 5, y: 6 });

        let actual = r1.expand(r2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn expand_4() {

        //
        //                    +---+---+
        //                    |6,0|   |
        //                    +-- +---+
        //                    |   |7,1|
        //    +-------+---+---+---+---+
        //    |2,2|   |   |   |
        //    +---------------+
        //    |   |   |   |   |
        //    +---------------+
        //    |   |   |   |5,4|
        //    +---+---+-------+

        // arrange
        let r1 = Rect(Point { x: 2, y: 2 }, Point { x: 5, y: 4 });
        let r2 = Rect(Point { x: 6, y: 0 }, Point { x: 7, y: 1 });

        let expected = Rect(Point { x: 2, y: 0 }, Point { x: 7, y: 4 });

        // act
        let merged = r1.expand(r2);

        // assert
        assert_eq!(expected, merged);
    }

    #[test]
    fn width_1_point() {
        let r = Rect(Point { x: 1, y: 1 }, Point { x: 1, y: 1 });
        assert_eq!(r.width(), 1);
    }

    #[test]
    fn height_1_point() {
        let r = Rect(Point { x: 1, y: 1 }, Point { x: 1, y: 1 });
        assert_eq!(r.height(), 1);
    }

}
