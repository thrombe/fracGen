#![allow(dead_code)]
#![allow(non_snake_case)]

use std::time;
mod img;
mod mandlebrot;
mod math;
mod buddhabrot;

mod mandlebrot_concurrency_testing;

fn main() {
    let now = time::Instant::now();

    mandlebrot::mandlebrot();
    // buddhabrot::buddhabrot();
    // ima();
    // mandlebrot_concurrency_testing::tesb();
    
    println!("{:?}", now.elapsed());
}

fn ima() {
    let width: u32 = 255;
    let height: u32 = 255;

    let mut img = img::new_img(width, height);
    for y in 0..width {
        for x in 0..height {
            img::set(&mut img, x, y, 255.0-x as f64, y as f64, x as f64);
        }
    }

    img::dump_img(img)
}

use std::io::Write; // for progress indicator
pub struct ProgressIndicator {
    oneperc: usize,
    current: usize,
    ended: bool,
}
impl ProgressIndicator {
    // total is the total amount of work done by the worker
    pub fn new(total: usize) -> ProgressIndicator {
        ProgressIndicator {
            oneperc: total/100,
            current: 0,
            ended: false,
        }
    }

    // prog is the current work done by the worker, should end at total
    #[inline(always)]
    pub fn indicate(&mut self, mut prog: usize) {
        if self.ended {return}
        if prog % self.oneperc == 0 {
            prog = prog/self.oneperc;
            if self.current != prog {
                self.current = prog;
                if prog != 0 {print!("\x08\x08\x08")}
                match prog {
                    0..=9 => print!("0{}%", prog),
                    10..=99 => print!("{}%", prog),
                    _ => {
                        println!("");
                        self.ended = true;
                        return            
                    },
                }
                std::io::stdout().flush().unwrap(); // needs std::io::Write
            }
        }
    }
}