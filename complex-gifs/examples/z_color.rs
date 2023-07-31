use complex_gifs::*;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or("../images/z_color.gif".to_string());

    let mut gradient = Vec::<[u8; 3]>::with_capacity(255);
    for i in 0..85 {
        gradient.push([255 - 3 * i, 3 * i, 0]);
    }
    for i in 0..85 {
        gradient.push([0, 255 - 3 * i, 3 * i]);
    }
    for i in 0..85 {
        gradient.push([3 * i, 0, 255 - 3 * i]);
    }

    let f = |z: Complex64| z;
    create_gradient_image(
        &ImageParameters {
            path,
            x_start: -1f64,
            x_end: 1f64,
            y_start: -1f64,
            y_end: 1f64,
            width: 500,
            height: 500,
        },
        gradient,
        f,
    );
}
