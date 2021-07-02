
use std::thread;
use std::sync::mpsc::sync_channel;
use std::sync::{Mutex, Arc};
use crossbeam_channel::bounded as crosschan;

pub fn tesb() {
    
    // conc_mutex();
    // conc_cross();
    // mandle_conc_cross();
    // mandle_conc_mutex();
    // mandle_conc_mutex2();
    // mandle_conc_mutex3();
    // mandle_conc_mutex4();
    mandle_conc_mutex5();
    // mandle_conc_mutex6();

}

pub fn conc_cross() {
    println!("storted");

    let (outchans, outchanr) = crosschan(25); // similar to a go channel
    let (workchans, workchanr) = crosschan(50);

    let works = vec!((2u32, 5u32); 2000);
    thread::spawn(move || { // provide work for workers (it consumes workchans)
        // let mut i = 0;
        for work in works.iter() {
            workchans.send(*work).unwrap();
            // i += 1;
            // if i % 20 == 0 {println!("{}", i)}
        }
    });

    println!("croeating workers");

    let mut worker_vec = vec![];
    for _ in 0..6 {
        let outchan = outchans.clone();
        let workchan = workchanr.clone();
        let worker = thread::spawn(move || {
            loop {
                let work: (u32, u32);
                match workchan.recv() {
                    Ok(worr) => work = worr,
                    Err(_) => break,
                }
                outchan.send((((work.0+work.1) as f64).powf(3.1641).sqrt().powf(1.2189142653).sqrt()) as u32).unwrap();
            }
        });
        worker_vec.push(worker);
    }
    drop(outchans); // makes sure that the channel is closed once threads are done with it

    println!("doing work");

    let mut bork: u32 = 0;
    for work in outchanr { // take credit for the worker's work
        // println!("{}", work);
        bork += work;
    }

    println!("bork: {}", bork);

    for worker in worker_vec { // making sure every thread is dead
        worker.join().unwrap();
        println!("dead");
    }
}

pub fn conc_mutex() {
    println!("storted");

    let (outchans, outchanr) = sync_channel(25); // similar to a go channel

    let works = Arc::new(Mutex::new(vec!((2u32, 5u32); 2000)));

    println!("croeating workers");

    let mut worker_vec = vec![];
    for _ in 0..6 {
        let outchan = outchans.clone();
        let workchan = Arc::clone(&works);
        let worker = thread::spawn(move || {
            loop {
                let work: (u32, u32) = { match workchan.lock().unwrap().pop() {
                    Some(val) => val,
                    None => break,
                }
            };
                outchan.send((((work.0+work.1) as f64).powf(3.1641).sqrt().powf(1.2189142653).sqrt()) as u32).unwrap();
            }
        });
        worker_vec.push(worker);
    }
    drop(outchans); // makes sure that the channel is closed once threads are done with it

    println!("doing work");

    let mut bork: u32 = 0;
    for work in outchanr { // take credit for the worker's work
        // println!("{}", work);
        bork += work;
    }

    println!("bork: {}", bork);

    for worker in worker_vec { // making sure every thread is dead
        worker.join().unwrap();
        println!("dead");
    }
}

// copied from mandlebrot.rs
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
// copied from mandlebrot.rs

pub fn mandle_conc_cross() {
    let width: u32 = 2000;
    let height: u32 = 2000;

    let samples = 4;
    let iterations: u32 = 1000;
    let (xfrom, xto, yfrom, yto) = math::xyrange(-6.0, -0.74571890570893210, -0.11624642707064532);

    let xmap = math::MapRange::new(0.0, width as f64, xfrom, xto);
    let ymap = math::MapRange::new(0.0, height as f64, yfrom, yto);
    let dovmap = math::MapRange::new(0.0, (iterations as f64)/69.0f64, 0.0, FRAC_PI_2);
    let pix_half_width = (xto-xfrom)/(2.0*width as f64);
    let sampf64 = samples as f64;
    let rng = StdRng::from_entropy();
    let randoff = Uniform::from(-pix_half_width..pix_half_width);

    let mut img = img::new_img(width, height);

    println!("storted");

    let (outchans, outchanr) = crosschan(300); // similar to a go channel
    let (workchans, workchanr) = crosschan(50);

    thread::spawn(move || { // provide work for workers (it consumes workchans)
        for y in 0..height {
            workchans.send(y).unwrap();
        }
    });

    println!("croeating workers");

    let mut worker_vec = vec![];
    for _ in 0..3 {
        let outchan = outchans.clone();
        let workchan = workchanr.clone();

        let xmap = xmap.clone();
        let ymap = ymap.clone();
        let dovmap = dovmap.clone();
        let mut rng = rng.clone();

        let worker = thread::spawn(move || {
            let mut y: u32;
            loop {
                match workchan.recv() {
                    Ok(wae) => y = wae,
                    Err(_) => break,
                    }
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
                                if zx*zx+zy*zy > 4.0 {
                                    col_scheme1(&mut r, &mut g, &mut b, i, &dovmap);
                                    break
                                }
                            }
                        }        
                        outchan.send(img::pix::new(x, y, r/sampf64, g/sampf64, b/sampf64)).unwrap();
                    }
                }
        });
        worker_vec.push(worker);
    }
    drop(outchans); // makes sure that the channel is closed once threads are done with it

    println!("doing work");

    for pix in outchanr { // take credit for the worker's work
        img::set(&mut img, pix.x, pix.y, pix.r, pix.g, pix.b);
    }
    img::dump_img(img);

    for worker in worker_vec { // making sure every thread is dead
        worker.join().unwrap();
        println!("dead");
    }
}

