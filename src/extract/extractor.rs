use image::Image;
use extract::structures::NumberPlate;

pub trait ExtractPlate {
    fn extract<T>(img: &Image) -> Vec<NumberPlate> {
        return vec![];
    }
}

pub struct PlateExtractor {
}
