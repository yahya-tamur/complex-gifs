//haskell-style function composition.
//absolutely unnecessary, and slightly embarassing but I thought it was fun
//
// allows multiple arguments to the last function:
//
//hc![f . g . h @ x, y]  = f(g(h(x,y)))
//
// doesn't support for partial application like g. (f 5) $ 6 for g(f(5,6))
macro_rules! hc {
    ( $f:ident . $($tail:tt)+ ) => {{
        $f ( hc!($($tail)* ) )
    }};
    ( $f:ident @ $($x:expr),+ ) => {{
        $f ( $($x),+ )
    }}
}

use gif::Encoder;
use num_complex::Complex64;
use std::f64::consts::PI;

struct ImageParameters {
    path: String,
    x_start: f64,
    x_end: f64,
    y_start: f64,
    y_end: f64,
    width: u16,
    height: u16,
}

fn to_complex(im: &ImageParameters, r: u16, c: u16) -> Complex64 {
    let t = |x: u16| x as f64;
    Complex64::new(
        (t(c) / t(im.height)) * (im.x_end - im.x_start) + im.x_start,
        (t(im.width - r) / t(im.width)) * (im.y_end - im.y_start) + im.y_start,
    )
}

fn create_gradient_image(
    im: &ImageParameters,
    gradient: Vec<[u8; 3]>,
    f: fn(Complex64) -> Complex64,
) {
    let from_complex = |z: Complex64| {
        let index = ((z.arg() + PI) / (2f64 * PI) * (gradient.len() as f64)) as usize;
        if index >= gradient.len() {
            gradient[0]
        } else {
            gradient[index]
        }
    };

    let v: Vec<u8> = (0..im.width)
        .flat_map(|r| {
            (0..im.height).flat_map(move |c| hc![from_complex . f . to_complex @ im, r, c ])
        })
        .collect();

    let mut image = std::fs::File::create(&im.path).unwrap();
    let mut encoder = Encoder::new(&mut image, im.width, im.height, &[]).unwrap();
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();
    let frame = gif::Frame::from_rgb(im.width, im.height, &v);
    encoder.write_frame(&frame).unwrap();
}

struct ContourParameters {
    contour_spacing: f64,
    contour_width: f64,
    contour_color: [u8; 3],
}

fn create_gradient_image_with_contours(
    im: &ImageParameters,
    cm: &ContourParameters,
    gradient: Vec<[u8; 3]>,
    f: fn(Complex64) -> Complex64,
) {
    let from_complex = |z: Complex64| {
        if (z.norm() % cm.contour_spacing) < cm.contour_width {
            cm.contour_color
        } else {
            let index = ((z.arg() + PI) / (2f64 * PI) * (gradient.len() as f64)) as usize;
            if index >= gradient.len() {
                gradient[0]
            } else {
                gradient[index]
            }
        }
    };

    let v: Vec<u8> = (0..im.width)
        .flat_map(|r| {
            (0..im.height).flat_map(move |c| hc![ from_complex . f . to_complex @ im, r, c])
        })
        .collect();

    let mut image = std::fs::File::create(&im.path).unwrap();
    let mut encoder = Encoder::new(&mut image, im.width, im.height, &[]).unwrap();
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();
    let frame = gif::Frame::from_rgb(im.width, im.height, &v);
    encoder.write_frame(&frame).unwrap();
}

struct LoopParameters {
    argument_color: [u8; 3],
    contour_color: [u8; 3],
    background_color: [u8; 3],
    contour_spacing: f64,
    contour_width: f64, // = contour spacing => whole plane is highlighted
    angle_width: f64,   // = 2pi => whole plane is highlighted
    frames: u16,
}

/*
fn create_loop_image(im: &ImageParameters, lp: &LoopParameters, f: fn(Complex64) -> Complex64) {
    let angle_width_in_frames = (angle_width / (2f64 * PI) * (frames as f64)) as u16;
    let contour_width_in_frames = (contour_width / contour_spacing * (frames as f64)) as u16;
    //gives single
    let frame_of_contour = |z: Complex64| {
        if (z.norm() % lp.contour_spacing)
            lp.contour_color
        } else {
            let index = ((z.arg() + PI) / (2f64 * PI) * (gradient.len() as f64)) as usize;
            if index >= gradient.len() {
                gradient[0]
            } else {
                gradient[index]
            }
        }
    };
*/
/*
let v: Vec<16> = (0..im.width)
    .flat_map(|r| (0..im.height).map(|c|
        */
//}

//fn function(

fn main() {
    //let gradient: Vec<[u8; 3]> = (0..255).map(|x| [x, x, x]).collect();
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
            path: "../images/example 1.gif".to_string(),
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
    let gradient: Vec<[u8; 3]> = (0..255).map(|x| [255, x, x]).collect();
    create_gradient_image_with_contours(
        &ImageParameters {
            path: "../images/example 2.gif".to_string(),
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
