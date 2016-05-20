pub mod structures;
pub mod extractor;
pub mod cser;

mod region_detector;

pub use self::region_detector::{ExtremalRegion, RegionDetector};
