use extract::cser::Incremental;
use structures::Point;
use image::Image;

use super::Feature;

#[derive(Debug, Copy, Clone)]
pub struct Compactness {
    perimeter: i32,
    area: i32,
}

impl Incremental for Compactness {
    fn init(_: Point, _: usize, _: i32) -> Self {
        Compactness {
            perimeter: 1,
            area: 1
        }
    }

    fn increment(&mut self, p: Point, _: i32,   img: &Image<u8>,  _: &Image<Option<usize>>) {
        self.area += 1;

        let mut sum = 0;

        let ns = [
            (p.x - 1, p.y), (p.x, p.y - 1),
            (p.x + 1, p.y), (p.x, p.y + 1)
        ];

        let pv = img[(p.x, p.y)];
        for n in ns.iter() {
            let (x, y) = *n;
            if img.inside(x, y) {
                let qv = img[(x, y)];
                if qv <= pv {
                    sum += 1;
                }
            }
        }

        self.perimeter += 4 - sum;
    }

    fn merge(&mut self, other: &Self, _: i32, _: &Image<u8>, _: &Image<Option<usize>>) {
        self.perimeter += other.perimeter;
        self.area += other.area;
    }
}

impl Feature for Compactness {
    fn value(&self, out: &mut Vec<f32>) {
        out.push((self.perimeter as f32) / (self.area as f32));
    }
}
