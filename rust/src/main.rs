#![allow(dead_code)]
#![allow(non_snake_case)]

use std::time;
mod img;
mod mandlebrot;
mod buddhabrot;
mod buddhabrot_dumper;
mod math;
mod vec4d;
mod progress_indicator;

mod mandlebrot_concurrency_testing;

fn main() {
    let now = time::Instant::now();

    mandlebrot::mandlebrot();
    // buddhabrot::buddhabrot();
    // ima();
    // mandlebrot_concurrency_testing::tesb();
    
    println!("{:?}", now.elapsed());
}
