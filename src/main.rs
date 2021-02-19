mod params_cmd;
mod create_img;

extern crate num;
use num::Complex;

fn main() {
    println!("Hello, world!");
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

