use image::Image;
use extract::structures::NumberPlate;

pub trait ExtractPlate {
    fn extract(img: Image) -> Vec<NumberPlate> {
        return vec![];
    }
}

pub struct PlateExtractor {
}
