//        DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004 
//
// Copyright (C) 2023 Hironori Ishibashi
//
// Everyone is permitted to copy and distribute verbatim or modified 
// copies of this license document, and changing it is allowed as long 
// as the name is changed. 
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE 
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION 
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

//#![allow(dead_code)]

use core::{
    f32::consts::*,
    default::Default,
    cmp::Ordering,
    ops::{
        Neg,
        Add,
        AddAssign,
        Sub,
        SubAssign,
        Mul,
        MulAssign,
        Div,
        DivAssign,
        Index
    },
    fmt
};

#[inline]
fn sqrt(x: f32) -> f32 {
    const MAGIC_32: u32 = 0x5f3759df;

    let a = x * 0.5;
    let y = f32::from_bits(MAGIC_32 - (x.to_bits() >> 1));

    y * (1.5 - (a * y * y)) * x
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f32,
    pub im: f32
}

impl Complex {
    #[inline]
    pub const fn new(re: f32, im: f32) -> Self {
        Self {
            re: re,
            im: im
        }
    }

    #[inline]
    pub fn abs_2(&self) -> f32 {
        (self.re * self.re) + (self.im * self.im)
    }

    #[inline]
    pub fn abs(&self) -> f32 {
        sqrt(self.abs_2())
    }

    #[inline]
    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    #[inline]
    pub fn recip(&self) -> Self {
        let abs_2 = self.abs_2();

        Self {
            re: self.re / abs_2,
            im: -(self.im / abs_2),
        }
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let abs = self.abs();

        Self {
            re: self.re / abs,
            im: self.im / abs
        }
    }

    #[inline]
    pub fn mul_i(&self) -> Self {
        Self {
            re: -self.im,
            im: self.re
        }
    }

    #[inline]
    pub fn mul_minus_i(&self) -> Self {
        Self {
            re: self.im,
            im: -self.re
        }
    }
}

impl From<f32> for Complex {
    #[inline]
    fn from(value: f32) -> Self {
        Self::new(value, 0.0)
    }
}

impl From<Complex> for f32 {
    #[inline]
    fn from(value: Complex) -> Self {
        value.re
    }
}

impl Default for Complex {
    #[inline]
    fn default() -> Self {
        Self::new(f32::default(), f32::default())
    }
}

impl PartialOrd for Complex {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.re + self.im).partial_cmp(&(other.re + other.im))
    }
}

impl Neg for Complex {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im
        }
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im
        }
    }
}

impl Add<f32> for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, other: f32) -> Complex {
        Complex {
            re: self.re + other,
            im: self.im
        }
    }
}

impl Add<Complex> for f32 {
    type Output = Complex;

    #[inline]
    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self + other.re,
            im: other.im
        }
    }
}

impl AddAssign<Complex> for Complex {
    #[inline]
    fn add_assign(&mut self, other: Complex) {
        self.re += other.re;
        self.im += other.im;
    }
}

impl AddAssign<f32> for Complex {
    #[inline]
    fn add_assign(&mut self, other: f32) {
        self.re += other;
    }
}

impl AddAssign<Complex> for f32 {
    #[inline]
    fn add_assign(&mut self, other: Complex) {
        *self += other.re;
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn sub(self, other: Complex) -> Complex {
        Complex {
            re: self.re - other.re,
            im: self.im - other.im
        }
    }
}

impl Sub<f32> for Complex {
    type Output = Complex;

    #[inline]
    fn sub(self, other: f32) -> Complex {
        Complex {
            re: self.re - other,
            im: self.im
        }
    }
}

impl Sub<Complex> for f32 {
    type Output = Complex;

    #[inline]
    fn sub(self, other: Complex) -> Complex {
        Complex {
            re: self - other.re,
            im: other.im
        }
    }
}

impl SubAssign<Complex> for Complex {
    #[inline]
    fn sub_assign(&mut self, other: Complex) {
        self.re -= other.re;
        self.im -= other.im;
    }
}

impl SubAssign<f32> for Complex {
    #[inline]
    fn sub_assign(&mut self, other: f32) {
        self.re -= other;
    }
}

impl SubAssign<Complex> for f32 {
    #[inline]
    fn sub_assign(&mut self, other: Complex) {
        *self -= other.re;
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: (self.re * other.re) - (other.im * self.im),
            im: (other.re * self.im) + (self.re * other.im)
        }
    }
}

impl Mul<f32> for Complex {
    type Output = Complex;

    #[inline]
    fn mul(self, other: f32) -> Complex {
        Complex {
            re: self.re * other,
            im: self.im * other
        }
    }
}

