
use image::{ImageBuffer, RgbImage};
use std::path::Path;

pub fn new_img(width: u32, height: u32) -> ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
    let image: RgbImage = ImageBuffer::new(width, height);
    image
}

// finds what file name is valid
fn file_name() -> String {
    let mut i = 0;
    let mut path: String;
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

pub fn dump_img_mut(img: &mut ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>) {
    img.save(file_name()).unwrap();
}

// set pixels of image
#[inline(always)]
pub fn set(img: &mut ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>, x: u32, y: u32, r: f64, g: f64, b: f64) {
    img.put_pixel(x, y, image::Rgb([r as u8, g as u8, b as u8]))
}

// set pixels of image
#[inline(always)]
pub fn set_u8(img: &mut ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>, x: u32, y: u32, r: u8, g: u8, b: u8) {
    img.put_pixel(x, y, image::Rgb([r, g, b]))
}

#[allow(non_camel_case_types)]
pub struct pix {
    pub x: u32,
    pub y: u32,
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl pix {

    #[inline(always)]
    pub fn pixel_create(x: u32, y: u32, r: f64, g: f64, b: f64) -> pix {
        pix {
            x,
            y,
            r,
            g,
            b,
        }
    }
}