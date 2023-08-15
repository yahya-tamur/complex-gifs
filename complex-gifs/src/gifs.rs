use gif::Encoder;
use num_complex::Complex64;
use rayon::prelude::*;
use std::f64::consts::PI;
use std::marker::Sync;

pub struct ImageParameters {
    pub path: String,
    pub x_start: f64,
    pub x_end: f64,
    pub y_start: f64,
    pub y_end: f64,
    pub width: u16,
    pub height: u16,
}

pub struct ContourParameters {
    pub contour_spacing: f64,
    pub contour_width: f64,
    pub contour_color: [u8; 3],
}

pub struct LoopContourParameters {
    pub contour_spacing: f64,
    pub contour_width: usize,
    pub contour_color: [u8; 3],
}

pub struct LoopParameters {
    pub argument_color: [u8; 3],
    pub background_color: [u8; 3],
    pub angle_width: usize,
    pub frames: usize,
}

pub struct SphereParameters {
    pub theta: f64,
    pub phi: f64,
    pub sphere_color: [u8; 3],
}

const fn generate_gradient() -> [u8; 765] {
    let mut gradient = [0u8; 765];
    let mut i = 0; //no for loops in const functions?????
    while i < 85 {
        let (rise, fall) = (3 * i as u8, 255 - 3 * i as u8);
        gradient[3 * i] = fall;
        gradient[3 * i + 1] = rise;
        gradient[3 * (i + 85) + 1] = fall;
        gradient[3 * (i + 85) + 2] = rise;
        gradient[3 * (i + 170) + 2] = fall;
        gradient[3 * (i + 170)] = rise;
        i += 1;
    }
    gradient
}

//safe to use with and without contours
pub const GRADIENT: [u8; 765] = generate_gradient();

fn to_complex(im: &ImageParameters, r: u16, c: u16) -> Complex64 {
    let t = |x: u16| x as f64;
    Complex64::new(
        (t(c) / t(im.width)) * (im.x_end - im.x_start) + im.x_start,
        (t(im.height - r) / t(im.height)) * (im.y_end - im.y_start) + im.y_start,
    )
}

fn arg_index(index_max: usize, z: Complex64) -> usize {
    let arg = z.arg();
    let arg = if arg > 0.0 { arg } else { arg + 2.0 * PI };

    let arg_index = (arg / (2.0 * PI) * (index_max as f64)) as usize;

    if arg_index < index_max {
        arg_index
    } else {
        0
    }
}

fn static_contour(cp: &ContourParameters, width: f64, z: Complex64) -> bool {
    let rem = z.norm() % cp.contour_spacing;

    rem < width
}

fn contour_index(lcp: &LoopContourParameters, frames: usize, z: Complex64) -> usize {
    let rem = z.norm() % lcp.contour_spacing;

    let rem_index = (rem / lcp.contour_spacing * (frames as f64)) as usize;

    if rem_index < frames {
        rem_index
    } else {
        0
    }
}

//is i in the next d indices of j modulo n?
fn mod_range(i: usize, j: usize, n: usize, d: usize) -> bool {
    if i >= j {
        i - j < d
    } else {
        i + n - j < d
    }
}

