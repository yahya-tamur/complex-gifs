use complex_gifs::gifs::*;
use complex_gifs::math::*;
use num_complex::Complex64;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Please provide the output directory in the first command line argument.");

    let polys = vec![
        vec![0.0, 0.0, 1.0],
        vec![2.0, 0.0, 1.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
    ];

    for (i, poly) in polys.iter().enumerate() {
        let f = |z: Complex64| eval_poly(poly, z);

        create_contour_loop_image(
            &ImageParameters {
                path: format!("{path}/poly_{}.gif", i + 1),
                x_start: -10.0,
                x_end: 10.0,
                y_start: -10.0,
                y_end: 10.,
                width: 500,
                height: 500,
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
