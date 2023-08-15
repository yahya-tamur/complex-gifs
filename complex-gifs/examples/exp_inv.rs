use complex_gifs::gifs::*;
use num_complex::Complex64;
use std::env;
use std::f64::consts::PI;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Please provide the output directory in the first command line argument.");

    let f = |z: Complex64| (1.0 / z).exp();

    create_contour_loop_image(
        &ImageParameters {
            path: format!("{path}/exp_inv.gif"),
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
            angle_width: 3,
            frames: 200,
        },
        &LoopContourParameters {
            contour_spacing: 0.04,
            contour_width: 30,
            contour_color: [0, 0, 0],
        },
        f,
    );

    let f = |z: Complex64| z.exp();

    create_sphere_contour_loop_image(
        &ImageParameters {
            path: format!("{path}/exp_inv_sphere.gif"),
            x_start: -1.0,
            x_end: 1.0,
            y_start: -1.0,
            y_end: 1.0,
            width: 700,
            height: 700,
        },
        &LoopParameters {
            argument_color: [100, 0, 0],
            background_color: [255, 255, 255],
            angle_width: 3,
            frames: 200,
        },
        &LoopContourParameters {
            contour_spacing: 0.8,
            contour_width: 30,
            contour_color: [0, 0, 0],
        },
        &SphereParameters {
            theta: PI - 1.0,
            phi: 0.0,
            sphere_color: [50, 100, 200],
        },
        f,
    );
}
