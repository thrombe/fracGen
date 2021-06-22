
use image::{ImageBuffer, RgbImage};
use std::path::Path;

pub fn new_img(width: u32, height: u32) -> ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
    let image: RgbImage = ImageBuffer::new(width, height);
    image
}

// finds what file name is valid
fn file_name() -> String {
    let mut i = 0;
    let mut path = String::new();
    loop {
        path = format!("../images/image{}.png", i);
        if !Path::new(&path).exists() {return path};
        i += 1;
    }
}

// dumps image with a valid filename
pub fn dump_img(img: ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>) {
    img.save(file_name()).unwrap();
}

// set pixels of image
#[inline(always)]
pub fn set(img: &mut ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>, x: u32, y: u32, r: f64, g: f64, b: f64) {
    img.put_pixel(x, y, image::Rgb([r as u8, g as u8, b as u8]))
}
