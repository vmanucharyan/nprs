use std::fs::File;
use std::path::Path;

use pd_image;
use pd_image::{GenericImage, DynamicImage, Pixel, ImageResult, ImageError};

use super::pixel as pix;
use super::image::Image;

pub fn load_from_file(file_name: &str) -> pd_image::ImageResult<Image<u8>> {
    let img: DynamicImage = try!(pd_image::open(&Path::new(file_name)));
    let gs_img: DynamicImage = img.grayscale();
    Ok(Image::from_data(
        gs_img.raw_pixels(),
        gs_img.width() as usize,
        gs_img.height() as usize,
    ))
}

pub fn save_to_file<T: pix::ToRgba + Clone + Copy>(file_name: &str, image: &Image<T>) -> ImageResult<()> {
    let mut out_img: DynamicImage = DynamicImage::new_rgb8(image.width() as u32, image.height() as u32);
    for x in 0..image.width() {
        for y in 0..image.height() {
            let rgba = image[(x, y)].to_rgba();
            out_img.put_pixel(x as u32, y as u32, Pixel::from_channels(rgba.r, rgba.g, rgba.b, 0));
        }
    }

    let ref mut fout = try!(File::create(&Path::new(file_name))
        .map_err(|e| ImageError::IoError(e)));

    out_img.save(fout, pd_image::PNG)
}
