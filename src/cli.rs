extern crate nprs;
extern crate stopwatch;

use std::env;

use stopwatch::Stopwatch;

use nprs::image;
use nprs::extract::cser::feature::{AspectRatio, Compactness, NumHoles};
use nprs::extract::cser::{FullTrace, PrintTrace, EmptyTrace};
use nprs::extract::cser;

type Features = (AspectRatio, Compactness, NumHoles);

fn main() {
    assert!(env::args().count() == 2, "usage: nprs-cli <file name>");

    if let Some(file_name) = env::args().nth(1) {
        let img = image::io::load_from_file(&file_name).unwrap();

        let sw = Stopwatch::start_new();
        //let full_trace = FullTrace { path: "trace" };
        let trace = EmptyTrace;

        let _: Vec<cser::Region<Features>> = cser::detect_regions(&img, &trace);

        println!("region detection took {}ms", sw.elapsed_ms());
    }
}
