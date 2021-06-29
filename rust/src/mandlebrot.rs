
use super::math;
use super::img;
use super::progress_indicator::ProgressIndicator;
use std::f64::consts::{PI, FRAC_PI_2};
use rand::distributions::{Uniform, Distribution};

use std::thread;
use std::sync::{Mutex, Arc};

#[inline(always)]
fn eq(zx: f64, zy: f64, cx: f64, cy: f64) -> (f64, f64) {
    (zx*zx-zy*zy + cx, 2.0*zx*zy + cy)
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

pub fn mandlebrot() { // output work in a image sized channel (crossbeam channel is very much faster), 'works' in arc mutex vector
    let width: usize = 2000;
    let height: usize = 2000;
    let cores: usize = 7;
    let samples: usize = 4;
    let iterations: u32 = 1000;
    let (xfrom, xto, yfrom, yto) = math::xyrange(-6.0, -0.74571890570893210, -0.11624642707064532);
    let dovmap = math::MapRange::new(0.0, (iterations as f64)/69.0f64, 0.0, FRAC_PI_2);
    let bailout_val_sq: f64 = 4.0;

    // setting up some variables
    let sampf64 = samples as f64;
    let randoff = {
        let pix_half_width = (xto-xfrom)/(2.0*width as f64);
        Uniform::new(-pix_half_width, pix_half_width)
    };

    // some stuff for multi-threading
    let work_per_thread = height/cores;
    let leftover_work = height-work_per_thread*cores;
    let img = Arc::new(Mutex::new(img::new_img(width as u32, height as u32))); // workers directly edit image
    let mut worker_vec = vec![];

    for i in 0..cores {
        let img = Arc::clone(&img);
        let dovmap = dovmap.clone();
        let thread_y = i*work_per_thread;
        
        // cloning and creating some stuff cuz the threads need their own stuff. children cant share stuff. smh spoiled kids
        let xmap = math::MapRange::new(0.0, width as f64, xfrom, xto);
        let ymap = math::MapRange::new(0.0, height as f64, yfrom, yto);

        let process = move || {
            let mut rng = rand::thread_rng();

            let mut indicator = ProgressIndicator::new(work_per_thread);
            let mut self_board = vec![vec![(0u8, 0u8, 0u8); width as usize]; work_per_thread as usize];
            for y in 0..work_per_thread {
                for x in 0..width {
                    let mut z: (f64, f64);
                    let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);
                    for _ in 0..samples {
                        let cx = xmap.map(x as f64) + randoff.sample(&mut rng);
                        let cy = ymap.map((y+thread_y) as f64) + randoff.sample(&mut rng);
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
                    self_board[y][x] = ((r/sampf64) as u8, (g/sampf64) as u8, (b/sampf64) as u8);
                }
                if i == 0 {indicator.indicate(y)} // progress indicator
            }
            let mut col: (u8, u8, u8);
            let mut img = img.lock().unwrap();
            for y in 0..work_per_thread {
                for x in 0..width {
                    col = self_board[y][x];
                    img::set_u8(&mut img, x as u32, (y+thread_y) as u32, col.0, col.1, col.2);
                }
            }
        };
        if i == cores-1 { // running last one on main thread and rest on children threads
            #[allow(unused_variables)]
            let work_per_thread = work_per_thread+leftover_work;
            process();
            break
        } else {
            let worker = thread::spawn(process);
            worker_vec.push(worker);
        }
    }
    
    for worker in worker_vec { // making sure every thread is dead
        worker.join().unwrap();
    }

    img::dump_img_mut(&mut img.lock().unwrap());
}
