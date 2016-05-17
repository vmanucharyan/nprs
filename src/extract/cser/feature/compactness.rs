use extract::cser::Incremental;
use structures::Point;
use image::Image;

use super::Feature;

#[derive(Debug, Copy, Clone)]
pub struct Compactness {
    perimeter: f32,
    area: f32,
}

impl Incremental for Compactness {
    fn init(p: Point) -> Self {
        Compactness {
            perimeter: 1f32,
            area: 1f32
        }
    }

    fn increment(&mut self, p: Point) {
        let neighbors = vec![
            Point { x: -1, y:  0 },
            Point { x:  1, y:  0 },
            Point { x:  0, y:  1 },
            Point { x:  0, y: -1 }
        ];
    }

    fn merge(&mut self, _: &Self) { }
}

impl Feature for Compactness {
    fn value(&self, out: &mut Vec<f32>) {
        out.push(self.perimeter / self.area);
    }
}
