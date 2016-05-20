use std::fs;
use std::hash::{Hash, Hasher, SipHasher};
use image;
use image::Image;
use image::pixel::{ToLuma, ToRgba, Rgba};
use structures::{Point, Rect};
use super::{ExtremalRegion, Incremental, Feature};

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

pub trait Trace<R: ExtremalRegion> {
    fn step(&self, num: i32, all_regions: &[R], reg_img: &Image<Option<usize>>);
    fn result(&self, all_regions: &[R], reg_img: &Image<Option<usize>>);
}

pub struct PrintTrace;

impl<R: ExtremalRegion> Trace<R> for PrintTrace {
    fn step(&self, num: i32, regions: &[R], _: &Image<Option<usize>>) {
        println!("step {}: found {} regions", num, regions.len());
    }

    fn result(&self, regions: &[R], _: &Image<Option<usize>>) {
        println!("Finished.");
        println!("found {} regions", regions.len());
        let sum_peaks: usize = regions.iter().map(|reg| reg.peaks().len()).fold(0, |a, b| a + b);
        println!("total number of peaks: {}", sum_peaks);
    }
}

pub struct EmptyTrace;

impl<R: ExtremalRegion> Trace<R> for EmptyTrace {
    fn step(&self, _: i32, _: &[R], _: &Image<Option<usize>>) {}
    fn result(&self, _: &[R], _: &Image<Option<usize>>) {}
}

pub struct FullTrace<'a, R: ExtremalRegion + Clone> {
    pub path: &'a str,
    regions: Vec<TracedRegion<R>>,
    regions_map: Vec<Vec<i32>>
}

impl<'a, R: ExtremalRegion + Clone> FullTrace<'a, R> {
    pub fn new(path: &'a str, image_width: usize, image_height: usize) -> Self {
        FullTrace {
            path: path,
            regions: vec![],
            regions_map: vec![vec![0;image_width]; image_height]
        }
    }
}

impl<'a, R: ExtremalRegion + Clone> Trace<TracedRegion<R>> for FullTrace<'a, R> {
    fn step(&self, num: i32, _: &[TracedRegion<R>], reg_img: &Image<Option<usize>>) {
        if num % 10 == 0 {
            let path = format!("{}", self.path);
            let _ = fs::create_dir_all(&path);
            let r = image::io::save_to_file(&format!("{}/{}.png", &path, num), &reg_img);
            debug_assert!(r.is_ok());
        }
    }

    fn result(&self, regions: &[TracedRegion<R>], _: &Image<Option<usize>>) {
        for r in regions.iter() {

        }
    }
}

#[derive(Clone)]
pub struct TracedRegion<R: ExtremalRegion + Clone> {
    region: R,
    prev_regions: Vec<RegionSnapshot<R::F>>
}

#[derive(Clone)]
pub struct RegionSnapshot<F: Feature> {
    feature: F,
    bounds: Rect
}

impl<R: ExtremalRegion + Clone> ExtremalRegion for TracedRegion<R> {
    type F = R::F;

    fn points<'a> (&'a self) -> &'a [Point] {
        &self.region.points()
    }

    fn weight(&self) -> f32 {
        self.region.weight()
    }

    fn bounds(&self) -> Rect {
        self.region.bounds()
    }

    fn peaks<'a> (&'a self) -> &'a [(Rect, R::F)] {
        &self.region.peaks()
    }
}

impl<R: ExtremalRegion + Incremental + Clone> Incremental for TracedRegion<R> {
    fn init(p: Point, reg_idx: usize) -> Self {
        TracedRegion {
            region: Incremental::init(p, reg_idx),
            prev_regions: vec![]
        }
    }

    fn increment(&mut self, p: Point,  img: &Image<u8>, reg_img: &Image<Option<usize>>) {
        self.region.increment(p, img, reg_img);
    }

    fn merge(&mut self, other: &Self, img: &Image<u8>, reg_img: &Image<Option<usize>>) {
        self.region.merge(&other.region, img, reg_img);
    }
}
