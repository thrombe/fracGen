
use super::math;
use super::img;

use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::distributions::{Uniform, Distribution};

// use rand_chacha; // i think chacha was faster than from_entropy()
// use rand::distributions::{Standard, Open01};

use std::thread;
use std::sync::{Mutex, Arc};

#[inline(always)]
fn eq(zx: f64, zy: f64, cx: f64, cy: f64) -> (f64, f64) {
    (zx*zx-zy*zy + cx, 2.0*zx*zy + cy)
}

pub fn buddhabrot() { // output work in a image sized channel (crossbeam channel is very much faster), 'works' in arc mutex vector
    let width: u32 = 3000;
    let height: u32 = 3000;
    let cores = 8;
    let iterations: u32 = 100_000;
    let trajectories: u32 = 1000_000;
    let bailout_val_sq: f64 = 4.0;
    let ignore_first_n_iterations = 0; // dont color first few points of every trajectory
    let min_iteration_threshold = 2; // dont accept the trajectory if it has less points
    let (xfrom, xto, yfrom, yto) = math::xyrange(2.0, 0.0, 0.0);
    let colmap = math::map_range(0.0, (iterations as f64).log(2.0)*2.0, 0.0, 255.0);

    // setting up some variables
    let randoff = Uniform::new(-2.0, 2.0);
    let rng = StdRng::from_entropy();

    // some stuff for multi-threading
    let board = vec![vec![0u32; width as usize]; height as usize]; // communal board
    let board = Arc::new(Mutex::new(board));
    let work_per_thread = trajectories/cores;
    let leftover_work = trajectories - work_per_thread*cores;

    for i in 0..cores {
        let board = Arc::clone(&board);
        let mut rng = rng.clone();
        // let mut rng = StdRng::from_entropy();
        // let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(676);
        
        // create some stuff per thread cuz the threads need their own stuff. children cant share stuff. smh spoiled kids
        let xmap = math::map_range(xfrom, xto, 0.0, width as f64);
        let ymap = math::map_range(yfrom, yto, 0.0, height as f64);

        let mut process = move || {
            // let mut rng = rand::thread_rng();
            let mut j = 0;
            let mut self_board = vec![vec![0u32; width as usize]; height as usize];
            while j < work_per_thread {
            
                let mut z: (f64, f64); // convert into struct + try the use of '*' and stuff (idk how)
                let cx = randoff.sample(&mut rng);
                let cy = randoff.sample(&mut rng);
                let mut zx = cx;
                let mut zy = cy;
                
                let mut traj: Vec<(usize, usize)> = Vec::with_capacity(iterations as usize);
                let mut index: (i32, i32);
                for i in 0..iterations {
                    index = (xmap.map(zx).round() as i32, ymap.map(zy).round() as i32);
                    if (index.0 > 0) && (index.1 > 0) &&
                       (index.0 < width as i32) && (index.1 < height as i32) &&
                       (i >= ignore_first_n_iterations) {
                        traj.push((index.0 as usize, index.1 as usize));
                    }
                    if zx*zx+zy*zy > bailout_val_sq {
                        // let mut board = board.lock().unwrap();
                        if i < min_iteration_threshold {break}
                        for (x, y) in traj {
                            self_board[y][x] += 1;
                        }
                        j += 1;
                        break
                    }
                    z = eq(zx, zy, cx, cy);
                    zx = z.0;
                    zy = z.1;
                }
            }
            let mut board = board.lock().unwrap();
            for y in 0..(height as usize) {
                for x in 0..(width as usize) {
                    board[y][x] += self_board[y][x];
                }
            }
        };
        if i == cores-1 { // running last one on main thread and rest on children threads
            #[allow(unused_variables)]
            let work_per_thread = work_per_thread + leftover_work; // should be captured by the closure
            process();
            break
        } else {
            thread::spawn(process);
        }
    }

    let board = board.lock().unwrap();
    // let mut max = 0;
    // let mut i: u32;
    // for y in 0..(height as usize) {
    //     for x in 0..(width as usize) {
    //         i = board[y][x];
    //         if i > max {max = i}
    //     }
    // }
    // shift the max input range up somehow, is max needed?
    // if max isnt needed, shift colmap up
    // also, another func can be used while setting pixel (for color) (for the value that goes in colmap)
    // let colmap = math::map_range(0.0, (max as f64).log(2.0)*2.0, 0.0, 255.0);

    let mut img = img::new_img(width, height);
    for y in 0..(height as usize) {
        for x in 0..(width as usize) {
            img::set(&mut img, x as u32, y as u32, 0.0, colmap.map(board[y][x] as f64), 0.0)
        }
    }
    img::dump_img(img);
}
