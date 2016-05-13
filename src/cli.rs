extern crate nprs;
extern crate stopwatch;

use std::env;

use stopwatch::Stopwatch;

use nprs::image;
use nprs::extract::cser;

fn main() {
    assert!(env::args().count() == 2, "usage: nprs-cli <file name>");

    if let Some(file_name) = env::args().nth(1) {
        let img = image::io::load_from_file(&file_name).unwrap();

        let trace_config = cser::TraceConfig {
            enabled: false,
            out_dir: "trace"
        };

        let sw = Stopwatch::start_new();
        let _: Vec<cser::Region> = cser::detect_regions(&img, trace_config);

        println!("region detection took {}ms", sw.elapsed_ms());
    }
}
