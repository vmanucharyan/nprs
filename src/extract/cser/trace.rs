use std::fs;
use std::hash::{Hash, Hasher, SipHasher};

use image;
use image::Image;
use image::pixel::{ToLuma, ToRgba, Rgba};

use extract::cser::ExtremalRegion;

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

pub trait Trace {
    fn step<A: ExtremalRegion>(&self, num: i32, all_regions: &[A], reg_img: &Image<Option<usize>>);
    fn result<A: ExtremalRegion>(&self, all_regions: &[A], reg_img: &Image<Option<usize>>);
}

pub struct PrintTrace;

impl Trace for PrintTrace {
    fn step<A: ExtremalRegion>(&self, num: i32, regions: &[A], _: &Image<Option<usize>>) {
        println!("step {}: found {} regions", num, regions.len());
    }

    fn result<A: ExtremalRegion>(&self, regions: &[A], _: &Image<Option<usize>>) {
        println!("Finished.");
        println!("found {} regions", regions.len());
        let sum_peaks: usize = regions.iter().map(|reg| reg.peaks().len()).fold(0, |a, b| a + b);
        println!("total number of peaks: {}", sum_peaks);
    }
}

pub struct EmptyTrace;

impl Trace for EmptyTrace {
    fn step<A: ExtremalRegion>(&self, _: i32, _: &[A], _: &Image<Option<usize>>) {}
    fn result<A: ExtremalRegion>(&self, _: &[A], _: &Image<Option<usize>>) {}
}

pub struct FullTrace<'a> {
    pub path: &'a str
}

impl<'a> Trace for FullTrace<'a> {
    fn step<A: ExtremalRegion>(&self, num: i32, _: &[A], reg_img: &Image<Option<usize>>) {
        if num % 10 == 0 {
            let path = format!("{}/step_{}", self.path, num);
            let _ = fs::create_dir_all(&path);
            let r = image::io::save_to_file(&format!("{}/reg_image.png", &path), &reg_img);
            debug_assert!(r.is_ok());
        }
    }

    fn result<A: ExtremalRegion>(&self, regions: &[A], _: &Image<Option<usize>>) {
        println!("{}", regions.len());
    }
}
