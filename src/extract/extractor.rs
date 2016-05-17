use image::Image;
use extract::structures::NumberPlate;

pub trait ExtractPlate {
    fn extract<T>(_: &Image<u8>) -> Vec<NumberPlate> {
        return vec![];
    }
}

pub struct PlateExtractor;
