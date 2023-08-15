use complex_gifs::gifs::*;
use num_complex::Complex64;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Please provide the output directory in the first command line argument.")
        + "/sin_series.gif";

    let f = |z: Complex64| {
        let mut ans = Complex64::new(0.0, 0.0);
        for k in 0..300 {
            let t = (2.0_f64.powi(k) * z - 1.1_f64.powi(k).ln()).exp();
            if t.is_finite() {
                ans += t;
            }
        }
        if !ans.is_finite() {
            println!("ans not finite!! z was {z:?}");
        }
        ans
    };
    create_contour_gradient_image(
        &ImageParameters {
            path,
            x_start: -1.0,
            x_end: 1.0,
            y_start: -1.0,
            y_end: 1.0,
            width: 2000,
            height: 2000,
        },
        &ContourParameters {
            contour_spacing: 0.3,
            contour_width: 0.03,
            contour_color: [0, 0, 0],
        },
        &GRADIENT,
        f,
    );
}
