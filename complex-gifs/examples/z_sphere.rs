use complex_gifs::gifs::*;
use num_complex::Complex64;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Please provide the output directory in the first command line argument.")
        + "/z_sphere.gif";

    let f = |z: Complex64| z;

    create_sphere_contour_loop_image(
        &ImageParameters {
            path,
            x_start: -1.0,
            x_end: 1.0,
            y_start: -1.0,
            y_end: 1.0,
            width: 1000,
            height: 1000,
        },
        &LoopParameters {
            argument_color: [100, 0, 0],
            background_color: [255, 255, 255],
            angle_width: 1,
            frames: 200,
        },
        &LoopContourParameters {
            contour_spacing: 0.1,
            contour_width: 50,
            contour_color: [0, 0, 0],
        },
        &SphereParameters {
            theta: 1.0,
            phi: -0.3,
            sphere_color: [50, 100, 200],
        },
        f,
    );
}
