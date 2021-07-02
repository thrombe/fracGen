
use super::math;
use super::vec4d::Vec4d;
use super::img;
use super::progress_indicator::ProgressIndicator;

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
    let cores: usize = 7;
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

use std::time;
/// helps you analyse the board and create multiple images from the generated points.
/// mainly useful to change the brightness slightly/to figure out how what values to choose to get good images
fn infini_dump(board: &std::vec::Vec<std::vec::Vec<u16>>, height: i32, width: i32, iterations: usize) {
    let now = time::Instant::now();
    
    fn stat(board: &std::vec::Vec<std::vec::Vec<u16>>, height: i32, width: i32) {
        let mut max = 0;
        let mut average = 0.0;
        // let mut mode = 0; // its gonna be very low. find a better average
        let mut i: u16;
        for y in 0..(height as usize) {
            for x in 0..(width as usize) {
                i = board[y][x];
                if i > max {max = i}
                average += i as f64;
            }
        }
        average = average/((width*height) as f64);
        println!("max = {} \naverage = {}\n", max, average)
    }
    
    fn dump(board: &std::vec::Vec<std::vec::Vec<u16>>, colmap: &math::MapRange,
    cholors: u8, height: i32, width: i32, func: u32, iterations: usize) {
        let mut img = img::new_img(width as u32, height as u32);
        for y in 0..(height as usize) {
            for x in 0..(width as usize) {
                img::set_u8(&mut img, x as u32, y as u32,
                    submit_color(colmap, board[y][x], func, iterations, 0, cholors), // dont call thrice if want some optimisations here
                    submit_color(colmap, board[y][x], func, iterations, 1, cholors),
                    submit_color(colmap, board[y][x], func, iterations, 2, cholors),
                );
            }
        }
        img::dump_img(img);
        println!("image dumped \n")
    }
    
    fn submit_color(colmap: &math::MapRange, color: u16, func: u32, iterations: usize, mut rgb: u8, cholors: u8) -> u8 {
        match cholors {
            1 => if rgb == 1 {rgb = 2} else if rgb == 2 {rgb = 1} else if rgb == 2 {rgb = 2}, // r b g
            2 => if rgb == 1 {rgb = 0} else if rgb == 2 {rgb = 1} else if rgb == 0 {rgb = 2}, // g b r
            3 => if rgb == 2 {rgb = 0} else if rgb == 0 {rgb = 1} else if rgb == 1 {rgb = 2}, // b r g
            _ => (), // r g b
        }
        let clor: f64;
        match func {
            1 => clor = colmap.map((color as f64).sqrt()),
            2 => clor = colmap.map((color as f64).log(2.0)),
            3 => clor = 255.0*((color as f64)/(iterations as f64)).sqrt(), // dosent really work, like ever (this needs big values of min_iteration_threshold)
            _ => clor = colmap.map(color as f64),
        }
        match rgb { // matching for mapping rgb one after the other
            0 => return clor as u8,
            1 => if clor > 255.0 {return (clor - 255.0) as u8} else {return 0}
            2 => if clor > 511.0 {return (clor - 511.0) as u8} else {return 0}
            _ => panic!(),
        }
        // match rgb { // the mod version
        //     0 => return ((clor as u32 + 15)%255) as u8,
        //     1 => return (((clor as u32 + 34)%511)/2) as u8,
        //     2 => return (((clor as u32 + 45)%1023)/4) as u8,
        //     _ => panic!(),
        // }
    }
    
    let help_text = "help -> this message\ndump | d -> dump image\nq -> quit\nstat -> prints stat\ncolmap -> select from a few pre-defined maps\ncholors - choose colors of the fractal\nmincol - set the minimum color value in colmap\n2.5(number) -> the value from board that maps to 255 (chopped at end)";
    let bad_input = "input not understood";
    let colmap_options = "choose from the following maps\n0(default if err) - colmap()\n1 - colmap(sqrt)\n2 - colmap(log)\n3 - sqrt(hit/limit)*255";
    let cholor_options = "0(default) - rgb\n1 - rbg\n2 - gbr\n3 - brg";

    let mut input_str: String = String::new();
    let mut colmap: math::MapRange = math::MapRange::new(0.0, (iterations as f64).log(2.0)*2.5, 0.0, 255.0);
    let mut selected_colmap: u32 = 0;
    let mut cholors: u8 = 0;
    let mut min_color_value: f64 = 0.0;
    let mut max_color_value: f64 = (iterations as f64).log(2.0)*2.5;
    
    println!("{}\n", help_text);
    loop {
        match input(&mut input_str) {
            "help" => println!("{}\n", help_text),
            "q" => {
                println!("ui time - {:?}", now.elapsed());
                return
            }
            "stat" => stat(board, height, width),
            "dump" | "d" => dump(board, &colmap, cholors, height, width, selected_colmap, iterations),
            "colmap" => {
                println!("{}", colmap_options);
                match input(&mut input_str).parse() {
                    Ok(val) => selected_colmap = val,
                    Err(_) => {
                        println!("{}", bad_input);
                        continue
                    }
                }
                println!("");
            },
            "cholors" => {
                println!("{}", cholor_options);
                match input(&mut input_str).parse() {
                    Ok(val) => cholors = val,
                    Err(_) => {
                        println!("{}", bad_input);
                        continue
                    }
                }
                println!("");
            },
            "mincol" => {
                match input(&mut input_str).parse() {
                    Ok(val) => min_color_value = val,
                    Err(_) => {
                        println!("{}\n", bad_input);
                        continue
                    },
                }
                println!("");
                colmap = math::MapRange::new(min_color_value, max_color_value, 0.0, 255.0); // linear map
            },
            _ => {
                match input_str[0..(input_str.len()-1)].parse() {
                    Ok(val) => max_color_value = val,
                    Err(_) => {
                        println!("{}\n", bad_input);
                        continue
                    },
                }
                println!("");
                colmap = math::MapRange::new(min_color_value, max_color_value, 0.0, 255.0); // linear map
            },
        }
    }
}

fn input(input_str: &mut String) -> &str {
    input_str.clear();
    std::io::stdin().read_line(input_str).unwrap();
    &input_str[0..(input_str.len()-1)]
}