pub fn mandle_conc_mutex() {
    let width: u32 = 2000;
    let height: u32 = 2000;

    let samples = 4;
    let iterations: u32 = 1000;
    let (xfrom, xto, yfrom, yto) = math::xyrange(-6.0, -0.74571890570893210, -0.11624642707064532);

    let xmap = math::MapRange::new(0.0, width as f64, xfrom, xto);
    let ymap = math::MapRange::new(0.0, height as f64, yfrom, yto);
    let dovmap = math::MapRange::new(0.0, (iterations as f64)/69.0f64, 0.0, FRAC_PI_2);
    let pix_half_width = (xto-xfrom)/(2.0*width as f64);
    let sampf64 = samples as f64;
    let rng = StdRng::from_entropy();
    let randoff = Uniform::from(-pix_half_width..pix_half_width);

    let mut img = img::new_img(width, height);

    println!("storted");

    let (outchans, outchanr) = crosschan(300); // similar to a go channel

    let mut vec: Vec<u32> = Vec::with_capacity(width as usize);
    for i in 0..width {vec.push(i)}
    let works = Arc::new(Mutex::new(vec));

    println!("croeating workers");

    let mut worker_vec = vec![];
    for _ in 0..3 {
        let outchan = outchans.clone();

        let works = Arc::clone(&works);
        let xmap = xmap.clone();
        let ymap = ymap.clone();
        let dovmap = dovmap.clone();
        let mut rng = rng.clone();

        let worker = thread::spawn(move || {
            let mut y: u32;
            loop {
                match works.lock().unwrap().pop() {
                    Some(val) => y = val,
                    None => break,
                }
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
                            if zx*zx+zy*zy > 4.0 {
                                col_scheme1(&mut r, &mut g, &mut b, i, &dovmap);
                                break
                            }
                        }
                    }        
                    outchan.send(img::pix::new(x, y, r/sampf64, g/sampf64, b/sampf64)).unwrap();
                }
            }
        });
        worker_vec.push(worker);
    }
    drop(outchans); // makes sure that the channel is closed once threads are done with it

    println!("doing work");

    for pix in outchanr { // take credit for the worker's work
        img::set(&mut img, pix.x, pix.y, pix.r, pix.g, pix.b);
    }
    img::dump_img(img);

    for worker in worker_vec { // making sure every thread is dead
        worker.join().unwrap();
        println!("dead");
    }
}

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn mandle_conc_mutex2() { // with works as mutable atomic (or something) vector
    let width: u32 = 2000;
    let height: u32 = 2000;

    let samples = 4;
    let iterations: u32 = 1000;
    let (xfrom, xto, yfrom, yto) = math::xyrange(-6.0, -0.74571890570893210, -0.11624642707064532);

    let xmap = math::MapRange::new(0.0, width as f64, xfrom, xto);
    let ymap = math::MapRange::new(0.0, height as f64, yfrom, yto);
    let dovmap = math::MapRange::new(0.0, (iterations as f64)/69.0f64, 0.0, FRAC_PI_2);
    let pix_half_width = (xto-xfrom)/(2.0*width as f64);
    let sampf64 = samples as f64;
    let rng = StdRng::from_entropy();
    let randoff = Uniform::from(-pix_half_width..pix_half_width);

    let mut img = img::new_img(width, height);

    println!("storted");

    let (outchans, outchanr) = crosschan(300); // similar to a go channel
    // let mut out = Arc::new(Mutex::new(Vec::with_capacity((width*height) as usize)));

    let mut vec: Vec<u32> = Vec::with_capacity(width as usize);
    for i in 0..width {vec.push(i)}
    let works = Arc::new(Mutex::new(vec));

    println!("croeating workers");

    let mut worker_vec = vec![];
    for _ in 0..3 {
        let outchan = outchans.clone();

        let works = Arc::clone(&works);
        let xmap = xmap.clone();
        let ymap = ymap.clone();
        let dovmap = dovmap.clone();
        let mut rng = rng.clone();
        // let out = Arc::clone(&out);

        let worker = thread::spawn(move || {
            let mut y: u32;
            loop {
                match works.lock().unwrap().pop() {
                    Some(val) => y = val,
                    None => break,
                }
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
                            if zx*zx+zy*zy > 4.0 {
                                col_scheme1(&mut r, &mut g, &mut b, i, &dovmap);
                                break
                            }
                        }
                    }
                    // out.lock().unwrap().push(img::pix::new(x, y, r/sampf64, g/sampf64, b/sampf64));
                    outchan.send(img::pix::new(x, y, r/sampf64, g/sampf64, b/sampf64)).unwrap();
                }
            }
        });
        worker_vec.push(worker);
    }
    drop(outchans); // makes sure that the channel is closed once threads are done with it

    println!("doing work");

    for worker in worker_vec { // making sure every thread is dead
        worker.join().unwrap();
        println!("dead");
    }

    for pix in outchanr { // take credit for the worker's work
        img::set(&mut img, pix.x, pix.y, pix.r, pix.g, pix.b);
    }
    img::dump_img(img);
}

