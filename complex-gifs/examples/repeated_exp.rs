use complex_gifs::gifs::*;
use num_complex::Complex64;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Please provide the output directory in the first command line argument.");

    for n in 2..=5 {
        let f = |z: Complex64| {
            let mut ans = z;
            for _ in 0..n {
                ans = ans.exp();
            }
            ans
        };

        create_contour_loop_image(
            &ImageParameters {
                path: format!("{path}/exp_{n}.gif"),
                x_start: -10.0,
                x_end: 10.0,
                y_start: -10.0,
                y_end: 10.0,
                width: 1000,
                height: 1000,
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
