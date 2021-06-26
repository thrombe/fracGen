
use super::math;
use super::img;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::distributions::{Uniform, Distribution};

use std::thread;
use std::sync::{Mutex, Arc};

#[inline(always)]
fn eq(zx: f64, zy: f64, cx: f64, cy: f64) -> (f64, f64) {
    (zx*zx-zy*zy + cx, 2.0*zx*zy + cy)
}

pub fn buddhabrot() { // output work in a image sized channel (crossbeam channel is very much faster), 'works' in arc mutex vector
    let width: u32 = 1000;
    let height: u32 = 1000;
    let cores = 4;

    let iterations: u32 = 20;
    let trajectories: u32 = 100_000_000;
    let (xfrom, xto, yfrom, yto) = math::xyrange(2.0, 0.0, 0.0);
    let bailout_val_sq: f64 = 4.0;

    // setting up some variables
    let xmap = math::map_range(xfrom, xto, 0.0, width as f64);
    let ymap = math::map_range(yfrom, yto, 0.0, height as f64);
    // let rng = StdRng::from_entropy();
    let randoff = Uniform::from(-2.0..2.0);

    // some stuff for multi-threading
    let board = vec![vec![0u32; width as usize]; height as usize];
    let board = Arc::new(Mutex::new(board));
    let wpt = trajectories/cores;
    let leftover = trajectories - wpt*cores;

    for i in 0..cores {
        let board = Arc::clone(&board);
        let mut rng = StdRng::from_entropy();
        
        // cloning some stuff cuz the threads need their own stuff. children cant share stuff. smh spoiled kids
        let xmap = xmap.clone();
        let ymap = ymap.clone();

        let mut process = move || {
            let mut j = 0;
            while j < wpt {
                let mut z: (f64, f64);
                let cx = randoff.sample(&mut rng);
                let cy = randoff.sample(&mut rng);
                let mut zx = cx;
                let mut zy = cy;
                let mut traj: Vec<(usize, usize)> = Vec::with_capacity(iterations as usize);
                let mut index: (i32, i32);
                for _ in 0..iterations {
                    index = (xmap.map(zx).round() as i32, ymap.map(zy).round() as i32);
                    if (index.0 > 0) && (index.1 > 0) &&
                       (index.0 < width as i32) && (index.1 < height as i32) {
                        traj.push((index.0 as usize, index.1 as usize));
                    }
                    if zx*zx+zy*zy > bailout_val_sq {
                        let mut board = board.lock().unwrap();
                        for (x, y) in traj {
                            board[y][x] += 1;
                        }
                        j += 1;
                        break
                    }
                    z = eq(zx, zy, cx, cy);
                    zx = z.0;
                    zy = z.1;
                }
            }
        };
        if i == cores-1 { // running last one on main thread and rest on children threads
            let wpt = wpt + leftover;
            process();
            break
        } else {
            thread::spawn(process);
        }
    }

    let board = board.lock().unwrap();
    let mut max = 0;
    let mut i = 0;
    for y in 0..(height as usize) {
        for x in 0..(width as usize) {
            i = board[y][x];
            if i > max {max = i}
        }
    }
    // let colmap = math::map_range(0.0, max as f64, 0.0, 255.0);

    let mut img = img::new_img(width, height);
    for y in 0..(height as usize) {
        for x in 0..(width as usize) {
            img::set(&mut img, x as u32, y as u32, 0.0, (board[y][x]%255) as f64, 0.0)
        }
    }
    img::dump_img(img);
}
