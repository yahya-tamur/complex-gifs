use complex_gifs::*;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or("../images/z_contour.gif".to_string());

    let f = |z: Complex64| Complex64::new(0.0, 5.0) * (z - Complex64::new(2.0, 0.0));

    create_loop_image(
        &ImageParameters {
            path,
            x_start: -5f64,
            x_end: 5f64,
            y_start: -5f64,
            y_end: 5f64,
            width: 500,
            height: 500,
        },
        &LoopParameters {
            argument_color: [100, 0, 0],
            background_color: [50, 100, 200],
            angle_width: 4,
            frames: 200,
        },
        &ContourParameters {
            contour_spacing: 1f64,
            contour_width: 0.1f64,
            contour_color: [0, 0, 0],
        },
        f,
    );
}
