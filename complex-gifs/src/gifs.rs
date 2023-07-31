use gif::Encoder;
pub use num_complex::Complex64;
use rayon::prelude::*;
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

fn to_complex(im: &ImageParameters, r: u16, c: u16) -> Complex64 {
    let t = |x: u16| x as f64;
    Complex64::new(
        (t(c) / t(im.height)) * (im.x_end - im.x_start) + im.x_start,
        (t(im.width - r) / t(im.width)) * (im.y_end - im.y_start) + im.y_start,
    )
}

fn index(index_max: usize, z: Complex64) -> usize {
    let i = ((z.arg() + PI) / (2f64 * PI) * (index_max as f64)) as usize;
    if i < index_max {
        i
    } else {
        0
    }
}

#[derive(Debug, Clone, Copy)]
enum ComplexInfo {
    Index(usize),
    Contour,
}

fn index_contour(index_max: usize, cp: &ContourParameters, z: Complex64) -> ComplexInfo {
    if (z.norm() % cp.contour_spacing) < cp.contour_width {
        ComplexInfo::Contour
    } else {
        ComplexInfo::Index(index(index_max, z))
    }
}

pub fn create_gradient_image(
    im: &ImageParameters,
    gradient: Vec<[u8; 3]>, //gradient must contain <= 256 colors!
    f: fn(Complex64) -> Complex64,
) {
    let n = gradient.len();
    let color = |i| i as u8;

    let plane_info: Vec<u8> = (0..im.width)
        .into_par_iter()
        .map(|r| {
            (0..im.height)
                .map(|c| hc![color, index n, f , to_complex @ im, r, c ])
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect();

    let gradient = gradient.into_iter().flatten().collect::<Vec<u8>>();
    let mut image = std::fs::File::create(&im.path).unwrap();
    let mut encoder = Encoder::new(&mut image, im.width, im.height, &[]).unwrap();
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();
    let frame = gif::Frame::from_palette_pixels(im.width, im.height, &plane_info, &gradient, None);
    encoder.write_frame(&frame).unwrap();
}

pub struct ContourParameters {
    pub contour_spacing: f64,
    pub contour_width: f64,
    pub contour_color: [u8; 3],
}

pub fn create_gradient_image_with_contours(
    im: &ImageParameters,
    cp: &ContourParameters,
    mut gradient: Vec<[u8; 3]>, //gradient must contain <= 255 colors!
    f: fn(Complex64) -> Complex64,
) {
    let n = gradient.len();
    gradient.push(cp.contour_color);
    let color = |ci: ComplexInfo| match ci {
        ComplexInfo::Index(i) => i as u8,
        ComplexInfo::Contour => n as u8,
    };

    let plane_info: Vec<u8> = (0..im.width)
        .into_par_iter()
        .map(|r| {
            (0..im.height)
                .map(|c| hc![color, index_contour n cp, f , to_complex @ im, r, c ])
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect();

    let gradient = gradient.into_iter().flatten().collect::<Vec<u8>>();

    let mut image = std::fs::File::create(&im.path).unwrap();
    let mut encoder = Encoder::new(&mut image, im.width, im.height, &[]).unwrap();
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();
    let frame = gif::Frame::from_palette_pixels(im.width, im.height, &plane_info, &gradient, None);
    encoder.write_frame(&frame).unwrap();
}

pub struct LoopParameters {
    pub argument_color: [u8; 3],
    pub background_color: [u8; 3],
    pub angle_width: usize, // angle_width / frames of the range is highlighted
    pub frames: u16,
}

pub fn create_loop_image(
    im: &ImageParameters,
    lp: &LoopParameters,
    cp: &ContourParameters,
    f: fn(Complex64) -> Complex64,
) {
    //?? let contour_width_in_frames = (cp.contour_width / cp.contour_spacing * (lp.frames as f64)) as u16;

    fn loop_diff(i: usize, j: usize, n: usize) -> usize {
        if j >= i {
            j - i
        } else {
            j + n - i
        }
    }

    let plane_info: Vec<Vec<ComplexInfo>> = (0..im.width)
        .into_par_iter()
        .map(|r| {
            (0..im.height)
                .map(|c| hc![index_contour lp.frames.into() cp, f , to_complex @ im, r, c ])
                .collect()
        })
        .collect();

    let get_plane_info = |r: usize, c: usize| plane_info[r][c];

    let palette = [cp.contour_color, lp.argument_color, lp.background_color]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>();

    let frames: Vec<gif::Frame> = (0..lp.frames)
        .into_par_iter()
        .map(|i| {
            let v: Vec<u8> = (0..im.width)
                .flat_map(|r| {
                    (0..im.height).map(move |c| match get_plane_info(r as usize, c as usize) {
                        ComplexInfo::Contour => 0,
                        ComplexInfo::Index(j)
                            if loop_diff(i.into(), j, lp.frames.into()) < lp.angle_width =>
                        {
                            1
                        }
                        _ => 2,
                    })
                })
                .collect();

            let mut frame =
                gif::Frame::from_palette_pixels(im.width, im.height, &v, &palette, None);
            frame.make_lzw_pre_encoded();
            frame
        })
        .collect();

    let mut image = std::fs::File::create(&im.path).unwrap();
    let mut encoder = Encoder::new(&mut image, im.width, im.height, &[]).unwrap();
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();
    for frame in frames {
        encoder.write_lzw_pre_encoded_frame(&frame).unwrap();
    }
}
