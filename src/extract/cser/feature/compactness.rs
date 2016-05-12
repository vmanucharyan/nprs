use structures::Point;
use image::Image;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Compactness<'a> {
    perimeter: f32,
    area: f32,
    image: & 'a Image<u8>
}

impl<'a> Incremental for Compactness<'a> {
    fn init(p: Point) -> Self {
        Compactness {
            perimeter: 1,
            area: 1,
            image: self.image
        }
    }

    fn increment(p: Point) -> Self {
        let neighbors = vec![
            Point { x: -1, y:  0 },
            Point { x:  1, y:  0 },
            Point { x:  0, y:  1 },
            Point { x:  0, y: -1 }
        ];
    }
}
