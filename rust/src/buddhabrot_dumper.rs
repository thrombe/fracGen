
use std::time;
use super::img;
use super::math;
use super::vec4d::Vec4d;
        
struct Options {
    height: i32,
    width: i32,
    iterations: usize,
    input_str: String,
    colmap: math::MapRange,
    selected_colmap: u32,
    color_swap: u8,
    min_color_value: f64,
    max_color_value: f64,
    color_method: u8,
    color_method_mod_off: Vec4d,
    color_vecs: Vec<Vec4d>,
}

/// helps you analyse the board and create multiple images from the generated points.
/// mainly useful to change the brightness slightly/to figure out how what values to choose to get good images
pub fn infini_dump(board: &std::vec::Vec<std::vec::Vec<u16>>, height: i32, width: i32, iterations: usize) {
    let now = time::Instant::now();
    
    let help_text = "help -> this message\ndump | d -> dump image\nq -> quit\nstat -> prints stat\ncolor_method - choose coloring method\ncolmap -> select from a few pre-defined maps\ncolor_swap - choose colors of the fractal\nmincol - set the minimum color value in colmap\n2.5(number) -> the value from board that maps to 255 (chopped at end)";
    let bad_input = "input not understood";
    let colmap_options = "choose from the following maps\n0(default if err) - colmap()\n1 - colmap(sqrt)\n2 - colmap(log)";
    let cholor_options = "0(default) - rgb\n1 - rbg\n2 - gbr\n3 - brg";
    let color_method_options = "0 - overflow\n1 - mod\n2 - lerp";

    let mut op = Options {
        height: height,
        width: width,
        iterations: iterations,
        input_str: String::new(),
        colmap: math::MapRange::new(0.0, (iterations as f64).log(2.0)*2.5, 0.0, 255.0),
        selected_colmap: 0,
        color_swap: 0,
        min_color_value: 0.0,
        max_color_value: (iterations as f64).log(2.0)*2.5,
        color_method: 0,
        color_method_mod_off: Vec4d::new(15.0, 48.0, 63.0, 0.0),
        color_vecs: vec![
            Vec4d::new(0.0, 0.0, 0.0, 0.0),
            Vec4d::new(200.0, 0.0, 200.0, 0.0), // better visible in linear (1)
            Vec4d::new(180.0, 30.0, 190.0, 0.0),
            Vec4d::new(140.0, 80.0, 190.0, 0.0), // sqrt - (3)
            Vec4d::new(80.0, 160.0, 255.0, 0.0), // log - (4)
            Vec4d::new(20.0, 235.0, 255.0, 0.0),
        ],
    };
    
    println!("{}\n", help_text);
    loop {
        match input(&mut op.input_str) {
            "help" => println!("{}\n", help_text),
            "q" => {
                println!("ui time - {:?}", now.elapsed());
                return
            }
            "stat" => stat(board, &op),
            "dump" | "d" => dump(board, &op),
            "colmap" => {
                println!("{}", colmap_options);
                match input(&mut op.input_str).parse() {
                    Ok(val) => op.selected_colmap = val,
                    Err(_) => {
                        println!("{}", bad_input);
                        continue
                    }
                }
                println!("");
            },
            "color_method" => {
                println!("{}", color_method_options);
                match input(&mut op.input_str).parse() {
                    Ok(val) => op.color_method = val,
                    Err(_) => {
                        println!("{}", bad_input);
                        continue
                    }
                }
                if op.color_method == 2 {op.color_swap = 0}
                println!("");  
            },
            "color_swap" => {
                println!("{}", cholor_options);
                match input(&mut op.input_str).parse() {
                    Ok(val) => op.color_swap = val,
                    Err(_) => {
                        println!("{}", bad_input);
                        continue
                    }
                }
                println!("");
            },
            "mincol" => {
                match input(&mut op.input_str).parse() {
                    Ok(val) => op.min_color_value = val,
                    Err(_) => {
                        println!("{}\n", bad_input);
                        continue
                    },
                }
                println!("");
                op.colmap = math::MapRange::new(op.min_color_value, op.max_color_value, 0.0, 255.0); // linear map
            },
            "color_vecs" => {
                
            },
            _ => {
                match op.input_str[0..(op.input_str.len()-1)].parse() {
                    Ok(val) => op.max_color_value = val,
                    Err(_) => {
                        println!("{}\n", bad_input);
                        continue
                    },
                }
                println!("");
                op.colmap = math::MapRange::new(op.min_color_value, op.max_color_value, 0.0, 255.0); // linear map
            },
        }
    }
}

