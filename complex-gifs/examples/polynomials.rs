use complex_gifs::gifs::*;
use num_complex::Complex64;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Please provide the output directory in the first command line argument.");

    let fs = [
        |z: Complex64| z * z,
        |z: Complex64| z * z + 2.0,
        |z: Complex64| z * z * z,
        |z: Complex64| z * z * z * z * z * z * z,
        |z: Complex64| {
            let i = Complex64::i();
            (z * z * z * z / (z - 2.0) / (z + 2.0) / (z - 2.0 * i) / (z + 2.0 * i)).powi(5)
        },
        |z: Complex64| {
            let i = Complex64::i();
            0.0001 * ((z - i).powi(2)) * ((z - 4.0 * i).powi(3)) * ((z - 2.0).powi(5))
                / (z + 1.0 * i)
        },
        |z: Complex64| 100.0 / z / z / z,
    ];

    for (i, f) in fs.iter().enumerate() {
        create_contour_loop_image(
            &ImageParameters {
                path: format!("{path}/poly_{}.gif", i + 1),
                x_start: -5.0,
                x_end: 5.0,
                y_start: -5.0,
                y_end: 5.0,
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
                contour_spacing: 10.0,
                contour_width: 30,
                contour_color: [0, 0, 0],
            },
            f,
        );
    }
}