pub fn create_gradient_image(
    im: &ImageParameters,
    // gradient must contain <= 256 colors.
    // Conforming to the gif crate, this is a 3*<number of colors> slice
    // containing the red value, then the green value, then the blue value for
    // every color.
    gradient: &[u8],
    f: impl Fn(Complex64) -> Complex64 + Sync,
) {
    let n = gradient.len() / 3;
    let color = |i| i as u8;

    let pixels: Vec<u8> = (0..im.height)
        .into_par_iter()
        .map(|r| {
            (0..im.width)
                .map(|c| hc![color, arg_index n, f , to_complex @ im, r, c ])
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect();

    let mut image = std::fs::File::create(&im.path).unwrap();
    let mut encoder = Encoder::new(&mut image, im.width, im.height, &[]).unwrap();
    let frame = gif::Frame::from_palette_pixels(im.width, im.height, &pixels, gradient, None);
    encoder.write_frame(&frame).unwrap();
}

pub fn create_contour_gradient_image(
    im: &ImageParameters,
    cp: &ContourParameters,
    // gradient must contain <= 255 colors, since one color is alloted for the
    // contour.
    // Conforming to the gif crate, this is a 3*<number of colors> slice
    // containing the red value, then the green value, then the blue value for
    // every color.
    gradient: &[u8],
    f: impl Fn(Complex64) -> Complex64 + Sync,
) {
    let n = gradient.len() / 3;
    let mut gradient = gradient.to_vec();
    gradient.extend_from_slice(&cp.contour_color);

    let color = |z: Complex64| {
        if static_contour(cp, cp.contour_width, z) {
            n as u8
        } else {
            arg_index(n, z) as u8
        }
    };

    let pixels: Vec<u8> = (0..im.height)
        .into_par_iter()
        .map(|r| {
            (0..im.width)
                .map(|c| hc![color, f , to_complex @ im, r, c ])
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect();

    let mut image = std::fs::File::create(&im.path).unwrap();
    let mut encoder = Encoder::new(&mut image, im.width, im.height, &[]).unwrap();
    let frame = gif::Frame::from_palette_pixels(im.width, im.height, &pixels, &gradient, None);
    encoder.write_frame(&frame).unwrap();
}

pub fn create_loop_image(
    im: &ImageParameters,
    lp: &LoopParameters,
    cp: &ContourParameters,
    f: impl Fn(Complex64) -> Complex64 + Sync,
) {
    let plane: &Vec<Vec<(bool, usize)>> = &(0..im.height)
        .into_par_iter()
        .map(|r| {
            (0..im.width)
                .map(|c| {
                    let z = hc![f , to_complex @ im, r, c ];
                    (
                        static_contour(cp, cp.contour_width, z),
                        arg_index(lp.frames, z),
                    )
                })
                .collect()
        })
        .collect();

    let palette = [cp.contour_color, lp.argument_color, lp.background_color]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>();

    let frames: Vec<gif::Frame> = (0..lp.frames)
        .into_par_iter()
        .map(|i| {
            let v: Vec<u8> = (0..im.height)
                .flat_map(|r| {
                    (0..im.width).map(move |c| {
                        let (contour, arg) = plane[r as usize][c as usize];
                        if contour {
                            0
                        } else if mod_range(i, arg, lp.frames, lp.angle_width) {
                            1
                        } else {
                            2
                        }
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

pub fn create_contour_loop_image(
    im: &ImageParameters,
    lp: &LoopParameters,
    lcp: &LoopContourParameters,
    f: impl Fn(Complex64) -> Complex64 + Sync,
) {
    let plane: &Vec<Vec<(usize, usize)>> = &(0..im.height)
        .into_par_iter()
        .map(|r| {
            (0..im.width)
                .map(|c| {
                    let z = hc![f , to_complex @ im, r, c ];
                    (contour_index(lcp, lp.frames, z), arg_index(lp.frames, z))
                })
                .collect()
        })
        .collect();

    let palette = [lcp.contour_color, lp.argument_color, lp.background_color]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>();

    let frames: Vec<gif::Frame> = (0..lp.frames)
        .into_par_iter()
        .map(|i| {
            let v: Vec<u8> = (0..im.height)
                .flat_map(|r| {
                    (0..im.width).map(move |c| {
                        let (contour, arg) = plane[r as usize][c as usize];
                        if mod_range(i, contour, lp.frames, lcp.contour_width) {
                            0
                        } else if mod_range(i, arg, lp.frames, lp.angle_width) {
                            1
                        } else {
                            2
                        }
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

pub fn create_sphere_contour_loop_image(
    im: &ImageParameters,
    lp: &LoopParameters,
    lcp: &LoopContourParameters,
    sp: &SphereParameters,
    f: impl Fn(Complex64) -> Complex64 + Sync,
) {
    let proj = |c: Complex64| {
        let (x, y) = (c.re, c.im);
        let t = sp.theta + PI;
        let p = sp.phi;
        let z_sq = 1.0 - x * x - y * y;
        if z_sq < 0.0 {
            None
        } else {
            let z = z_sq.sqrt();
            let (x, y, z) = (x * p.cos() - z * p.sin(), y, x * p.sin() + z * p.cos());
            let (x, y, z) = (x, y * t.cos() - z * t.sin(), y * t.sin() + z * t.cos());
            Some(Complex64::new(x / (1.0 - z), y / (1.0 - z)))
        }
    };

    let plane: &Vec<Vec<Option<(usize, usize)>>> = &(0..im.height)
        .into_par_iter()
        .map(|r| {
            (0..im.width)
                .map(|c| {
                    hc![proj , to_complex @ im, r, c ].map(|z| {
                        (
                            contour_index(lcp, lp.frames, f(z)),
                            arg_index(lp.frames, f(z)),
                        )
                    })
                })
                .collect()
        })
        .collect();

    let palette = [
        lcp.contour_color,
        lp.argument_color,
        sp.sphere_color,
        lp.background_color,
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<u8>>();

    let frames: Vec<gif::Frame> = (0..lp.frames)
        .into_par_iter()
        .map(|i| {
            let v: Vec<u8> = (0..im.height)
                .flat_map(|r| {
                    (0..im.width).map(move |c| {
                        if let Some((contour, arg)) = plane[r as usize][c as usize] {
                            if mod_range(i, contour, lp.frames, lcp.contour_width) {
                                0
                            } else if mod_range(i, arg, lp.frames, lp.angle_width) {
                                1
                            } else {
                                2
                            }
                        } else {
                            3
                        }
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
