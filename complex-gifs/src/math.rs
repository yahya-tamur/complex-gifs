use num_complex::Complex64;

//All the f64 can be Complex64, but I didn't need that yet.

pub fn eval_poly(poly: &[f64], z: Complex64) -> Complex64 {
    let mut ans = Complex64::new(0.0, 0.0);

    for &c in poly.iter().rev() {
        ans = ans * z + c;
    }

    ans
}

//This should be better for power series with terms like z^n/n! than the one above.
pub fn eval_progressive_poly(poly: &[f64], z: Complex64) -> Complex64 {
    let mut ans = Complex64::new(1.0, 0.0);

    let mut term = Complex64::new(1.0, 0.0);

    for &c in poly.iter() {
        term *= c * z;
        ans += term;
    }

    ans
}

pub fn eval_root_poly(roots: &[(Complex64, i32)], z: Complex64) -> Complex64 {
    let mut ans = Complex64::new(1.0, 0.0);

    for &(r, n) in roots.iter() {
        ans *= Complex64::powi(&(z - r), n);
    }

    ans
}