impl Mul<Complex> for f32 {
    type Output = Complex;

    #[inline]
    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: self * other.re,
            im: self * other.im
        }
    }
}

impl MulAssign<Complex> for Complex {
    #[inline]
    fn mul_assign(&mut self, other: Complex) {
        let re = (self.re * other.re) - (other.im * self.im);
        let im = (other.re * self.im) + (self.re * other.im);

        self.re = re;
        self.im = im;
    }
}

impl MulAssign<f32> for Complex {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.re *= other;
        self.im *= other;
    }
}

impl MulAssign<Complex> for f32 {
    #[inline]
    fn mul_assign(&mut self, other: Complex) {
        *self *= other.re;
    }
}

impl Div<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn div(self, other: Complex) -> Complex {
        let abs_2 = other.abs_2();

        Complex {
            re: ((self.re * other.re) + (other.im * self.im)) / abs_2,
            im: ((other.re * self.im) - (self.re * other.im)) / abs_2
        }
    }
}

impl Div<f32> for Complex {
    type Output = Complex;

    #[inline]
    fn div(self, other: f32) -> Complex {
        Complex {
            re: self.re / other,
            im: self.im / other
        }
    }
}

impl Div<Complex> for f32 {
    type Output = Complex;

    #[inline]
    fn div(self, other: Complex) -> Complex {
        let abs_2 = other.abs_2();

        Complex {
            re: (self * other.re) / abs_2,
            im: -((self * other.im) / abs_2)
        }
    }
}

impl DivAssign<Complex> for Complex {
    #[inline]
    fn div_assign(&mut self, other: Complex) {
        let abs_2 = other.abs_2();

        let re = ((self.re * other.re) + (other.im * self.im)) / abs_2;
        let im = ((other.re * self.im) - (self.re * other.im)) / abs_2;

        self.re = re;
        self.im = im;
    }
}

impl DivAssign<f32> for Complex {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.re /= other;
        self.im /= other;
    }
}

impl DivAssign<Complex> for f32 {
    #[inline]
    fn div_assign(&mut self, other: Complex) {
        let abs_2 = other.abs_2();

        *self = (*self * other.re) / abs_2;
    }
}

impl fmt::Display for Complex {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}{:+}i", self.re, self.im)
    }
}

const BASE_LEN: usize = 11;

const SIN_BASE: [f32; BASE_LEN] = [
    0.0007669903187427045,
    0.0015339801862847655,
    0.003067956762965976,
    0.006135884649154475,
    0.012271538285719925,
    0.024541228522912288,
    0.049067674327418015,
    0.0980171403295606,
    0.19509032201612825,
    0.3826834323650898,
    0.7071067811865475
];

const COS_BASE: [f32; BASE_LEN] = [
    0.9999997058628822,
    0.9999988234517019,
    0.9999952938095762,
    0.9999811752826011,
    0.9999247018391445,
    0.9996988186962042,
    0.9987954562051724,
    0.9951847266721969,
    0.9807852804032304,
    0.9238795325112867,
    0.7071067811865476
];

const CIS_BASE: [Complex; BASE_LEN] = [
    Complex::new(COS_BASE[0], SIN_BASE[0]),
    Complex::new(COS_BASE[1], SIN_BASE[1]),
    Complex::new(COS_BASE[2], SIN_BASE[2]),
    Complex::new(COS_BASE[3], SIN_BASE[3]),
    Complex::new(COS_BASE[4], SIN_BASE[4]),
    Complex::new(COS_BASE[5], SIN_BASE[5]),
    Complex::new(COS_BASE[6], SIN_BASE[6]),
    Complex::new(COS_BASE[7], SIN_BASE[7]),
    Complex::new(COS_BASE[8], SIN_BASE[8]),
    Complex::new(COS_BASE[9], SIN_BASE[9]),
    Complex::new(COS_BASE[10], SIN_BASE[10])
];

const QUADRANT_0: Complex = Complex::new(1.0, 0.0);
const QUADRANT_1: Complex = Complex::new(0.0, 1.0);
const QUADRANT_2: Complex = Complex::new(-1.0, 0.0);
const QUADRANT_3: Complex = Complex::new(0.0, -1.0);

impl Complex {
    #[inline]
    pub const fn full_circle_angle() -> usize {
        const ANGLE: usize = 1 << (BASE_LEN + 2);

        ANGLE
    }

    #[inline]
    pub const fn rem_full_circle_angle(angle: usize) -> usize {
        const MASK: usize = Complex::full_circle_angle() - 1;
        angle & MASK
    }

