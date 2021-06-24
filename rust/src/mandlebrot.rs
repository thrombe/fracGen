
use super::math;
use super::img;
use std::f64::consts::{PI, FRAC_PI_2};
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::distributions::{Uniform, Distribution};

#[inline(always)]
fn eq(zx: f64, zy: f64, cx: f64, cy: f64) -> (f64, f64) {
    (zx*zx-zy*zy+cx, 2.0*zx*zy + cy)
}

pub fn mandlebrot() {
    let width: u32 = 2000;
    let height: u32 = 2000;

    let samples = 4;
    let iterations: u32 = 1000;
    let (xfrom, xto, yfrom, yto) = math::xyrange(-6.0, -0.74571890570893210, -0.11624642707064532);
    let bailout_val_sq: f64 = 4.0;

    let xmap = math::map_range(0.0, width as f64, xfrom, xto);
    let ymap = math::map_range(0.0, height as f64, yfrom, yto);
    let dovmap = math::map_range(0.0, (iterations as f64)/69.0f64, 0.0, FRAC_PI_2);
    let pix_half_width = (xto-xfrom)/(2.0*width as f64);
    let sampf64 = samples as f64;
    let mut rng = StdRng::from_entropy();
    let randoff = Uniform::from(-pix_half_width..pix_half_width);

    let mut img = img::new_img(width, height);
    for y in 0..height {
        for x in 0..width {
            let mut z: (f64, f64);
            let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);
            for _ in 0..samples {
                let cx = xmap.map(x as f64) + randoff.sample(&mut rng);
                let cy = ymap.map(y as f64) + randoff.sample(&mut rng);
                let mut zx = cx;
                let mut zy = cy;
                for i in 0..iterations {
                    z = eq(zx, zy, cx, cy);
                    zx = z.0;
                    zy = z.1;
                    if zx*zx+zy*zy > bailout_val_sq {
                        col_scheme1(&mut r, &mut g, &mut b, i, &dovmap);
                        break
                    }
                }
            }
            img::set(&mut img, x, y, r/sampf64, g/sampf64, b/sampf64)
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
    let mut tmp: f64;
    
    tmp = 255.0*(it-PI*(0.5+0.1666666667)).cos();
    if tmp < 0.0 {tmp = 0.0}
    *r += tmp;

    tmp = 255.0*it.cos();
    if tmp < 0.0 {tmp = 0.0}
    *g += tmp;
    
    tmp = 255.0*(it+PI*(0.5+0.1666666667)).cos();
    if tmp < 0.0 {tmp = 0.0}
    *b += tmp;
}