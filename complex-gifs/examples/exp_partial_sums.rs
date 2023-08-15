use complex_gifs::gifs::*;
use num_complex::Complex64;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Please provide the output directory in the first command line argument.");

    for n in [5, 10, 20] {
        let f = |z: Complex64| {
            let mut ans = Complex64::new(1.0, 0.0);

            let mut term = Complex64::new(1.0, 0.0);

            for k in 1..=n {
                term *= z / (k as f64);
                ans += term;
            }

            ans
        };

        create_contour_loop_image(
            &ImageParameters {
                path: format!("{path}/exp_sum_to_{}.gif", n),
                x_start: -10.0,
                x_end: 10.0,
                y_start: -10.0,
                y_end: 10.0,
                width: 700,
                height: 700,
            },
            &LoopParameters {
                argument_color: [100, 0, 0],
                background_color: [50, 100, 200],
                angle_width: 10,
                frames: 200,
            },
            &LoopContourParameters {
                contour_spacing: 10.0,
                contour_width: 30,
                contour_color: [0, 0, 0],
            },
            f,
        );
    }
}
