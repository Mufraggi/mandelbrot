use num::Complex;

/// function to convert a comlex to pixel
/// 'bords' is one paire of the size off the image height and width
fn pixel_to_point(bords:(usize, usize),
                  pixel:(usize, usize),
                  super_ga: Complex<f64>,
                  infer_dr:Complex<f64>) -> Complex<f64> {
    let (width, height) = (infer_dr.re - super_ga.re , super_ga.im - infer_dr.im);

    Complex {
        re: super_ga.re + pixel.0 as f64 * width / bords.0 as f64,
        im: super_ga.im - pixel.1 as f64 * height / bords.1 as f64
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100,100),
                              (25,75),
                              Complex{ re:-1.0, im: 1.0 },
                              Complex{ re: 1.0, im: -1.0 }
    ), Complex{re:-0.5, im: -0.5});
}