use complex_gifs::gifs::*;
use num_complex::Complex64;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Please provide the output directory in the first command line argument.");

    let parameters = vec![
        (Complex64::new(0.0, 5.0), Complex64::new(2.0, 0.0)),
        (Complex64::new(-0.5, 0.0), Complex64::new(3.0, 2.0)),
    ];

    for (i, (a, b)) in parameters.iter().enumerate() {
        let f = |z: Complex64| a * (z - b);

        create_contour_loop_image(
            &ImageParameters {
                path: format!("{path}/linear_{}.gif", i + 1),
                x_start: -5f64,
                x_end: 5f64,
                y_start: -5f64,
                y_end: 5f64,
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
                contour_spacing: 1f64,
                contour_width: 30,
                contour_color: [0, 0, 0],
            },
            f,
        );
    }
}