    pub fn cis(angle: usize) -> Self {
        const QUADRANT_1_ANGLE: usize = Complex::full_circle_angle() >> 2;
        const QUADRANT_2_ANGLE: usize = Complex::full_circle_angle() >> 1;
        const QUADRANT_3_ANGLE: usize = QUADRANT_1_ANGLE + QUADRANT_2_ANGLE;
        const BIT_MASK: usize = 1;

        let mut angle = Self::rem_full_circle_angle(angle);

        let mut ret = if angle < QUADRANT_1_ANGLE {
            QUADRANT_0
        } else if angle < QUADRANT_2_ANGLE {
            QUADRANT_1
        } else if angle < QUADRANT_3_ANGLE {
            QUADRANT_2
        } else {
            QUADRANT_3
        };

        for cis_base in CIS_BASE {
            if (angle & BIT_MASK) == 1 {
                ret *= cis_base;
            }

            angle >>= 1;
        }

        ret
    }

    #[inline]
    pub fn from_polar(table: &CisTable, mag: f32, phase: usize) -> Self {
        table[phase] * mag
    }

    #[inline]
    pub fn radian_to_angle(rad: f32) -> usize {
        const MAX_ANGLE: f32 = Complex::full_circle_angle() as f32;

        let angle = (((rad % TAU) * MAX_ANGLE) / TAU) + MAX_ANGLE;

        Self::rem_full_circle_angle(angle as usize)
    }

    #[inline]
    pub fn angle_to_radian(angle: usize) -> f32 {
        const MAX_ANGLE: f32 = Complex::full_circle_angle() as f32;

        ((Self::rem_full_circle_angle(angle) as f32) * TAU) / MAX_ANGLE 
    }

    #[inline]
    pub fn rot(&mut self, table: &CisTable, angle: usize) {
        *self *= table[angle];
    }

    fn polar_core(
        table: &CisTable,
        cis: &Complex,
        mut min_angle: usize,
        mut max_angle: usize
    ) -> usize {
        loop {
            let min_d = (*cis - table[min_angle]).abs_2();
            let max_d = (*cis - table[max_angle]).abs_2();

            let middle_angle = (min_angle + max_angle) >> 1;

            if min_d < max_d {
                if max_angle == middle_angle {break middle_angle}

                max_angle = middle_angle;
            } else {
                if min_angle == middle_angle {break middle_angle}

                min_angle = middle_angle;
            }

        }
    }

    pub fn polar(&self, table: &CisTable) -> (f32, usize) {
        const QUADRANT_0_ANGLE: usize = 0;
        const QUADRANT_1_ANGLE: usize = Complex::full_circle_angle() >> 2;
        const QUADRANT_2_ANGLE: usize = Complex::full_circle_angle() >> 1;
        const QUADRANT_3_ANGLE: usize = QUADRANT_1_ANGLE + QUADRANT_2_ANGLE;
        const QUADRANT_4_ANGLE: usize = Complex::full_circle_angle();

        let abs = self.abs();
        let cis = *self / abs;

        let angle = if cis.re >= 0.0 {
            if cis.im >= 0.0 {
                const MAX: usize = QUADRANT_1_ANGLE - 1;
                Self::polar_core(
                    table,
                    &cis,
                    QUADRANT_0_ANGLE,
                    MAX
                )
            } else {
                const MAX: usize = QUADRANT_4_ANGLE - 1;
                Self::polar_core(
                    table,
                    &cis,
                    QUADRANT_3_ANGLE,
                    MAX
                )
            }
        } else {
            if cis.im >= 0.0 {
                const MAX: usize = QUADRANT_2_ANGLE - 1;
                Self::polar_core(
                    table,
                    &cis,
                    QUADRANT_1_ANGLE,
                    MAX
                )
            } else {
                const MAX: usize = QUADRANT_3_ANGLE - 1;
                Self::polar_core(
                    table,
                    &cis,
                    QUADRANT_2_ANGLE,
                    MAX
                )
            }
        };

        (abs, angle)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CisTable {
    body: [Complex; Complex::full_circle_angle()]
}

impl CisTable {
    pub fn new() -> Self {
        const FULL_CIRCLE_ANGLE: usize = Complex::full_circle_angle();

        let mut body: [Complex; FULL_CIRCLE_ANGLE] =
            [Complex::default(); FULL_CIRCLE_ANGLE];

        for i in 0..FULL_CIRCLE_ANGLE {
            body[i] = Complex::cis(i);
        }

        Self {body: body}
    }
}

impl Index<usize> for CisTable {
    type Output = Complex;

    #[inline]
    fn index(&self, index: usize) -> &Complex {
        &self.body[Complex::rem_full_circle_angle(index)]
    }
}
