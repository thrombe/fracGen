
use super::math;
use super::img;
use super::progress_indicator::ProgressIndicator;
use super::buddhabrot_dumper::infini_dump;

// use rand::SeedableRng; // for rng
// use rand::rngs::StdRng;
use rand::distributions::{Uniform, Distribution};
// use rand_chacha; // i think chacha was faster than from_entropy()
// use rand::distributions::{Standard, Open01};

use std::thread; // for multi-threading
use std::sync::{Mutex, Arc};

pub fn buddhabrot() { // 3.5 for the good 5k image(10^5, 10^7), (1.5(its a tid bit dark), 10k)
    let width: i32 = 3000;
    let height: i32 = 3000;
    let cores: usize = 4;
    let iterations: usize = 100_000;
    let trajectories: usize = 100_000; // should be linearly proportional to the generation time
    let min_iteration_threshold: usize = 20; // dont accept the trajectory if it has less points (not for anti)
    let ignore_first_n_iterations: usize = 15; // dont color first few points of every trajectory
    let bailout_val_sq: f64 = 4.0;
    let anti: bool = false; // if true, includes trajectories even if they lie in the mandlebrot set
    let mut early_bailout: bool = true; // make this false if the equation is not for mandlebrot set (anti turns this off)
    let (xfrom, xto, yfrom, yto) = math::xyrange(2.0, 0.0, 0.0);
    let infini: bool = true;
    let colmap = math::MapRange::new(0.0, (iterations as f64).log(2.0)*2.5, 0.0, 255.0); // only if infini is false

    #[inline(always)]
    fn eq(zx: f64, zy: f64, cx: f64, cy: f64) -> (f64, f64) {
        (zx*zx-zy*zy + cx, 2.0*zx*zy + cy)
    }
    #[inline(always)]
    fn submit_color(colmap: &math::MapRange, color: u16) -> u8 { // this gets chopped into [0, 255]
        colmap.map(color as f64) as u8
        // (color%255) as u8
        // ((colmap.map(color as f64) as u16)%255) as u8
        // 255.0/(1.0+e^-color/scale) - 0.5 or something
    }
    
    // some stuff for multi-threading
    let board = vec![vec![0u16; width as usize]; height as usize]; // communal board (is u16 fine here? 65_535)
    let board = Arc::new(Mutex::new(board));
    let work_per_thread = trajectories/cores;
    let leftover_work = trajectories - work_per_thread*cores;
    let mut worker_vec = vec![];
    
    const ISHTOP: u16 = !1;
    if anti {early_bailout = false}

    #[inline(always)]
    fn add_traj(traj: &Vec<(u16, u16)>,
     self_board: &mut std::vec::Vec<std::vec::Vec<u16>>, j: &mut usize) {
       for (x, y) in traj {
           if *x == ISHTOP {break}
           self_board[*y as usize][*x as usize] += 1;
       }
       *j += 1;
    }

    for core in 0..cores {
        let board = Arc::clone(&board);

        // create some stuff per thread cuz the threads need their own stuff. children cant share stuff. smh spoiled kids
        let xmap = math::MapRange::new(xfrom, xto, 0.0, width as f64);
        let ymap = math::MapRange::new(yfrom, yto, 0.0, height as f64);

        let work_per_thread = if core == cores-1 {work_per_thread + leftover_work} else {work_per_thread};
        
        let process = move || {
            // dont clone rngs, results in same random no. generation
            let randoff = Uniform::new(-2.0, 2.0);
            // let mut rng = StdRng::from_entropy();
            // let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(698776*core as u64); // seed should be differrent per thread
            let mut rng = rand::thread_rng();

            let mut indicator = ProgressIndicator::new(work_per_thread);
            let mut j = 0;
            let mut self_board = vec![vec![0u16; width as usize]; height as usize];
            let mut traj = vec![(0u16, 0u16); iterations];
            let mut index: (i32, i32);

            while j < work_per_thread {
            
                let mut z: (f64, f64); // convert into struct + try the use of '*' and stuff (idk how)
                let cx = randoff.sample(&mut rng);
                let cy = randoff.sample(&mut rng);
                let mut zx = cx;
                let mut zy = cy;
                
                if early_bailout && {
                    let x = cx-0.25;
                    let q = x*x+cy*cy;
                    ((q+x/2.0)*(q+x/2.0)-q/4.0 < 0.0) || (q-0.0625 < 0.0)
                } {continue}
                for i in 0..iterations {
                    index = (xmap.map(zx).round() as i32, ymap.map(zy).round() as i32);
                    if (index.0 > 0) && (index.1 > 0) && // if inside image, include it
                       (index.0 < width) && (index.1 < height) &&
                       (i >= ignore_first_n_iterations) {
                        traj[i] = (index.0 as u16, index.1 as u16);
                    }
                    if zx*zx+zy*zy > bailout_val_sq {
                        if anti || (i < min_iteration_threshold) {break}
                        if i < iterations-1 {traj[i+1] = (ISHTOP, 0)}
                        add_traj(&traj, &mut self_board, &mut j);
                        break
                    }
                    z = eq(zx, zy, cx, cy);
                    zx = z.0;
                    zy = z.1;
                }
                if anti {add_traj(&traj, &mut self_board, &mut j)}
                if core == cores-1 {indicator.indicate(j)} // progress indicator
            }
            let mut board = board.lock().unwrap();
            for y in 0..(height as usize) {
                for x in 0..(width as usize) {
                    board[y][x] += self_board[y][x];
                }
            }
        };
        if core == cores-1 { // running last one on main thread and rest on children threads
            process();
        } else {
            let worker = thread::spawn(process);
            worker_vec.push(worker);
        }
    }

    for worker in worker_vec { // making sure every child thread is dead
        worker.join().unwrap();
    }
    let board = board.lock().unwrap();

    if infini {
        infini_dump(&board, height, width, iterations);
        return
    }

    let mut img = img::new_img(width as u32, height as u32);
    for y in 0..(height as usize) {
        for x in 0..(width as usize) {
            img::set_u8(&mut img, x as u32, y as u32, 0, submit_color(&colmap, board[y][x]), 0);
        }
    }
    img::dump_img(img);
}