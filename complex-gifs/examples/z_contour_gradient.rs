use complex_gifs::gifs::*;
use num_complex::Complex64;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Please provide the output directory in the first command line argument.")
        + "/z_contour_gradient.gif";

    let gradient: Vec<u8> = (0..255).map(|x| [255, x, x]).flatten().collect();

    let f = |z: Complex64| z;

    create_contour_gradient_image(
        &ImageParameters {
            path,
            x_start: -1.0,
            x_end: 1.0,
            y_start: -1.0,
            y_end: 1.0,
            width: 1000,
            height: 1000,
        },
        &ContourParameters {
            contour_spacing: 0.1,
            contour_width: 0.01,
            contour_color: [0, 0, 0],
        },
        &gradient,
        f,
    );
}
