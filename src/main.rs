mod params_cmd;
mod create_img;

extern crate num;
use num::Complex;
use std::io::Write;
use crate::create_img::{render, write_img};
use crate::params_cmd::{check_pair, check_complex};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        writeln!(std::io::stderr(),
                 "Usage: mandelbrot FILENAME PIXELS SUPGA INFDR")
            .unwrap();
        writeln!(std::io::stderr(),
                 "Exemple {} mandel.png 1000*750 -1.20,0.35 -1,0.20", args[0])
            .unwrap();
        std::process::exit(1);
    }
    let bords = check_pair(&args[2], '*').expect("Error Size image");
    let super_ga = check_complex(&args[3]).expect("Error check point left");
    let infer_dr = check_complex(&args[4]).expect("Error check point right");
    let mut pixels = vec![0; bords.0 * bords.1];
    render(&mut pixels, bords, super_ga, infer_dr);
    write_img(&args[1], &pixels, bords).expect("Error write PNG")
}

/*#[allow(dead_code)]
fn complex_square_add_loop(mut c: Complex<f64>) {
    let mut z = Complex { re:0.0, im:0.0};
    loop {
        z = z * z + c
    }
}*/
///Escape_time
///
/// Function for calculate if the complex is in the mandelbrot ensemble for n iteration (limit)
fn escape_time(c: Complex<f64>, limit:u32) -> Option<u32>{
    let mut z = Complex { re:0.0, im:0.0};
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

