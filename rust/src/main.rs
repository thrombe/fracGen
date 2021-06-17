
mod img;
mod mandlebrot;
mod math;

fn main() {
    ima()
}

fn ima() {
    let width: u32 = 255;
    let height: u32 = 255;

    let mut img = img::new_img(width, height);
    for y in 0..width {
        for x in 0..height {
            img::set(&mut img, x, y, 255.0-x as f64, y as f64, x as f64);
        }
    }

    img::dump_img(img)
}