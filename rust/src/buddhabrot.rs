
use super::math;
use super::img;

use rand::SeedableRng; // for rng
use rand::rngs::StdRng;
use rand::distributions::{Uniform, Distribution};
// use rand_chacha; // i think chacha was faster than from_entropy()
// use rand::distributions::{Standard, Open01};

use std::thread; // for multi-threading
use std::sync::{Mutex, Arc};

pub fn buddhabrot() {
    let width: i32 = 1000;
    let height: i32 = 1000;
    let cores: usize = 7;
    let iterations: usize = 100_000;
    let trajectories: usize = 100_000;
    let bailout_val_sq: f64 = 4.0;
    let ignore_first_n_iterations: usize = 0; // dont color first few points of every trajectory
    let min_iteration_threshold: usize = 2; // dont accept the trajectory if it has less points
    let (xfrom, xto, yfrom, yto) = math::xyrange(2.0, 0.0, 0.0);
    let colmap = math::MapRange::new(0.0, (iterations as f64).log(2.0)*2.5, 0.0, 255.0);

    #[inline(always)]
    fn eq(zx: f64, zy: f64, cx: f64, cy: f64) -> (f64, f64) {
        (zx*zx-zy*zy + cx, 2.0*zx*zy + cy)
    }
    #[inline(always)]
    fn submit_color(colmap: &math::MapRange, color: u16) -> u8 { // this gets chopped into [0, 255]
        colmap.map(color as f64) as u8
        // (color%255) as u8
        // 1/(1+e^-x) or something
    }
    
    // setting up some variables
    let randoff = Uniform::new(-2.0, 2.0);
    let rng = StdRng::from_entropy();

    // some stuff for multi-threading
    let board = vec![vec![0u16; width as usize]; height as usize]; // communal board (is u16 fine here? 65_535)
    let board = Arc::new(Mutex::new(board));
    let work_per_thread = trajectories/cores;
    let leftover_work = trajectories - work_per_thread*cores;
    let mut worker_vec = vec![];

    for i in 0..cores {
        let board = Arc::clone(&board);
        let mut rng = rng.clone();
        // let mut rng = StdRng::from_entropy();
        // let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(676);
        
        // create some stuff per thread cuz the threads need their own stuff. children cant share stuff. smh spoiled kids
        let xmap = math::MapRange::new(xfrom, xto, 0.0, width as f64);
        let ymap = math::MapRange::new(yfrom, yto, 0.0, height as f64);

        let mut process = move || {
            let mut indicator = super::ProgressIndicator::new(work_per_thread);
            // let mut rng = rand::thread_rng();
            let mut j = 0;
            let mut self_board = vec![vec![0u16; width as usize]; height as usize];
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
                       (index.0 < width) && (index.1 < height) &&
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
                if i == 0 {indicator.indicate(j)} // progress indicator
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
            let worker = thread::spawn(process);
            worker_vec.push(worker);
        }
    }

    for worker in worker_vec { // making sure every child thread is dead
        worker.join().unwrap();
    }
    let board = board.lock().unwrap();

    // let mut max = 0;
    // let mut i: u16;
    // for y in 0..(height as usize) {
    //     for x in 0..(width as usize) {
    //         i = board[y][x];
    //         if i > max {max = i}
    //     }
    // }
    // let colmap = math::MapRange::new(0.0, (max as f64)/60.0, 0.0, 255.0);

    let mut img = img::new_img(width as u32, height as u32);
    for y in 0..(height as usize) {
        for x in 0..(width as usize) {
            // img::set(&mut img, x as u32, y as u32, 0.0, submit_color(&colmap, board[y][x]), 0.0);
            img::set_u8(&mut img, x as u32, y as u32, 0, submit_color(&colmap, board[y][x]), 0);
        }
    }
    img::dump_img(img);
}
