use complex_gifs::gifs::*;
use num_complex::Complex64;
use std::env;
use std::f64::consts::PI;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Please provide the output directory in the first command line argument.");

    let fs = [
        |z: Complex64| Complex64::new(z.norm().ln(), z.arg()),
        |z: Complex64| {
            let arg = z.arg();
            let arg = if arg > 0.0 { arg } else { arg + 2.0 * PI };

            Complex64::new(z.norm().ln(), arg)
        },
    ];

    for (i, f) in fs.iter().enumerate() {
        create_contour_loop_image(
            &ImageParameters {
                path: format!("{path}/log_{}.gif", i + 1),
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
                angle_width: 4,
                frames: 200,
            },
            &LoopContourParameters {
                contour_spacing: 1.0,
                contour_width: 30,
                contour_color: [0, 0, 0],
            },
            f,
        );
    }
}
