extern crate nprs;
extern crate stopwatch;

use std::env;
use std::fs;
use std::path::Path;

use stopwatch::Stopwatch;

use nprs::image;
use nprs::extract::cser::feature::{AspectRatio, Compactness, NumHoles, HorizontalCrossings};
use nprs::extract::cser::{FullTrace};
use nprs::extract::cser::{Region, TracedRegion, CserDetector};
use nprs::extract::RegionDetector;

type Features = (AspectRatio, Compactness, HorizontalCrossings, NumHoles);
type Reg = Region<Features>;
type Detector<'a> = CserDetector<TracedRegion<Reg>, FullTrace<'a, Reg>>;

fn main() {
    assert!(env::args().count() == 3, "usage: nprs-trace <file name> <trace file name>");

    if let (Some(file_name), Some(trace_file_name)) = (env::args().nth(1),  env::args().nth(2)) {
        let img = image::io::load_from_file(&file_name).unwrap();

        let sw = Stopwatch::start_new();
        let mut full_trace: FullTrace<Reg> = FullTrace::new("trace", img.width(), img.height());

        Detector::detect(&img, &mut full_trace);

        println!("region detection took {}ms", sw.elapsed_ms());

        let trace_path = Path::new(&trace_file_name);

        trace_path.parent().map(|pp| {
            let _ = fs::create_dir_all(&pp);
        });

        let mut f = fs::File::create(&trace_file_name)
            .unwrap_or_else(|e| panic!("Failed to create file `{:?}`: {:?}", &trace_file_name, e));

        full_trace.write_zipped_json(&mut f)
            .unwrap_or_else(|e| panic!("Failed to write trace as zipped json: {:?}", e));
    }
}
