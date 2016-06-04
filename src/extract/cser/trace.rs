use std::fs;
use std::io::prelude::*;
use std::io;
use std::marker::PhantomData;
use std::hash::{Hash, Hasher, SipHasher};
use std::collections::HashMap;

use rustc_serialize::json;

use flate2::Compression;
use flate2::write::{DeflateEncoder};

use image::Image;
use image::pixel::{ToLuma, ToRgba, Rgba};
use structures::{Point, Rect};
use extract::ExtremalRegion;
use super::{Incremental};

static MAX_THRES_REGS: i32 = 1000000;

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
    fn step(&mut self, num: i32, all_regions: &[R], reg_img: &Image<Option<usize>>);
    fn result(&self, all_regions: &[R], reg_img: &Image<Option<usize>>);
}

pub struct PrintTrace;

impl<R: ExtremalRegion> Trace<R> for PrintTrace {
    fn step(&mut self, num: i32, regions: &[R], _: &Image<Option<usize>>) {
        println!("step {}: found {} regions", num, regions.len());
    }

    fn result(&self, regions: &[R], _: &Image<Option<usize>>) {
        println!("Finished.");
        println!("found {} regions", regions.len());
        let sum_peaks: usize = regions.iter()
            .map(|reg| reg.peaks().len())
            .fold(0, |a, b| a + b);
        println!("total number of peaks: {}", sum_peaks);
    }
}

pub struct EmptyTrace;

impl<R: ExtremalRegion> Trace<R> for EmptyTrace {
    fn step(&mut self, _: i32, _: &[R], _: &Image<Option<usize>>) {}
    fn result(&self, _: &[R], _: &Image<Option<usize>>) {}
}

pub struct FullTrace<'a, R: ExtremalRegion + Clone> {
    pub path: &'a str,
    trace_result: TraceResult,
    r: PhantomData<R>,
    min_region_dims: (i32, i32),
    max_region_dims: (i32, i32)
}

#[derive(Debug)]
pub enum TraceWriteError {
    EncodeError(json::EncoderError),
    CompressErrror(io::Error),
    IoError(io::Error)
}

pub type TraceWriteResult<T> = Result<T, TraceWriteError>;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct TraceResult {
    regions: HashMap<i32, RegionSnapshot>,
    regions_map: Vec<Vec<Vec<i32>>>,
}

impl<'a, R: ExtremalRegion + Clone> FullTrace<'a, R> {
    pub fn new(
        path: &'a str,
        image_width: usize,
        image_height: usize,
        min_region_dims: (i32, i32),
        max_region_dims: (i32, i32)
    ) -> Self {
        FullTrace {
            path: path,
            trace_result: TraceResult {
                regions: HashMap::new(),
                regions_map: vec![vec![vec![];image_width]; image_height],
            },
            r: PhantomData,
            min_region_dims: min_region_dims,
            max_region_dims: max_region_dims
        }
    }

    fn is_good(&self, r: &R) -> bool {
        self.min_region_dims.0 <= r.bounds().width() &&
        self.min_region_dims.1 <= r.bounds().height() &&
        self.max_region_dims.0 >= r.bounds().width() &&
        self.max_region_dims.1 >= r.bounds().height()
    }

    pub fn get_trace_result<'b>(&'b self) -> &'b TraceResult {
        &self.trace_result
    }

    pub fn as_json(&self) -> TraceWriteResult<String> {
        json::encode(&self.trace_result)
            .map_err(|e| TraceWriteError::EncodeError(e))
    }

    pub fn write_json(&self, write: &mut Write) -> TraceWriteResult<()> {
        let json = try!(self.as_json());
        write.write_all(json.as_bytes())
            .map_err(|e| TraceWriteError::IoError(e))
    }

    pub fn write_zipped_json(&self, write: &mut Write) -> TraceWriteResult<()> {
        let json = try!(self.as_json());
        let mut encoder = DeflateEncoder::new(write, Compression::Default);
        encoder.write_all(json.as_bytes())
            .map_err(|e| TraceWriteError::CompressErrror(e))
    }
}

impl<'a, R: ExtremalRegion + Clone> Trace<TracedRegion<R>> for FullTrace<'a, R> {
    fn step(&mut self, num: i32, all_regions: &[TracedRegion<R>], reg_img: &Image<Option<usize>>) {
        if num % 10 == 0 && num < 255 {
            let regs_count = all_regions.len();

            let this_step_regions: Vec<(&TracedRegion<R>, usize)> = all_regions.iter()
                .zip(0 .. regs_count)
                .filter(|r| r.0.threshold() > num - 10 && r.0.threshold() <= num && self.is_good(&r.0.region))
                .collect();

            let it = this_step_regions.iter()
                .map(|x| {
                    let (r, i) = *x;
                    let mut f: Vec<f32> = vec![];
                    r.feature_vec(&mut f);
                    let rs = RegionSnapshot {
                        features: f,
                        bounds: r.bounds(),
                        thres: r.threshold()
                    };
                    (r.threshold() * MAX_THRES_REGS + (i as i32), rs)
                });

            for x in it {
                let (i, r) = x;
                self.trace_result.regions.insert(i, r);
            }

            for x in 0 .. reg_img.width() {
                for y in 0 .. reg_img.height() {
                    if let Some(reg_idx) = reg_img[(x, y)] {
                        let r = &all_regions[reg_idx];
                        if r.threshold() > num - 10 && r.threshold() <= num  && self.is_good(&r.region) {
                            let idx = r.threshold() * MAX_THRES_REGS + (reg_idx as i32);
                            (self.trace_result.regions_map[y][x]).push(idx);
                        }
                    }
                }
            }
        }
    }

    fn result(&self, _: &[TracedRegion<R>], _: &Image<Option<usize>>) {}
}

#[derive(Debug, Clone)]
pub struct TracedRegion<R: ExtremalRegion + Clone> {
    pub region: R
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct RegionSnapshot {
    features: Vec<f32>,
    bounds: Rect,
    thres: i32
}

impl<R: ExtremalRegion + Clone> ExtremalRegion for TracedRegion<R> {
    type F = R::F;

    fn threshold(&self) -> i32 {
        self.region.threshold()
    }

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

    fn feature_vec(&self, v: &mut Vec<f32>) {
        self.region.feature_vec(v);
    }
}

impl<R: ExtremalRegion + Incremental + Clone> Incremental for TracedRegion<R> {
    fn init(p: Point, reg_idx: usize, thres: i32) -> Self {
        TracedRegion {
            region: Incremental::init(p, reg_idx, thres)
        }
    }

    fn increment(&mut self, p: Point, thres: i32, img: &Image<u8>,  reg_img: &Image<Option<usize>>) {
        self.region.increment(p, thres, img, reg_img);
    }

    fn merge(&mut self, other: &Self, thres: i32, img: &Image<u8>, reg_img: &Image<Option<usize>>) {
        self.region.merge(&other.region, thres, img, reg_img);
    }
}
