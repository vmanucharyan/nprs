use std::fs;
use std::hash::{Hash, Hasher, SipHasher};

use image;
use image::Image;
use image::pixel::{ToLuma, ToRgba, Rgba};

use super::{ExtremalRegion, Feature};

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

pub trait Trace<F: Feature, R: ExtremalRegion<F>> {
    fn step(&self, num: i32, all_regions: &[R], reg_img: &Image<Option<usize>>);
    fn result(&self, all_regions: &[R], reg_img: &Image<Option<usize>>);
}

pub struct PrintTrace;

impl<F: Feature, R: ExtremalRegion<F>> Trace<F, R> for PrintTrace {
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

impl<F: Feature, R: ExtremalRegion<F>> Trace<F, R> for EmptyTrace {
    fn step(&self, _: i32, _: &[R], _: &Image<Option<usize>>) {}
    fn result(&self, _: &[R], _: &Image<Option<usize>>) {}
}

pub struct FullTrace<'a> {
    pub path: &'a str
}

impl<'a, F: Feature, R: ExtremalRegion<F>> Trace<F, R> for FullTrace<'a> {
    fn step(&self, num: i32, _: &[R], reg_img: &Image<Option<usize>>) {
        if num % 10 == 0 {
            let path = format!("{}/step_{}", self.path, num);
            let _ = fs::create_dir_all(&path);
            let r = image::io::save_to_file(&format!("{}/reg_image.png", &path), &reg_img);
            debug_assert!(r.is_ok());
        }
    }

    fn result(&self, _: &[R], _: &Image<Option<usize>>) {}
}
