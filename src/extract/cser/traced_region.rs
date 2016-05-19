use structures::{Point, Rect};
use super::{Feature, ExtremalRegion, Incremental, Region};

#[derive(Clone)]
pub struct TracedRegion<F: Feature, R: ExtremalRegion<F>> {
    region: R,
    prev_regions: Vec<TracedRegion<F, R>>
}

impl<F: Feature, R: ExtremalRegion<F>> ExtremalRegion<F> for TracedRegion<F, R> {
    fn points<'a> (&'a self) -> &'a [Point] {
        &self.region.points()
    }

    fn weight(&self) -> f32 {
        self.region.weight()
    }

    fn bounds(&self) -> Rect {
        self.region.bounds()
    }

    fn peaks<'a> (&'a self) -> &'a [(Rect, F)] {
        &self.region.peaks()
    }
}

impl<F: Feature, R: ExtremalRegion<F>> Incremental for TracedRegion<F, R> {
    fn init(p: Point, reg_idx: usize) -> Self {
        TracedRegion {
            region: Incremental::init(p, reg_idx)
        }
    }

    fn increment(&mut self, p: Point,  img: &Image<u8>, reg_img: &Image<Option<usize>>) {

    }

    fn merge(&mut self, other: &Self, img: &Image<u8>, reg_img: &Image<Option<usize>>) {

    }
}
