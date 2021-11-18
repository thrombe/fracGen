
use super::math;
use super::img;
use super::vec4d;

#[allow(unused_imports)]
use std::f64::consts::{PI, E};

#[inline(always)]
fn eq(x: f64, y: f64) -> f64 {
    ((x*x+y*y).cos() - x*y/4.0).abs()
    // ((x*x+y*y).cos() - x*y/4.0)
}

#[inline(always)]
fn colfunc(x: f64, colmap: &math::MapRange) -> f64 {
    let val = colmap.map(x.ln()*4.0);
    if val <= 1.0 {0.0} else if val < 2.0 {val - 1.0} else {1.0}
    // let val = colmap.map(x.ln()*2.75);
    // if val <= 1.0 {0.0} else if val < 2.0 {0.0} else {1.0}
    // val
}

pub fn ploteq() {
    let width = 2000;
    let height = 2000;
    let (xfrom, xto, yfrom, yto) = math::xyrange(3.5, 0.0, 0.0);
    let plotcolor = vec4d::Vec4d::new(0.0, 255.0, 0.0, 0.0); // w is useless here
    let bgcolor = vec4d::Vec4d::new(0.0, 0.0, 0.0, 0.0);
    // let antialiasing = false;
    
    let xmap = math::MapRange::new(0.0, width as f64, xfrom, xto);
    let ymap = math::MapRange::new(0.0, height as f64, yfrom, yto);
    let mut board = vec![vec![0.0;width];height];
    
    let mut max = 0.0;
    for y in 0..height {
        for x in 0..width {
            board[y][x] = eq(xmap.map(x as f64), ymap.map(y as f64));
            if board[y][x] > max {max = board[y][x]}
        }
    }
    
    let colmap = math::MapRange::new(max, 0.0, 0.0, 1.0);
    let mut img = img::new_img(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let color = plotcolor.lerp_with_chop(bgcolor, colfunc(board[y][x], &colmap));
            img::set(&mut img, x as u32, y as u32, color.x, color.y, color.z);
        }
    }
    img::dump_img(img);
}