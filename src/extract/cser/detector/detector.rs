use std::hash::{Hash, Hasher, SipHasher};

use image;
use image::Image;
use image::pixel::{ToLuma, ToRgba, Rgba};
use structures::Point;
use extract::cser::{Incremental, HasPoints};

impl ToLuma for Option<usize> {
    fn to_luma(&self) -> u8 {
        if self.is_some() { 255 } else { 0 }
    }
}

impl ToRgba for Option<usize> {
    fn to_rgba(&self) -> Rgba {
        if let Some(val) = *self {
            let mut s = SipHasher::new();
            (val as i32).hash(&mut s);
            Rgba::from_i32(s.finish() as i32)
        }
        else {
            Rgba::from_i32(0i32)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TraceConfig {
    pub enabled: bool,
    pub out_dir: &'static str
}

pub fn detect_regions<T: Incremental + HasPoints + Sized> (image: &Image<u8>, trace: TraceConfig) -> Vec<T> {
    let baskets = hist(image);
    let mut all_regions: Vec<T> = vec![];
    let mut reg_image: Image<Option<usize>> = image.map( |_| None );

    for i in 0..255 {
        let points = &baskets[i];
        for p in points {
            process_point(p.clone(), &mut reg_image, &mut all_regions);
        }
        if trace.enabled {
            if i % 5 == 0 {
                let r = image::io::save_to_file(&format!("{}/step_{}.png", trace.out_dir, i), &reg_image);
                debug_assert!(r.is_ok());
            }
        }
    }

    return all_regions;
}

pub fn process_point<T: Incremental + HasPoints + Sized>(
    p: Point,
    reg_image: &mut Image<Option<usize>>,
    all_regions: &mut Vec<T>) {

    let ns = find_neighbors(&reg_image, p.clone());

    match &ns[..] {
        [] => {
            all_regions.push(Incremental::init(p));
            let idx = all_regions.len() - 1;
            reg_image.set_pixel(p.x, p.y, Some(idx));
        },
        [r_idx] => {
            let r = &mut (all_regions[r_idx]);
            r.increment(p);
            reg_image.set_pixel(p.x, p.y, Some(r_idx));
        },
        [r1_idx, rest..] => {
            all_regions[r1_idx].increment(p);
            for r_idx in rest {
                if let Some((r1, r2)) = index_twice(&mut all_regions[..], r1_idx, *r_idx) {
                    r1.increment(p);
                    reg_image.set_pixel(p.x, p.y, Some(r1_idx));

                    r1.merge(r2);
                    for p in r2.points() {
                        reg_image.set_pixel(p.x, p.y, Some(r1_idx));
                    }
                } else {
                    panic!("failed to get regions - got None");
                }
            }
        }
    }
}

pub fn hist(image: &Image<u8>) -> Vec<Vec<Point>> {
    let mut baskets: Vec<Vec<Point>> = vec![];

    for _ in 0..256 {
        baskets.push(vec![])
    }

    for x in 0..image.width() {
        for y in 0..image.height() {
            let intensity = image[(x, y)];
            baskets[intensity as usize].push(Point { x: (x as i32), y: (y as i32) });
        }
    }

    return baskets;
}

pub fn find_neighbors(reg_image: &Image<Option<usize>>, p: Point) -> Vec<usize> {
    let neighbors = vec![
        Point { x: -1, y: 0 }, Point { x: 1, y: 0 },
        Point { x: 0, y: -1 }, Point { x: 0, y: 1 }
    ];

    let points = neighbors.iter()
        .map(|n| Point { x: p.x - n.x, y: p.y - n.y });

    let inside_points = points.filter(|p| reg_image.inside(p.x, p.y));
    let some_points = inside_points.filter(|p| reg_image[(p.x as usize, p.y as usize)].is_some());
    let mut indexes: Vec<usize> = some_points
        .map(|p| reg_image[(p.x as usize, p.y as usize)].unwrap())
        .collect();

    indexes.sort();
    indexes.dedup();

    return indexes;
}

pub fn index_twice<T>(slc: &mut [T], a: usize, b: usize) -> Option<(&mut T, &mut T)> {
    if a >= slc.len() || b >= slc.len() {
        None
    } else {
        unsafe {
            let ar = &mut *(slc.get_unchecked_mut(a) as *mut _);
            let br = &mut *(slc.get_unchecked_mut(b) as *mut _);
            Some((ar, br))
        }
    }
}