#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn mandle_conc_mutex3() { // with both out and works as mutable atomic (or something) vectors
    let width: u32 = 2000;
    let height: u32 = 2000;

    let samples = 4;
    let iterations: u32 = 1000;
    let (xfrom, xto, yfrom, yto) = math::xyrange(-6.0, -0.74571890570893210, -0.11624642707064532);

    let xmap = math::MapRange::new(0.0, width as f64, xfrom, xto);
    let ymap = math::MapRange::new(0.0, height as f64, yfrom, yto);
    let dovmap = math::MapRange::new(0.0, (iterations as f64)/69.0f64, 0.0, FRAC_PI_2);
    let pix_half_width = (xto-xfrom)/(2.0*width as f64);
    let sampf64 = samples as f64;
    let mut rng = StdRng::from_entropy();
    let randoff = Uniform::from(-pix_half_width..pix_half_width);

    let mut img = img::new_img(width, height);

    println!("storted");

    let mut out = Arc::new(Mutex::new(Vec::with_capacity((width*height) as usize)));

    let mut vec: Vec<u32> = Vec::with_capacity(width as usize);
    for i in 0..width {vec.push(i)}
    let works = Arc::new(Mutex::new(vec));

    let works2 = Arc::clone(&works);
    let xmap2 = xmap.clone();
    let ymap2 = ymap.clone();
    let dovmap2 = dovmap.clone();
    let mut rng2 = rng.clone();
    let out2 = Arc::clone(&out);

    let mut process = move || {
        println!("holeo");
        let mut y: u32;
        loop {
            match works2.lock().unwrap().pop() {
                Some(val) => y = val,
                None => break,
            }
            for x in 0..width {
                let mut z: (f64, f64);
                let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);
                for _ in 0..samples {
                    let cx = xmap2.map(x as f64) + randoff.sample(&mut rng2);
                    let cy = ymap2.map(y as f64) + randoff.sample(&mut rng2);
                    let mut zx = cx;
                    let mut zy = cy;
                    for i in 0..iterations {
                        z = eq(zx, zy, cx, cy);
                        zx = z.0;
                        zy = z.1;
                        if zx*zx+zy*zy > 4.0 {
                            col_scheme1(&mut r, &mut g, &mut b, i, &dovmap2);
                            break
                        }
                    }
                }
                out2.lock().unwrap().push(img::pix::new(x, y, r/sampf64, g/sampf64, b/sampf64));
            }
        }
    };

    println!("croeating workers");

    let mut worker_vec = vec![];
    for _ in 0..3 {
        let out2 = Arc::clone(&out);

        let works2 = Arc::clone(&works);
        let xmap2 = xmap.clone();
        let ymap2 = ymap.clone();
        let dovmap2 = dovmap.clone();
        let mut rng2 = rng.clone();
        let process = process.clone();

        let worker = thread::spawn(process);
        worker_vec.push(worker);
    }
    process ();

    println!("doing work");

    for pix in out.lock().unwrap().iter() { // take credit for the worker's work
        img::set(&mut img, pix.x, pix.y, pix.r, pix.g, pix.b);
    }
    img::dump_img(img);

    for worker in worker_vec { // making sure every thread is dead
        worker.join().unwrap();
        println!("dead");
    }
}

