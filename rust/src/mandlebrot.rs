
use super::math;
use super::img;
use std::f64::consts::{PI, FRAC_PI_2};

#[inline(always)]
fn eq(zx: f64, zy: f64, cx: f64, cy: f64) -> (f64, f64) {
    (zx*zx-zy*zy+cx, 2.0*zx*zy + cy)
}

pub fn mandlebrot() {
    let width: u32 = 2000;
    let height: u32 = 2000;

    let iterations: u32 = 1000;
    let (xfrom, xto, yfrom, yto) = math::xyrange(-6.0, -0.74571890570893210, -0.11624642707064532);

    let xmap = math::map_range(0.0, width as f64, xfrom, xto);
    let ymap = math::map_range(0.0, height as f64, yfrom, yto);
    let dovmap = math::map_range(0.0, (iterations as f64)/69.0f64, 0.0, FRAC_PI_2);

    let mut img = img::new_img(width, height);
    for y in 0..height {
        for x in 0..width {
            let mut z = (0.0, 0.0);
            let mut zx = xmap.map(x as f64);
            let mut zy = ymap.map(y as f64);
            let cx = zx;
            let cy = zy;
            let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);
            for i in 0..iterations {
                z = eq(zx, zy, cx, cy);
                zx = z.0;
                zy = z.1;
                if zx*zx+zy*zy > 4.0 {
                    col_scheme1(&mut r, &mut g, &mut b, i, &dovmap);
                    break
                }
            }
            img::set(&mut img, x, y, r, g, b)
        }
        if y % ((height as f64)/100.0) as u32 == 0 {
            println!("{}% done", (y as f64)*100.0/(height as f64))
        }
    }
    img::dump_img(img);
}

#[inline(always)]
fn col_scheme1(r: &mut f64, g: &mut f64, b: &mut f64, i: u32, dovmap: &math::MapRange) {
    let it = dovmap.map(i as f64);
    *r = 255.0*(it-PI*(0.5+0.1666666667)).cos();
    *g = 255.0*it.cos();
    *b = 255.0*(it+PI*(0.5+0.1666666667)).cos();
    if *r < 0.0 {*r = 0.0}
    if *g < 0.0 {*g = 0.0}
    if *b < 0.0 {*b = 0.0}
}