mod params_cmd;
mod create_img;

extern crate crossbeam;
extern crate num;
use num::Complex;
use std::io::Write;
use crate::create_img::{render, write_img, pixel_to_point };
use crate::params_cmd::{check_pair, check_complex};



fn main() {
    let thread = 8;

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
    let line_by_band = bords.1 / thread + 1;
    let bandes: Vec<&mut [u8]> = pixels.chunks_mut(line_by_band * bords.0).collect();
    crossbeam::scope(|spawner| {
       for (i, bande) in bandes.into_iter().enumerate() {
           let top = line_by_band * i;
           let haute = bande.len() / bords.0;
           let bande_bords = (bords.0, haute);
           let bande_supg = pixel_to_point(
               bords, (0, top), super_ga, infer_dr);
           let bande_inf = pixel_to_point(
               bords, (bords.0, top + haute), super_ga, infer_dr);
           spawner.spawn(move || {
               render(bande, bande_bords,
                                        bande_supg, bande_inf);
           });
       }
    });
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

