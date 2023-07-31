use complex_gifs::*;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or("../images/z_loop.gif".to_string());


    let f = |z: Complex64| z;

    create_loop_image(
        &ImageParameters {
            path,
            x_start: -1f64,
            x_end: 1f64,
            y_start: -1f64,
            y_end: 1f64,
            width: 500,
            height: 500,
        },
        &LoopParameters {
          argument_color: [100, 0, 0],
          background_color: [0, 0, 100],
          angle_width: 0.1,
          frames: 100,
        },
        &ContourParameters {
            contour_spacing: 0.1f64,
            contour_width: 0.01f64,
            contour_color: [0, 0, 0],
        },
        f,
    );
}
