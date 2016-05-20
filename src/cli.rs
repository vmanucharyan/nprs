extern crate nprs;
extern crate stopwatch;

use std::env;

use stopwatch::Stopwatch;

use nprs::image;
use nprs::extract::cser::feature::{AspectRatio, Compactness, NumHoles, HorizontalCrossings};
use nprs::extract::cser::{FullTrace, EmptyTrace};
use nprs::extract::cser::{Region, TracedRegion, CserDetector};
use nprs::extract::RegionDetector;

type Features = (AspectRatio, Compactness, NumHoles, HorizontalCrossings);
type Reg = Region<Features>;
type Detector<'a> = CserDetector<TracedRegion<Reg>, FullTrace<'a, Reg>>;

fn main() {
    assert!(env::args().count() == 2, "usage: nprs-cli <file name>");

    if let Some(file_name) = env::args().nth(1) {
        let img = image::io::load_from_file(&file_name).unwrap();

        let sw = Stopwatch::start_new();
        let mut full_trace: FullTrace<Reg> = FullTrace::new("trace", img.width(), img.height());
        let trace = EmptyTrace;

        Detector::detect(&img, &mut full_trace);

        println!("region detection took {}ms", sw.elapsed_ms());
    }
}
