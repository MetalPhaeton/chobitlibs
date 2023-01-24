extern crate chobitlibs;

use std::prelude::rust_2021::*;

use chobitlibs::chobit_complex::*;
use chobitlibs::chobit_rand::ChobitRand;

#[inline]
fn rand_num(rng: &mut ChobitRand) -> f32 {
    ((rng.next_f64() * 20.0) - 10.0) as f32
}

#[test]
fn add_test() {
    const COUNT: usize = 1000;

    let mut rng = ChobitRand::new("add_test".as_bytes());

    for _ in 0..COUNT {
        let x_re = rand_num(&mut rng);
        let x_im = rand_num(&mut rng);

        let y_re = rand_num(&mut rng);
        let y_im = rand_num(&mut rng);

        let mut x = Complex::new(x_re, x_im);
        let y = Complex::new(y_re, y_im);

        let check = Complex::new(x_re + y_re, x_im + y_im);

        assert_eq!(x + y, check);

        x += y;
        assert_eq!(x, check);

        let mut scalar = rand_num(&mut rng);

        let mut x = Complex::new(x_re, x_im);
        let check = Complex::new(x_re + scalar, x_im);

        assert_eq!(x + scalar, check);
        assert_eq!(scalar + x, check);

        x += scalar;
        assert_eq!(x, check);

        let x = Complex::new(x_re, x_im);
        let check = scalar + x_re;

        scalar += x;
        assert_eq!(scalar, check);
    }
}

#[test]
fn sub_test() {
    const COUNT: usize = 1000;

    let mut rng = ChobitRand::new("sub_test".as_bytes());

    for _ in 0..COUNT {
        let x_re = rand_num(&mut rng);
        let x_im = rand_num(&mut rng);

        let y_re = rand_num(&mut rng);
        let y_im = rand_num(&mut rng);

        let mut x = Complex::new(x_re, x_im);
        let y = Complex::new(y_re, y_im);

        let check = Complex::new(x_re - y_re, x_im - y_im);

        assert_eq!(x - y, check);

        x -= y;
        assert_eq!(x, check);

        let mut scalar = rand_num(&mut rng);

        let mut x = Complex::new(x_re, x_im);
        let check = Complex::new(x_re - scalar, x_im);

        assert_eq!(x - scalar, check);

        let check = Complex::new(scalar - x_re, x_im);

        assert_eq!(scalar - x, check);

        let check = Complex::new(x_re - scalar, x_im);

        x -= scalar;
        assert_eq!(x, check);

        let x = Complex::new(x_re, x_im);
        let check = scalar - x_re;

        scalar -= x;
        assert_eq!(scalar, check);
    }
}

#[test]
fn mul_test() {
    const COUNT: usize = 1000;

    let mut rng = ChobitRand::new("mul_test".as_bytes());

    for _ in 0..COUNT {
        let x_re = rand_num(&mut rng);
        let x_im = rand_num(&mut rng);

        let y_re = rand_num(&mut rng);
        let y_im = rand_num(&mut rng);

        let mut x = Complex::new(x_re, x_im);
        let y = Complex::new(y_re, y_im);

        let check = Complex::new(
            (x_re * y_re) - (x_im * y_im),
            (x_re * y_im) + (x_im * y_re)
        );

        assert_eq!(x * y, check);

        x *= y;
        assert_eq!(x, check);

        let mut scalar = rand_num(&mut rng);

        let mut x = Complex::new(x_re, x_im);
        let check = Complex::new(x_re * scalar, x_im * scalar);

        assert_eq!(x * scalar, check);
        assert_eq!(scalar * x, check);

        x *= scalar;
        assert_eq!(x, check);

        let x = Complex::new(x_re, x_im);
        let check = scalar * x_re;

        scalar *= x;
        assert_eq!(scalar, check);
    }
}

#[test]
fn div_test() {
    const COUNT: usize = 1000;

    let mut rng = ChobitRand::new("div_test".as_bytes());

    for _ in 0..COUNT {
        let x_re = rand_num(&mut rng);
        let x_im = rand_num(&mut rng);

        let y_re = rand_num(&mut rng);
        let y_im = rand_num(&mut rng);

        let mut x = Complex::new(x_re, x_im);
        let y = Complex::new(y_re, y_im);

        let abs_sq = (y_re * y_re) + (y_im * y_im);
        let check = Complex::new(
            ((x_re * y_re) + (x_im * y_im)) / abs_sq,
            ((x_im * y_re) - (x_re * y_im)) / abs_sq
        );

        assert_eq!(x / y, check);

        x /= y;
        assert_eq!(x, check);

        let mut scalar = rand_num(&mut rng);

        let mut x = Complex::new(x_re, x_im);
        let check = Complex::new(x_re / scalar, x_im / scalar);

        assert_eq!(x / scalar, check);

        x /= scalar;
        assert_eq!(x, check);

        let x = Complex::new(x_re, x_im);
        let check = Complex::new(scalar, 0.0) / Complex::new(x_re, x_im);

        scalar /= x;
        assert_eq!(scalar, check.re);
    }
}

