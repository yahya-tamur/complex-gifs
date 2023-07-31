use gif::Encoder;
pub use num_complex::Complex64;
use std::f64::consts::PI;

pub struct ImageParameters {
    pub path: String,
    pub x_start: f64,
    pub x_end: f64,
    pub y_start: f64,
    pub y_end: f64,
    pub width: u16,
    pub height: u16,
}

pub fn to_complex(im: &ImageParameters, r: u16, c: u16) -> Complex64 {
    let t = |x: u16| x as f64;
    Complex64::new(
        (t(c) / t(im.height)) * (im.x_end - im.x_start) + im.x_start,
        (t(im.width - r) / t(im.width)) * (im.y_end - im.y_start) + im.y_start,
    )
}

pub fn create_gradient_image(
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

pub struct ContourParameters {
    pub contour_spacing: f64,
    pub contour_width: f64,
    pub contour_color: [u8; 3],
}

pub fn create_gradient_image_with_contours(
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

pub struct LoopParameters {
    pub argument_color: [u8; 3],
    pub contour_color: [u8; 3],
    pub background_color: [u8; 3],
    pub contour_spacing: f64,
    pub contour_width: f64, // = contour spacing => whole plane is highlighted
    pub angle_width: f64,   // = 2pi => whole plane is highlighted
    pub frames: u16,
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
