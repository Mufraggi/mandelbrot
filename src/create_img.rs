extern crate  image;

use image::ColorType;
use std::fs::File;
use num::Complex;

use crate::escape_time;
use image::png::PNGEncoder;

/// function to convert a comlex to pixel
/// 'bords' is one paire of the size off the image height and width
pub(crate) fn pixel_to_point(bords:(usize, usize),
                  pixel:(usize, usize),
                  super_ga: Complex<f64>,
                  infer_dr:Complex<f64>) -> Complex<f64> {
    let (width, height) = (infer_dr.re - super_ga.re , super_ga.im - infer_dr.im);

    Complex {
        re: super_ga.re + pixel.0 as f64 * width / bords.0 as f64,
        im: super_ga.im - pixel.1 as f64 * height / bords.1 as f64
    }
}

pub(crate) fn render(pixels: &mut [u8],
          bords:(usize, usize),
          super_ga: Complex<f64>,
          infer_dr:Complex<f64>
) {
    assert!(pixels.len() == bords.0 * bords.1);
    for line in 0..bords.1 {
        for col in 0..bords.0 {
            let point = pixel_to_point(bords, (col, line), super_ga, infer_dr);
            pixels[line * bords.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            }
        }
    }
}

/// write file
///
pub(crate) fn write_img(file_nane: &str, pixels: &[u8], bords: (usize,usize))
    -> Result<(), std::io::Error>
{
    let output = File::create(file_nane)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels,
                   bords.0 as u32,
                   bords.1 as u32,
                   ColorType::Gray(8))?;
    Ok(())
}



#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100,100),
                              (25,75),
                              Complex{ re:-1.0, im: 1.0 },
                              Complex{ re: 1.0, im: -1.0 }
    ), Complex{re:-0.5, im: -0.5});
}