#[test]
fn rad_to_angle_test() {
    for angle in 0..CisTable::full_circle_angle() {
        let rad = CisTable::angle_to_radian(angle);
        let angle_2 = CisTable::radian_to_angle(rad);

        assert_eq!(angle_2, angle);
    }
}

#[test]
fn rot_test() {
    const COUNT: usize = 5000;

    let mut rng = ChobitRand::new("rot_test".as_bytes());
    let table = CisTable::new();

    for _ in 0..COUNT {
        let rad = rand_num(&mut rng);

        let x = Complex::new(1.0, 0.0);
        let y = x * table[CisTable::radian_to_angle(rad)];

        let diff_re = (y.re - rad.cos()).abs();
        let diff_im = (y.im - rad.sin()).abs();

        assert!(diff_re < 0.002);
        assert!(diff_im < 0.002);
    }
}

#[test]
fn polar_test() {
    const COUNT: usize = 10000;

    let mut rng = ChobitRand::new("polar_test".as_bytes());
    let table = CisTable::new();

    for _ in 0..COUNT {
        let x = Complex::new(rand_num(&mut rng), rand_num(&mut rng));
        let (mag, phase) = x.polar(&table);
        let y = table[phase] * mag;

        let diff = (x - y).abs();
        assert!(diff < 0.025);
    }
}

#[test]
fn div_test_2() {
    let table = CisTable::new();

    for angle in 0..CisTable::full_circle_angle() {
        let cis_1 = table[angle];
        let cis_2 = table[angle];
        let check = Complex::new(1.0, 0.0);

        let result = cis_1 / cis_2;

        let diff = (result - check).abs();

        assert!(diff < 0.000001);
    }
}

#[test]
fn recip_test() {
    const COUNT: usize = 10000;

    let mut rng = ChobitRand::new("recip_test".as_bytes());

    for _ in 0..COUNT {
        let x = Complex::new(rand_num(&mut rng), rand_num(&mut rng));
        let y = x.recip();
        let check = Complex::new(1.0, 0.0);
        let result = x * y;

        let diff = (result - check).abs();

        assert!(diff < 0.000001);
    }
}

#[test]
fn cis_table_abs_test() {
    let table = CisTable::new();

    for cis in table.as_slice() {
        let abs = cis.abs();
        let diff = (abs - 1.0).abs();

        assert!(diff < 0.00008);
    }
}

#[test]
fn normalize_test() {
    const COUNT: usize = 10000;

    let mut rng = ChobitRand::new("normalize_test".as_bytes());
    let table = CisTable::new();

    for _ in 0..COUNT {
        let x = Complex::new(rand_num(&mut rng), rand_num(&mut rng));
        let (_, phase) = x.polar(&table);
        let cis = table[phase];
        let y = x.normalize();

        let diff = (y - cis).abs();

        assert!(diff < 0.002);
    }
}

#[test]
fn cis_table_slice_test() {
    const QUADRANT_1_ANGLE: usize = CisTable::full_circle_angle() >> 2;
    const QUADRANT_2_ANGLE: usize = CisTable::full_circle_angle() >> 1;

    let table = CisTable::new();
    let table_slice = &table[QUADRANT_1_ANGLE..QUADRANT_2_ANGLE];
    assert_eq!(table_slice.len(), QUADRANT_1_ANGLE);

    for angle in 0..QUADRANT_1_ANGLE {
        assert_eq!(table_slice[angle], table[angle + QUADRANT_1_ANGLE]);
    }
}

#[test]
fn rem_full_circle_angle_test() {
    const QUADRANT_0_ANGLE: usize = 0;
    const QUADRANT_1_ANGLE: usize = CisTable::full_circle_angle() >> 2;
    const QUADRANT_2_ANGLE: usize = CisTable::full_circle_angle() >> 1;
    const QUADRANT_3_ANGLE: usize = QUADRANT_1_ANGLE + QUADRANT_2_ANGLE;

    let angle = QUADRANT_0_ANGLE;
    let angle = angle.wrapping_sub(QUADRANT_1_ANGLE);

    assert_ne!(angle, QUADRANT_3_ANGLE);

    let angle = CisTable::normalize_angle(angle);
    assert_eq!(angle, QUADRANT_3_ANGLE);
}