#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn mandle_conc_mutex4() { // similar to version 5 but no refactoring
    let width: u32 = 2000;
    let height: u32 = 2000;

    let samples = 4;
    let iterations: u32 = 1000;
    let (xfrom, xto, yfrom, yto) = math::xyrange(-6.0, -0.74571890570893210, -0.11624642707064532);

    let xmap = math::MapRange::new(0.0, width as f64, xfrom, xto);
    let ymap = math::MapRange::new(0.0, height as f64, yfrom, yto);
    let dovmap = math::MapRange::new(0.0, (iterations as f64)/69.0f64, 0.0, FRAC_PI_2);
    let pix_half_width = (xto-xfrom)/(2.0*width as f64);
    let sampf64 = samples as f64;
    let mut rng = StdRng::from_entropy();
    let randoff = Uniform::from(-pix_half_width..pix_half_width);

    let mut img = img::new_img(width, height);

    println!("storted");

    let (outchans, outchanr) = crosschan((width*height) as usize); // similar to a go channel

    let mut vec: Vec<u32> = Vec::with_capacity(width as usize);
    for i in 0..width {vec.push(i)}
    let works = Arc::new(Mutex::new(vec));

    let works2 = Arc::clone(&works);
    let xmap2 = xmap.clone();
    let ymap2 = ymap.clone();
    let dovmap2 = dovmap.clone();
    let mut rng2 = rng.clone();
    let outchan2 = outchans.clone();

    let mut process = move || {
        println!("holeo");
        let mut y: u32;
        loop {
            match works2.lock().unwrap().pop() {
                Some(val) => y = val,
                None => break,
            }
            for x in 0..width {
                let mut z: (f64, f64);
                let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);
                for _ in 0..samples {
                    let cx = xmap2.map(x as f64) + randoff.sample(&mut rng2);
                    let cy = ymap2.map(y as f64) + randoff.sample(&mut rng2);
                    let mut zx = cx;
                    let mut zy = cy;
                    for i in 0..iterations {
                        z = eq(zx, zy, cx, cy);
                        zx = z.0;
                        zy = z.1;
                        if zx*zx+zy*zy > 4.0 {
                            col_scheme1(&mut r, &mut g, &mut b, i, &dovmap2);
                            break
                        }
                    }
                }
                outchan2.send(img::pix::new(x, y, r/sampf64, g/sampf64, b/sampf64)).unwrap();
            }
        }
        drop(outchan2);
    };

    println!("croeating workers");

    let mut worker_vec = vec![];
    for _ in 0..3 {
        let outchan2 = outchans.clone();

        let works2 = Arc::clone(&works);
        let xmap2 = xmap.clone();
        let ymap2 = ymap.clone();
        let dovmap2 = dovmap.clone();
        let mut rng2 = rng.clone();
        let process = process.clone();

        let worker = thread::spawn(process);
        worker_vec.push(worker);
    }
    process ();
    drop(outchans); // makes sure that the channel is closed once threads are done with it

    println!("done work");
    
    for pix in outchanr { // take credit for the worker's work
    img::set(&mut img, pix.x, pix.y, pix.r, pix.g, pix.b);
    }
    img::dump_img(img);

    for worker in worker_vec { // making sure every thread is dead
        worker.join().unwrap();
        println!("dead");
    }
}

