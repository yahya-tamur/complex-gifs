use complex_gifs::gifs::*;
use num_complex::Complex64;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Please provide the output directory in the first command line argument.")
        + "/z_color.gif";

    let f = |z: Complex64| z;
    create_gradient_image(
        &ImageParameters {
            path,
            x_start: -1.0,
            x_end: 1.0,
            y_start: -1.0,
            y_end: 1.0,
            width: 500,
            height: 500,
        },
        &GRADIENT,
        f,
    );
}
