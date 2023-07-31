use complex_gifs::*;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or("../images/example2.gif".to_string());

    let gradient: Vec<[u8; 3]> = (0..255).map(|x| [255, x, x]).collect();

    let f = |z: Complex64| z;

    create_gradient_image_with_contours(
        &ImageParameters {
            path,
            x_start: -1f64,
            x_end: 1f64,
            y_start: -1f64,
            y_end: 1f64,
            width: 500,
            height: 500,
        },
        &ContourParameters {
            contour_spacing: 0.1f64,
            contour_width: 0.01f64,
            contour_color: [0, 0, 0],
        },
        gradient,
        f,
    );
}
