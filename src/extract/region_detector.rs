use image::Image;
use structures::{Point, Rect};
use super::cser::Feature;
use super::cser::Trace;

pub trait ExtremalRegion : Sized {
    type F: Feature + Sized;

    fn threshold(&self) -> i32;
    fn points<'a> (&'a self) -> &'a [Point];
    fn bounds(&self) -> Rect;
    fn peaks<'a>(&'a self) -> &'a [(Rect, Self::F)];
    fn weight(&self) -> f32;
    fn feature_vec(&self, v: &mut Vec<f32>);
}

pub trait RegionDetector {
    type Region: ExtremalRegion;
    type Trace: Trace<Self::Region>;

    fn detect(img: &Image<u8>, trace: &mut Self::Trace) -> Vec<Self::Region>;
}