// best yet (9.7 sec 4 cores) // channel with big bounds eat a lot of ram (2.6 gigs for 10k*10k image)(maybe)(rough calcs)
pub fn mandle_conc_mutex5() { // output work in a image sized channel (crossbeam channel is very much faster), 'works' in arc mutex vector
    let width: u32 = 2000;
    let height: u32 = 2000;
    let cores = 4;

    let samples = 4;
    let iterations: u32 = 1000;
    let (xfrom, xto, yfrom, yto) = math::xyrange(-6.0, -0.74571890570893210, -0.11624642707064532);
    let dovmap = math::MapRange::new(0.0, (iterations as f64)/69.0f64, 0.0, FRAC_PI_2);
    let bailout_val_sq: f64 = 4.0;

    // setting up some variables
    let xmap = math::MapRange::new(0.0, width as f64, xfrom, xto);
    let ymap = math::MapRange::new(0.0, height as f64, yfrom, yto);
    let pix_half_width = (xto-xfrom)/(2.0*width as f64);
    let sampf64 = samples as f64;
    let rng = StdRng::from_entropy();
    let randoff = Uniform::from(-pix_half_width..pix_half_width);

    // some stuff for multi-threading
    let (outchans, outchanr) = crosschan((width*height) as usize); // workers submit here
    let mut vec: Vec<u32> = Vec::with_capacity(width as usize);
    for i in 0..width {vec.push(i)}
    let works = Arc::new(Mutex::new(vec)); // workers take work from here

    // let mut worker_vec = vec![];
    for i in 0..cores {
        let outchan = outchans.clone();
        let works = Arc::clone(&works);
        let dovmap = dovmap.clone();
        
        // cloning some stuff cuz the threads need their own stuff. children cant share stuff. smh spoiled kids
        let xmap = xmap.clone();
        let ymap = ymap.clone();
        let mut rng = rng.clone();

        let mut process = move || {
            let mut y: u32;
            loop {
                match works.lock().unwrap().pop() { // finding work
                    Some(val) => y = val,
                    None => break,
                }
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
                    outchan.send(img::pix::new(x, y, r/sampf64, g/sampf64, b/sampf64)).unwrap();
                }
            }
        };

        if i == cores-1 { // running last one on main thread and rest on children threads
            process();
            break
        }
        thread::spawn(process);
        // let worker = thread::spawn(process);
        // worker_vec.push(worker);
    }
    drop(outchans); // makes sure that the channel is closed once threads are done with it

    let mut img = img::new_img(width, height);
    for pix in outchanr { // take credit for the worker's work
    img::set(&mut img, pix.x, pix.y, pix.r, pix.g, pix.b);
    }
    img::dump_img(img);

    // for worker in worker_vec { // making sure every thread is dead
    //     worker.join().unwrap();
    // }
}
 // this will be nice if having both, the image and the output channel in ram is too much.
pub fn mandle_conc_mutex6() { // 5th but threads directly modify images
    let width: u32 = 2000;
    let height: u32 = 2000;
    let cores = 4;

    let samples = 4;
    let iterations: u32 = 1000;
    let (xfrom, xto, yfrom, yto) = math::xyrange(-6.0, -0.74571890570893210, -0.11624642707064532);
    let dovmap = math::MapRange::new(0.0, (iterations as f64)/69.0f64, 0.0, FRAC_PI_2);
    let bailout_val_sq: f64 = 4.0;

    // setting up some variables
    let xmap = math::MapRange::new(0.0, width as f64, xfrom, xto);
    let ymap = math::MapRange::new(0.0, height as f64, yfrom, yto);
    let pix_half_width = (xto-xfrom)/(2.0*width as f64);
    let sampf64 = samples as f64;
    let rng = StdRng::from_entropy();
    let randoff = Uniform::from(-pix_half_width..pix_half_width);
    
    // some stuff for multi-threading
    let mut vec: Vec<u32> = Vec::with_capacity(width as usize);
    for i in 0..width {vec.push(i)}
    let works = Arc::new(Mutex::new(vec)); // workers take work from here
    let img = Arc::new(Mutex::new(img::new_img(width, height))); // workers directly edit image

    // let mut worker_vec = vec![];
    for i in 0..cores {
        let works = Arc::clone(&works);
        let img = Arc::clone(&img);
        let dovmap = dovmap.clone();
        
        // cloning some stuff cuz the threads need their own stuff. children cant share stuff. smh spoiled kids
        let xmap = xmap.clone();
        let ymap = ymap.clone();
        let mut rng = rng.clone();

        let mut process = move || {
            let mut y: u32;
            loop {
                match works.lock().unwrap().pop() { // finding work
                    Some(val) => y = val,
                    None => break,
                }
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
                    img::set(&mut img.lock().unwrap(), x, y, r/sampf64, g/sampf64, b/sampf64);
                }
            }
        };

        if i == cores-1 { // running last one on main thread and rest on children threads
            process();
            break
        }
        thread::spawn(process);
        // let worker = thread::spawn(process);
        // worker_vec.push(worker);
    }
    img::dump_img_mut(&mut img.lock().unwrap());

    // for worker in worker_vec { // making sure every thread is dead
    //     worker.join().unwrap();
    // }
}