fn submit_color(color: u16, op: &Options) -> Vec4d {
    let clor: f64;
    match op.selected_colmap { // applying the funcs to the hit value
        1 => clor = op.colmap.map((color as f64).sqrt()),
        2 => clor = op.colmap.map((color as f64).log(2.0)),
        _ => clor = op.colmap.map(color as f64),
    }

    let mut clor_vec = Vec4d::new(0.0, 0.0, 0.0, 0.0);

    if op.color_method == 0 { // overflow version
        clor_vec.x = clor;
        if clor > 255.0 {clor_vec.y = clor - 255.0} else {clor_vec.y = 0.0}
        if clor > 511.0 {clor_vec.z = clor - 511.0} else {clor_vec.z = 0.0}
    }
    if op.color_method == 1 { // mod version
        clor_vec.x = (((clor + op.color_method_mod_off.x) as u32)%255) as f64;
        clor_vec.y = ((((clor + op.color_method_mod_off.y) as u32)%511)/2) as f64;
        clor_vec.z = ((((clor + op.color_method_mod_off.z) as u32)%1023)/4) as f64;
    }
    if op.color_method == 2 { // lerp version
        let mut t: f64 = clor/255.0;
        let intervals = op.color_vecs.len()-1;
        t = t*intervals as f64;
        let mut index = t.floor() as usize; // gif
        if index < 1 {index = 1}
        if index > intervals {index = intervals}
        clor_vec = op.color_vecs[index].lerp(op.color_vecs[index-1], t.floor()); // lerping
        // if (y < 1550) && (y > 1450) && (x == y) {println!("{}--{:?}", t, color)}
    }

    match op.color_swap { // swapping the color channels
        1 => Vec4d::new(clor_vec.x, clor_vec.z, clor_vec.y, 0.0), // r b g
        2 => Vec4d::new(clor_vec.y, clor_vec.z, clor_vec.x, 0.0), // g b r
        3 => Vec4d::new(clor_vec.z, clor_vec.x, clor_vec.y, 0.0), // b r g
        _ => clor_vec, // r g b
    }
}

fn stat(board: &std::vec::Vec<std::vec::Vec<u16>>, op: &Options) {
    let mut max = 0;
    let mut average = 0.0;
    // let mut mode = 0; // its gonna be very low. find a better average
    let mut i: u16;
    for y in 0..(op.height as usize) {
        for x in 0..(op.width as usize) {
            i = board[y][x];
            if i > max {max = i}
            average += i as f64;
        }
    }
    average = average/((op.width*op.height) as f64);
    println!("max = {} \naverage = {}\n", max, average)
}

fn input(input_str: &mut String) -> &str {
    input_str.clear();
    std::io::stdin().read_line(input_str).unwrap();
    &input_str[0..(input_str.len()-1)]
}

fn dump(board: &std::vec::Vec<std::vec::Vec<u16>>, op: &Options) {
    let mut img = img::new_img(op.width as u32, op.height as u32);
    for y in 0..(op.height as usize) {
        for x in 0..(op.width as usize) {
            let color = submit_color(board[y][x], op);
            img::set_u8(&mut img, x as u32, y as u32,
                color.x as u8,
                color.y as u8,
                color.z as u8,
            );
        }
    }
    img::dump_img(img);
    println!("image dumped \n")
}


