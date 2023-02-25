//        DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004 
//
// Copyright (C) 2022 Hironori Ishibashi
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

//! Neural network library.
//!
//! This library needs `alloc` crate.
//!
//! # Example
//!

use alloc::{boxed::Box, vec, vec::Vec};

use core::{
    default::Default,
    ops::{
        Add,
        AddAssign,
        Sub,
        SubAssign,
        Mul,
        MulAssign,
        Div,
        DivAssign,
        Rem,
        RemAssign,
        Deref,
        DerefMut
    },
    slice::{from_raw_parts, from_raw_parts_mut},
    mem::size_of
};

#[inline]
fn abs(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7fffffff)
}

#[inline]
fn sqrt(x: f32) -> f32 {
    const MAGIC_1: u32 = 0x5f1ffff9;
    const MAGIC_2: f32 = 0.703952253;
    const MAGIC_3: f32 = 2.38924456;

    let y = f32::from_bits(MAGIC_1 - (x.to_bits() >> 1));

    y * (MAGIC_2 * (MAGIC_3 - (x * y * y))) * x
}

macro_rules! pointwise_op {
    ($self:expr, $other:expr, $ops:tt) => {{
        for i in 0..N {
            unsafe {
                *$self.body.get_unchecked_mut(i)
                    $ops *$other.body.get_unchecked(i);
            }
        }
    }};
}

macro_rules! scalar_op {
    ($self:expr, $other:expr, $ops:tt) => {{
        for i in 0..N {
            unsafe {
                *$self.body.get_unchecked_mut(i) $ops $other;
            }
        }
    }};
}

const SIZE_OF_U32: usize = size_of::<u32>();

fn u8_slice_to_f32_slice(
    u8_slice: &[u8],
    f32_slice: &mut [f32]
) -> Option<()> {
    let f32_len = f32_slice.len();

    if (f32_len * SIZE_OF_U32) != u8_slice.len() {return None;}

    for i in 0..f32_len {
        let u8_start = i * SIZE_OF_U32;
        let u8_end = u8_start + SIZE_OF_U32;

        let val = u32::from_le_bytes(
            u8_slice[u8_start..u8_end].try_into().ok()?
        );
        let val = f32::from_bits(val);

        *(f32_slice.get_mut(i)?) = val;
    }

    Some(())
}

fn f32_slice_to_u8_slice(
    f32_slice: &[f32],
    u8_slice: &mut [u8]
) -> Option<()> {
    let f32_len = f32_slice.len();

    if (f32_len * SIZE_OF_U32) != u8_slice.len() {return None;}

    for i in 0..f32_len {
        let u8_start = i * SIZE_OF_U32;
        let u8_end = u8_start + SIZE_OF_U32;

        let val = *(f32_slice.get(i)?);
        let val = val.to_bits();

        u8_slice[u8_start..u8_end].copy_from_slice(
            val.to_le_bytes().as_slice()
        );
    }

    Some(())
}

/// Vector for mathematics.
///
/// * `N` : Dimension.
#[derive(Debug, Clone, PartialEq)]
pub struct MathVec<const N: usize> {
    body: Box<[f32]>
}

impl<const N: usize> MathVec<N> {
    /// Creates MathVec.
    #[inline]
    pub fn new() -> Self {
        Self {
            body: vec![f32::default(); N].into_boxed_slice()
        }
    }

    /// Gets this as immutable slice.
    ///
    /// * _Return_ : slice.
    #[inline]
    pub fn as_slice(&self) -> &[f32] {&*self.body}

    /// Gets this as mutable slice.
    ///
    /// * _Return_ : slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [f32] {&mut *self.body}

    /// Resets all value into 0.
    #[inline]
    pub fn clear(&mut self) {self.body.fill(f32::default());}

    /// Pointwise multiplication.
    ///
    /// * `other` : Other factor.
    /// * _Return_ : Result.
    #[inline]
    pub fn pointwise_mul(&self, other: &Self) -> Self {
        let mut ret = self.clone();

        pointwise_op!(ret, other, *=);

        ret
    }

    /// Pointwise multiplication and Assign.
    ///
    /// * `other` : Other factor.
    #[inline]
    pub fn pointwise_mul_assign(&mut self, other: &Self) {
        pointwise_op!(self, other, *=);
    }

    /// Pointwise division.
    ///
    /// * `other` : Divisor.
    /// * _Return_ : Result.
    #[inline]
    pub fn pointwise_div(&self, other: &Self) -> Self {
        let mut ret = self.clone();

        pointwise_op!(ret, other, /=);

        ret
    }

    /// Pointwise division and Assign.
    ///
    /// * `other` : Divisor.
    #[inline]
    pub fn pointwise_div_assign(&mut self, other: &Self) {
        pointwise_op!(self, other, /=);
    }

    /// Pointwise division remainder.
    ///
    /// * `other` : Divisor.
    /// * _Return_ : Result.
    #[inline]
    pub fn pointwise_rem(&self, other: &Self) -> Self {
        let mut ret = self.clone();

        pointwise_op!(ret, other, %=);

        ret
    }

    /// Pointwise division remainder and Assign.
    ///
    /// * `other` : Divisor.
    #[inline]
    pub fn pointwise_rem_assign(&mut self, other: &Self) {
        pointwise_op!(self, other, %=);
    }

    /// Copies from another vector.
    ///
    /// * `other` : Another vector.
    #[inline]
    pub fn copy_from(&mut self, other: &Self) {
        self.body.copy_from_slice(&*other.body);
    }

    /// Copies to another vector.
    ///
    /// * `other` : Another vector.
    #[inline]
    pub fn copy_to(&self, other: &mut Self) {
        other.body.copy_from_slice(self.as_slice());
    }

    #[inline]
    pub fn read_bytes(&mut self, bytes: &[u8]) -> Option<()> {
        u8_slice_to_f32_slice(bytes, &mut self.body)
    }

    #[inline]
    pub fn write_bytes(&self, buffer: &mut Vec<u8>) -> Option<()> {
        buffer.resize(self.body.len() * SIZE_OF_U32, 0);

        f32_slice_to_u8_slice(&self.body, buffer.as_mut_slice())
    }
}

impl<const N: usize> Default for MathVec<N> {
    #[inline]
    fn default() -> Self {Self::new()}
}

impl<const N: usize> Add<&MathVec<N>> for &MathVec<N> {
    type Output = MathVec<N>;

    #[inline]
    fn add(self, other: &MathVec<N>) -> MathVec<N> {
        let mut ret = self.clone();

        pointwise_op!(ret, other, +=);

        ret
    }
}

impl<const N: usize> AddAssign<&MathVec<N>> for MathVec<N> {
    #[inline]
    fn add_assign(&mut self, other: &MathVec<N>) {
        pointwise_op!(self, other, +=);
    }
}

impl<const N: usize> Sub<&MathVec<N>> for &MathVec<N> {
    type Output = MathVec<N>;

    #[inline]
    fn sub(self, other: &MathVec<N>) -> MathVec<N> {
        let mut ret = self.clone();

        pointwise_op!(ret, other, -=);

        ret
    }
}

impl<const N: usize> SubAssign<&MathVec<N>> for MathVec<N> {
    #[inline]
    fn sub_assign(&mut self, other: &MathVec<N>) {
        pointwise_op!(self, other, -=);
    }
}

impl<const N: usize> Mul<f32> for &MathVec<N> {
    type Output = MathVec<N>;

    #[inline]
    fn mul(self, other: f32) -> MathVec<N> {
        let mut ret = self.clone();

        scalar_op!(ret, other, *=);

        ret
    }
}

impl<const N: usize> MulAssign<f32> for MathVec<N> {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        scalar_op!(self, other, *=);
    }
}

impl<const N: usize> Div<f32> for &MathVec<N> {
    type Output = MathVec<N>;

    #[inline]
    fn div(self, other: f32) -> MathVec<N> {
        let mut ret = self.clone();

        scalar_op!(ret, other, /=);

        ret
    }
}

impl<const N: usize> DivAssign<f32> for MathVec<N> {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        scalar_op!(self, other, /=);
    }
}

impl<const N: usize> Rem<f32> for &MathVec<N> {
    type Output = MathVec<N>;

    #[inline]
    fn rem(self, other: f32) -> MathVec<N> {
        let mut ret = self.clone();

        scalar_op!(ret, other, %=);

        ret
    }
}

impl<const N: usize> RemAssign<f32> for MathVec<N> {
    #[inline]
    fn rem_assign(&mut self, other: f32) {
        scalar_op!(self, other, %=);
    }
}

impl<const N: usize> Mul<&MathVec<N>> for &MathVec<N> {
    type Output = f32;

    #[inline]
    fn mul(self, other: &MathVec<N>) -> f32 {
        let mut ret = 0.0;

        for i in 0..N {
            ret += unsafe {
                *self.body.get_unchecked(i) * *other.body.get_unchecked(i)
            };
        }

        ret
    }
}

impl<const N: usize> Deref for MathVec<N> {
    type Target = [f32];

    #[inline]
    fn deref(&self) -> &[f32] {
        &*self.body
    }
}

impl<const N: usize> DerefMut for MathVec<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [f32] {
        &mut *self.body
    }
}

macro_rules! to_label_body {
    ($self:expr, $type:ty) => {{
        const FLAG: $type = (1 as $type).rotate_right(1);
        let mut ret: $type = 0;

        $self.iter().for_each(
            |bit| {
                ret >>= 1;

                ret |= if *bit >= 0.0 {
                    FLAG
                } else {
                    0
                };
            }
        );

        ret
    }};
}

macro_rules! load_label_body {
    ($self:expr, $type:ty, $label:expr) => {{
        const MASK: $type = 0x01;

        $self.iter_mut().for_each(
            |bit| {
                *bit = if ($label & MASK) == 1 {
                    1.0 - f32::EPSILON
                } else {
                    -1.0 + f32::EPSILON
                };

                $label >>= 1;
            }
        );
    }};
}

impl MathVec<8> {
    /// Converts to `u8` label
    ///
    /// * _Return_ : Label.
    #[inline]
    pub fn to_u8_label(&self) -> u8 {
        to_label_body!(self, u8)
    }

    /// Loads values from `u8` label
    ///
    /// * `label` : Label.
    #[inline]
    pub fn load_u8_label(&mut self, mut label: u8) {
        load_label_body!(self, u8, label)
    }
}

impl MathVec<16> {
    /// Converts to `u16` label
    ///
    /// * _Return_ : Label.
    #[inline]
    pub fn to_u16_label(&self) -> u16 {
        to_label_body!(self, u16)
    }

    /// Loads values from `u16` label
    ///
    /// * `label` : Label.
    #[inline]
    pub fn load_u16_label(&mut self, mut label: u16) {
        load_label_body!(self, u16, label)
    }
}

impl MathVec<32> {
    /// Converts to `u32` label
    ///
    /// * _Return_ : Label.
    #[inline]
    pub fn to_u32_label(&self) -> u32 {
        to_label_body!(self, u32)
    }

    /// Loads values from `u32` label
    ///
    /// * `label` : Label.
    #[inline]
    pub fn load_u32_label(&mut self, mut label: u32) {
        load_label_body!(self, u32, label)
    }
}

impl MathVec<64> {
    /// Converts to `u64` label
    ///
    /// * _Return_ : Label.
    #[inline]
    pub fn to_u64_label(&self) -> u64 {
        to_label_body!(self, u64)
    }

    /// Loads values from `u64` label
    ///
    /// * `label` : Label.
    #[inline]
    pub fn load_u64_label(&mut self, mut label: u64) {
        load_label_body!(self, u64, label)
    }
}

impl MathVec<128> {
    /// Converts to `u64` label
    ///
    /// * _Return_ : Label.
    #[inline]
    pub fn to_u128_label(&self) -> u128 {
        to_label_body!(self, u128)
    }

    /// Loads values from `u64` label
    ///
    /// * `label` : Label.
    #[inline]
    pub fn load_u128_label(&mut self, mut label: u128) {
        load_label_body!(self, u128, label)
    }
}

/// Weights of a linear function.
#[derive(Debug)]
pub struct Weights<const OUT: usize, const IN: usize> {
    body: Box<[f32]>,

    ptr_b: *const f32,
    ptr_i: *const [f32; IN],
    ptr_s: *const [f32; OUT],

    mut_ptr_b: *mut f32,
    mut_ptr_i: *mut [f32; IN],
    mut_ptr_s: *mut [f32; OUT]
}

impl<const OUT: usize, const IN: usize> Weights<OUT, IN> {
    #[inline]
    pub fn new() -> Self {
        let offset_i: usize = OUT;
        let offset_s: usize = offset_i + (OUT * IN);
        let length: usize = offset_s + (OUT * OUT);

        let mut body = vec![f32::default(); length].into_boxed_slice();

        let ptr_b = body.as_ptr();
        let ptr_i = unsafe {ptr_b.add(offset_i) as *const [f32; IN]};
        let ptr_s = unsafe {ptr_b.add(offset_s) as *const [f32; OUT]};

        let mut_ptr_b = body.as_mut_ptr();
        let mut_ptr_i = unsafe {mut_ptr_b.add(offset_i) as *mut [f32; IN]};
        let mut_ptr_s = unsafe {mut_ptr_b.add(offset_s) as *mut [f32; OUT]};

        Self {
            body: body,

            ptr_b: ptr_b,
            ptr_i: ptr_i,
            ptr_s: ptr_s,

            mut_ptr_b: mut_ptr_b,
            mut_ptr_i: mut_ptr_i,
            mut_ptr_s: mut_ptr_s
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[f32] {&*self.body}

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [f32] {&mut *self.body}

    #[inline]
    pub fn clear(&mut self) {self.body.fill(f32::default());}

    #[inline]
    pub fn bias(&self) -> &[f32] {
        unsafe {
            from_raw_parts(self.ptr_b, OUT)
        }
    }

    #[inline]
    pub fn bias_mut(&mut self) -> &mut [f32] {
        unsafe {
            from_raw_parts_mut(self.mut_ptr_b, OUT)
        }
    }

    #[inline]
    pub fn input_weights(&self) -> &[[f32; IN]] {
        unsafe {
            from_raw_parts(self.ptr_i, OUT)
        }
    }

    #[inline]
    pub fn input_weights_mut(&mut self) -> &mut [[f32; IN]] {
        unsafe {
            from_raw_parts_mut(self.mut_ptr_i, OUT)
        }
    }

    #[inline]
    pub fn state_weights(&self) -> &[[f32; OUT]] {
        unsafe {
            from_raw_parts(self.ptr_s, OUT)
        }
    }

    #[inline]
    pub fn state_weights_mut(&mut self) -> &mut [[f32; OUT]] {
        unsafe {
            from_raw_parts_mut(self.mut_ptr_s, OUT)
        }
    }

    pub fn calc(
        &self,
        input: &MathVec<IN>,
        state: Option<&MathVec<OUT>>,
        output: &mut MathVec<OUT>
    ) {
        self.init_output_with_bias(output);

        self.calc_input(input, output);

        if let Some(state) = state {
            self.calc_state(state, output);
        }
    }

    #[inline]
    fn init_output_with_bias(&self, output: &mut MathVec<OUT>) {
        unsafe {
            output.as_mut_ptr().copy_from(self.ptr_b, OUT);
        }
    }

    #[inline]
    fn calc_input(&self, input: &MathVec<IN>, output: &mut MathVec<OUT>) {
        for i in 0..OUT {
            let weights = unsafe {(*self.ptr_i.add(i)).as_slice()};

            for j in 0..IN {
                unsafe {
                    *output.get_unchecked_mut(i) +=
                        *weights.get_unchecked(j)
                        * *input.get_unchecked(j);
                }
            }
        }
    }

    #[inline]
    fn calc_state(&self, state: &MathVec<OUT>, output: &mut MathVec<OUT>) {
        for i in 0..OUT {
            let weights = unsafe {(*self.ptr_s.add(i)).as_slice()};

            for j in 0..OUT {
                unsafe {
                    *output.get_unchecked_mut(i) +=
                        *weights.get_unchecked(j)
                        * *state.get_unchecked(j)
                }
            }
        }
    }

    pub fn grad_with_input(
        &self,
        coefficient: &MathVec<OUT>,
        grad: &mut MathVec<IN>
    ) {
        grad.clear();

        for i in 0..OUT {
            let weights = unsafe {(*self.ptr_i.add(i)).as_slice()};

            for j in 0..IN {
                unsafe {
                    *grad.get_unchecked_mut(j) +=
                        *coefficient.get_unchecked(i)
                        * *weights.get_unchecked(j);
                }
            }
        }
    }

    pub fn grad_with_state(
        &self,
        coefficient: &MathVec<OUT>,
        grad: &mut MathVec<OUT>
    ) {
        grad.clear();

        for i in 0..OUT {
            let weights = unsafe {(*self.ptr_s.add(i)).as_slice()};

            for j in 0..OUT {
                unsafe {
                    *grad.get_unchecked_mut(j) +=
                        *coefficient.get_unchecked(i)
                        * *weights.get_unchecked(j);
                }
            }
        }
    }

    pub fn grad_with_weights(
        coefficient: &MathVec<OUT>,
        input: &MathVec<IN>,
        state: Option<&MathVec<OUT>>,
        grad: &mut Self
    ) {
        grad.clear();

        Self::grad_with_weights_b(coefficient, grad);
        Self::grad_with_weights_i(coefficient, input, grad);

        if let Some(state) = state {
            Self::grad_with_weights_s(coefficient, state, grad);
        }
    }

    #[inline]
    fn grad_with_weights_b(coefficient: &MathVec<OUT>, grad: &mut Self) {
        unsafe {coefficient.as_ptr().copy_to(grad.mut_ptr_b, OUT)}
    }

    #[inline]
    fn grad_with_weights_i(
        coefficient: &MathVec<OUT>,
        input: &MathVec<IN>,
        grad: &mut Self
    ) {
        for i in 0..OUT {
            let grad_slice = unsafe {(*grad.mut_ptr_i.add(i)).as_mut_slice()};

            for j in 0..IN {
                unsafe {
                    *grad_slice.get_unchecked_mut(j) +=
                        *coefficient.get_unchecked(i)
                        * *input.get_unchecked(j);
                }
            }
        }
    }

    #[inline]
    fn grad_with_weights_s(
        coefficient: &MathVec<OUT>,
        state: &MathVec<OUT>,
        grad: &mut Self
    ) {
        for i in 0..OUT {
            let grad_slice = unsafe {(*grad.mut_ptr_s.add(i)).as_mut_slice()};

            for j in 0..OUT {
                unsafe {
                    *grad_slice.get_unchecked_mut(j) +=
                        *coefficient.get_unchecked(i)
                        * *state.get_unchecked(j);
                }
            }
        }
    }

    #[inline]
    pub fn copy_from(&mut self, other: &Self) {
        self.as_mut_slice().copy_from_slice(other.as_slice());
    }

    #[inline]
    pub fn copy_to(&self, other: &mut Self) {
        other.as_mut_slice().copy_from_slice(self.as_slice());
    }

    #[inline]
    pub fn read_bytes(&mut self, bytes: &[u8]) -> Option<()> {
        u8_slice_to_f32_slice(bytes, &mut self.body)
    }

    #[inline]
    pub fn write_bytes(&self, buffer: &mut Vec<u8>) -> Option<()> {
        buffer.resize(self.body.len() * SIZE_OF_U32, 0);

        f32_slice_to_u8_slice(&self.body, buffer.as_mut_slice())
    }
}

impl<const OUT: usize, const IN: usize> Default for Weights<OUT, IN> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const OUT: usize, const IN: usize> Clone for Weights<OUT, IN> {
    #[inline]
    fn clone(&self) -> Self {

        let offset_i: usize = OUT;
        let offset_s: usize = offset_i + (OUT * IN);

        let mut body = self.body.clone();

        let ptr_b = body.as_ptr();
        let ptr_i = unsafe {ptr_b.add(offset_i) as *const [f32; IN]};
        let ptr_s = unsafe {ptr_b.add(offset_s) as *const [f32; OUT]};

        let mut_ptr_b = body.as_mut_ptr();
        let mut_ptr_i = unsafe {mut_ptr_b.add(offset_i) as *mut [f32; IN]};
        let mut_ptr_s = unsafe {mut_ptr_b.add(offset_s) as *mut [f32; OUT]};

        Self {
            body: body,

            ptr_b: ptr_b,
            ptr_i: ptr_i,
            ptr_s: ptr_s,

            mut_ptr_b: mut_ptr_b,
            mut_ptr_i: mut_ptr_i,
            mut_ptr_s: mut_ptr_s
        }
    }
}

impl<const OUT: usize, const IN: usize> PartialEq for Weights<OUT, IN> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<const OUT: usize, const IN: usize> Deref for Weights<OUT, IN> {
    type Target = [f32];

    #[inline]
    fn deref(&self) -> &[f32] {&*self.body}
}

impl<const OUT: usize, const IN: usize> DerefMut for Weights<OUT, IN> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [f32] {&mut *self.body}
}

/// Activation function for Neuron.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Activation {
    /// No activation function.
    Linear,

    /// ReLU : `(-inf, +inf) -> [0.0, +inf)`
    ReLU,

    /// SoftSign : `(-inf, +inf) -> (-1.0, +1.0)`
    SoftSign,

    /// Sigmoid : `(-inf, +inf) -> (0.0, +1.0)`
    Sigmoid
}

impl Activation {
    /// Calculates activation function.
    ///
    /// * `x` : Input number.
    /// * _Return_ : Output number.
    #[inline]
    pub fn activate(&self, x: f32) -> f32 {
        match self {
            Self::Linear => x,
            Self::ReLU => x.max(0.0),
            Self::SoftSign => Self::softsign(x),
            Self::Sigmoid => Self::sigmoid(x)
        }
    }

    /// Calculates its derivative function.
    ///
    /// * `x` : Input number.
    /// * _Return_ : Differential coefficient.
    #[inline]
    pub fn d_activate(&self, x: f32) -> f32 {
        match self {
            Self::Linear => 1.0,

            Self::ReLU => if x <= 0.0 {0.0} else {1.0},

            Self::SoftSign => Self::d_softsign(x),

            Self::Sigmoid => Self::d_sigmoid(x)
        }
    }

    #[inline]
    fn softsign_deno(x: f32) -> f32 {
        1.0 + abs(x)
    }

    #[inline]
    fn softsign(x: f32) -> f32 {
        x / Self::softsign_deno(x)
    }

    #[inline]
    fn d_softsign(x: f32) -> f32 {
        let deno = Self::softsign_deno(x);
        (deno * deno).recip()
    }

    #[inline]
    fn sigmoid(x: f32) -> f32 {
        (Self::softsign(x) + 1.0) * 0.5
    }

    #[inline]
    fn d_sigmoid(x: f32) -> f32 {
        Self::d_softsign(x) * 0.5
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layer<const OUT: usize, const IN: usize> {
    weights: Weights<OUT, IN>,
    activation: Activation
}

impl<const OUT: usize, const IN: usize> Layer<OUT, IN> {
    /// Creates Neuron.
    ///
    /// * `activation` : Activation function.
    /// * _Return_ : Neuron.
    #[inline]
    pub fn new(activation: Activation) -> Self {
        Self {
            weights: Weights::<OUT, IN>::default(),
            activation: activation
        }
    }

    #[inline]
    pub fn weights(&self) -> &Weights<OUT, IN> {&self.weights}

    #[inline]
    pub fn mut_weights(&mut self) -> &mut Weights<OUT, IN> {&mut self.weights}

    #[inline]
    pub fn activation(&self) -> &Activation {&self.activation}

    #[inline]
    pub fn mut_activation(&mut self) -> &mut Activation {&mut self.activation}

    #[inline]
    pub fn calc(
        &self,
        input: &MathVec<IN>,
        state: Option<&MathVec<OUT>>,
        output: &mut MathVec<OUT>
    ) {
        self.weights.calc(input, state, output);

        output.iter_mut().for_each(
            |val| {*val = self.activation.activate(*val);}
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MLCache<const OUT: usize, const IN: usize> {
    input: MathVec<IN>,
    state: MathVec<OUT>,
    has_state: bool,

    middle_value: MathVec<OUT>,

    output: MathVec<OUT>
}

impl<const OUT: usize, const IN: usize> MLCache<OUT, IN> {
    #[inline]
    pub fn new() -> Self {
        Self {
            input: MathVec::<IN>::new(),
            state: MathVec::<OUT>::new(),
            has_state: false,

            middle_value: MathVec::<OUT>::new(),

            output: MathVec::<OUT>::new()
        }
    }

    #[inline]
    pub fn calc_error(
        &self,
        train_out: &MathVec<OUT>,
        error: &mut MathVec<OUT>
    ) {
        error.copy_from(&self.output);
        *error -= train_out;
    }

    #[inline]
    pub fn input(&self) -> &MathVec<IN> {&self.input}

    #[inline]
    pub fn state(&self) -> Option<&MathVec<OUT>> {
        self.has_state.then(|| &self.state)
    }

    #[inline]
    pub fn middle_value(&self) -> &MathVec<OUT> {&self.middle_value}

    #[inline]
    pub fn output(&self) -> &MathVec<OUT> {&self.output}
}

#[derive(Debug, Clone, PartialEq)]
pub struct MLLayer<const OUT: usize, const IN: usize> {
    layer: Layer<OUT, IN>,

    total_grad: Weights<OUT, IN>,
    momentum_1: Weights<OUT, IN>,
    momentum_2: MathVec<OUT>,

    tmp_error: MathVec<OUT>,
    tmp_grad: Weights<OUT, IN>
}

impl<const OUT: usize, const IN: usize> MLLayer<OUT, IN> {
    /// Creates Neuron.
    ///
    /// * `activation` : Activation function.
    /// * _Return_ : Neuron.
    #[inline]
    pub fn new(layer: Layer<OUT, IN>) -> Self {
        Self {
            layer: layer,

            total_grad: Weights::<OUT, IN>::default(),
            momentum_1: Weights::<OUT, IN>::default(),
            momentum_2: MathVec::<OUT>::default(),

            tmp_error: MathVec::<OUT>::default(),
            tmp_grad: Weights::<OUT, IN>::default()
        }
    }

    #[inline]
    pub fn layer(&self) -> &Layer<OUT, IN> {&self.layer}

    #[inline]
    pub fn drop(self) -> Layer<OUT, IN> {self.layer}

    #[inline]
    pub fn clear_study_data(&mut self) {
        self.total_grad.clear();
        self.momentum_1.clear();
        self.momentum_2.clear();
    }

    pub fn ready(
        &self,
        input: &MathVec<IN>,
        state: Option<&MathVec<OUT>>,
        cache: &mut MLCache<OUT, IN>
    ) {
        cache.input.copy_from(input);

        match state {
            Some(state) => {
                cache.state.copy_from(state);
                cache.has_state = true;
            },

            None => {
                cache.has_state = false;
            }
        }

        self.layer.weights.calc(input, state, &mut cache.middle_value);

        for i in 0..OUT {
            debug_assert!(cache.output.get(i).is_some());
            debug_assert!(cache.middle_value.get(i).is_some());

            unsafe {
                *cache.output.get_unchecked_mut(i) =
                    self.layer.activation.activate(
                        *cache.middle_value.get_unchecked(i)
                    );
            }
        }
    }

    pub fn study(
        &mut self,
        error: &MathVec<OUT>,
        cache: &MLCache<OUT, IN>,
        input_error: &mut MathVec<IN>,
        state_error: Option<&mut MathVec<OUT>>
    ) {
        self.calc_tmp_error(error, cache);

        // add self.total_grad ----------
        Weights::grad_with_weights(
            &self.tmp_error,
            &cache.input,
            cache.has_state.then(|| &cache.state),
            &mut self.tmp_grad
        );

        for i in 0..self.total_grad.len() {
            unsafe {
                *self.total_grad.get_unchecked_mut(i) +=
                    *self.tmp_grad.get_unchecked(i);
            }
        }

        // calc errors ----------
        self.layer.weights.grad_with_input(&self.tmp_error, input_error);

        if let Some(state_error) = state_error {
            if cache.has_state {
                self.layer.weights.grad_with_state(
                    &self.tmp_error,
                    state_error
                );
            }
        }
    }

    #[inline]
    fn calc_tmp_error(
        &mut self,
        error: &MathVec<OUT>,
        cache: &MLCache<OUT, IN>
    ) {
        for i in 0..OUT {
            debug_assert!(self.tmp_error.get(i).is_some());
            debug_assert!(error.get(i).is_some());
            debug_assert!(cache.middle_value.get(i).is_some());

            unsafe {
                *self.tmp_error.get_unchecked_mut(i) =
                    *error.get_unchecked(i)
                    * self.layer.activation.d_activate(
                        *cache.middle_value.get_unchecked(i)
                    )
            }
        }
    }

    pub fn update(&mut self, rate: f32) {
        self.next_momentum_1();
        self.next_momentum_2();

        self.total_grad.copy_from(&self.momentum_1);

        // calc delta weights.
        for i in 0..OUT {
            debug_assert!(self.momentum_2.get(i).is_some());
            debug_assert!(self.total_grad.input_weights().get(i).is_some());
            debug_assert!(self.total_grad.state_weights().get(i).is_some());
            debug_assert!(self.total_grad.bias().get(i).is_some());

            unsafe {
                let rate_2 = rate
                    / (sqrt(*self.momentum_2.get_unchecked(i)) + f32::EPSILON);

                *self.total_grad.bias_mut().get_unchecked_mut(i) *= rate_2;

                for j in 0..IN {
                    debug_assert!(
                        self.total_grad
                            .input_weights()
                            .get_unchecked(i)
                            .get(j)
                            .is_some()
                    );

                    *self.total_grad
                        .input_weights_mut()
                        .get_unchecked_mut(i)
                        .get_unchecked_mut(j) *= rate_2;
                }

                for j in 0..OUT {
                    debug_assert!(
                        self.total_grad
                            .state_weights()
                            .get_unchecked(i)
                            .get(j)
                            .is_some()
                    );
                    *self.total_grad
                        .state_weights_mut()
                        .get_unchecked_mut(i)
                        .get_unchecked_mut(j) *= rate_2;
                }
            }
        }

        // updates weights.
        for i in 0..self.layer.weights.len() {
            debug_assert!(self.layer.weights.get(i).is_some());
            debug_assert!(self.total_grad.get(i).is_some());

            unsafe {
                *self.layer.weights.get_unchecked_mut(i) -=
                    *self.total_grad.get_unchecked(i);
            }
        }

        self.total_grad.clear();
    }

    #[inline]
    fn next_momentum_1(&mut self) {
        const BETA: f32 = 0.9;
        const BETA_INV: f32 = 1.0 - BETA;

        for i in 0..self.momentum_1.len() {
            debug_assert!(self.momentum_1.get(i).is_some());
            debug_assert!(self.total_grad.get(i).is_some());

            unsafe {
                *self.momentum_1.get_unchecked_mut(i) =
                    (BETA * *self.momentum_1.get_unchecked(i))
                    + (BETA_INV * *self.total_grad.get_unchecked(i));
            }
        }
    }

    #[inline]
    fn next_momentum_2(&mut self) {
        const BETA: f32 = 0.999;
        const BETA_INV: f32 = 1.0 - BETA;

        for i in 0..OUT {
            debug_assert!(self.momentum_2.get(i).is_some());
            debug_assert!(self.total_grad.input_weights().get(i).is_some());
            debug_assert!(self.total_grad.state_weights().get(i).is_some());
            debug_assert!(self.total_grad.bias().get(i).is_some());

            let mut dot_product: f32 = 0.0;

            unsafe {
                let bias = *self.total_grad.bias().get_unchecked(i);
                dot_product += bias * bias;

                for j in 0..IN {
                    debug_assert!(
                        self.total_grad
                            .input_weights()
                            .get_unchecked(i)
                            .get(j)
                            .is_some()
                    );

                    let val = *self.total_grad
                        .input_weights()
                        .get_unchecked(i)
                        .get_unchecked(j);

                    dot_product += val * val;
                }

                for j in 0..OUT {
                    debug_assert!(
                        self.total_grad
                            .state_weights()
                            .get_unchecked(i)
                            .get(j)
                            .is_some()
                    );

                    let val = *self.total_grad
                        .state_weights()
                        .get_unchecked(i)
                        .get_unchecked(j);

                    dot_product += val * val;
                }

                *self.momentum_2.get_unchecked_mut(i) =
                    (BETA * *self.momentum_2.get_unchecked(i))
                    + (BETA_INV * dot_product);
            }

        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChobitAI<const OUT: usize, const MIDDLE: usize, const IN: usize> {
    middle_layer: Layer<MIDDLE, IN>,
    output_layer: Layer<OUT, MIDDLE>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> ChobitAI<OUT, MIDDLE, IN> {
    #[inline]
    pub fn new(activation: Activation) -> Self {
        Self {
            middle_layer: Layer::<MIDDLE, IN>::new(Activation::ReLU),
            output_layer: Layer::<OUT, MIDDLE>::new(activation)
        }
    }

    #[inline]
    pub fn middle_layer(&self) -> &Layer<MIDDLE, IN> {&self.middle_layer}

    #[inline]
    pub fn middle_layer_mut(&mut self) -> &mut Layer<MIDDLE, IN> {
        &mut self.middle_layer
    }

    #[inline]
    pub fn output_layer(&self) -> &Layer<OUT, MIDDLE> {&self.output_layer}

    #[inline]
    pub fn output_layer_mut(&mut self) -> &mut Layer<OUT, MIDDLE> {
        &mut self.output_layer
    }

    #[inline]
    pub fn calc(
        &self,
        input: &MathVec<IN>,
        output: &mut MathVec<OUT>,
        working_buffer: &mut MathVec<MIDDLE>
    ) {
        self.middle_layer.calc(input, None, working_buffer);
        self.output_layer.calc(working_buffer, None, output);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChobitMLAI<const OUT: usize, const MIDDLE: usize, const IN: usize> {
    middle_layer: MLLayer<MIDDLE, IN>,
    output_layer: MLLayer<OUT, MIDDLE>,

    middle_cache: MLCache<MIDDLE, IN>,
    output_cache: MLCache<OUT, MIDDLE>,

    input_error: MathVec<IN>,
    middle_error: MathVec<MIDDLE>,
    output_error: MathVec<OUT>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> ChobitMLAI<OUT, MIDDLE, IN> {
    #[inline]
    pub fn new(ai: ChobitAI<OUT, MIDDLE, IN>) -> Self {
        let ChobitAI::<OUT, MIDDLE, IN> {middle_layer, output_layer} = ai;

        Self {
            middle_layer: MLLayer::<MIDDLE, IN>::new(middle_layer),
            output_layer: MLLayer::<OUT, MIDDLE>::new(output_layer),

            middle_cache: MLCache::<MIDDLE, IN>::new(),
            output_cache: MLCache::<OUT, MIDDLE>::new(),

            input_error: MathVec::<IN>::new(),
            middle_error: MathVec::<MIDDLE>::new(),
            output_error: MathVec::<OUT>::new(),
        }
    }

    #[inline]
    pub fn drop(self) -> ChobitAI<OUT, MIDDLE, IN> {
        let Self {middle_layer, output_layer, ..} = self;

        ChobitAI::<OUT, MIDDLE, IN> {
            middle_layer: middle_layer.drop(),
            output_layer: output_layer.drop()
        }
    }

    pub fn study(&mut self, train_in: &MathVec<IN>, train_out: &MathVec<OUT>) {
        self.middle_layer.ready(train_in, None, &mut self.middle_cache);

        self.output_layer.ready(
            &self.middle_cache.output,
            None,
            &mut self.output_cache
        );

        self.output_cache.calc_error(train_out, &mut self.output_error);

        self.output_layer.study(
            &self.output_error,
            &self.output_cache,
            &mut self.middle_error,
            None
        );

        self.middle_layer.study(
            &self.middle_error,
            &self.middle_cache,
            &mut self.input_error,
            None
        );
    }

    #[inline]
    pub fn update(&mut self, rate: f32) {
        self.middle_layer.update(rate);
        self.output_layer.update(rate);
    }
}

//#[derive(Debug, Clone, PartialEq)]
//pub struct LSTM<const OUT: usize, const IN: usize> {
//    f_gate: Layer<OUT, IN>,
//    i_gate: Layer<OUT, IN>,
//    o_gate: Layer<OUT, IN>,
//    c_gate: Layer<OUT, IN>,
//    tanh: Activation
//}
//
//impl<const OUT: usize, const IN: usize> LSTM<OUT, IN> {
//    pub fn new() -> Self {
//        Self {
//            f_gate: Layer::<OUT, IN>::new(Activation::Sigmoid),
//            i_gate: Layer::<OUT, IN>::new(Activation::Sigmoid),
//            o_gate: Layer::<OUT, IN>::new(Activation::Sigmoid),
//            c_gate: Layer::<OUT, IN>::new(Activation::SoftSign),
//
//            tanh: Activation::SoftSign
//        }
//    }
//
//    #[inline]
//    pub fn f_gate(&self) -> &Layer<OUT, IN> {&self.f_gate}
//
//    #[inline]
//    pub fn f_gate_mut(&mut self) -> &mut Layer<OUT, IN> {&mut self.f_gate}
//
//    #[inline]
//    pub fn i_gate(&self) -> &Layer<OUT, IN> {&self.i_gate}
//
//    #[inline]
//    pub fn i_gate_mut(&mut self) -> &mut Layer<OUT, IN> {&mut self.i_gate}
//
//    #[inline]
//    pub fn o_gate(&self) -> &Layer<OUT, IN> {&self.o_gate}
//
//    #[inline]
//    pub fn o_gate_mut(&mut self) -> &mut Layer<OUT, IN> {&mut self.o_gate}
//
//    #[inline]
//    pub fn c_gate(&self) -> &Layer<OUT, IN> {&self.c_gate}
//
//    #[inline]
//    pub fn c_gate_mut(&mut self) -> &mut Layer<OUT, IN> {&mut self.c_gate}
//
//    pub fn calc(
//        &self,
//        input: &MathVec<IN>,
//        prev_output: &MathVec<OUT>,
//        prev_cell: &MathVec<OUT>,
//        output: &mut MathVec<OUT>,
//        cell: &mut MathVec<OUT>,
//        working_buffer_1: &mut MathVec<OUT>,
//        working_buffer_2: &mut MathVec<OUT>
//    ) {
//        // cell = (f_gate * prev_cell) + (i_gate * c_gate);
//        self.f_gate.calc(input, Some(prev_output), cell);
//        self.i_gate.calc(input, Some(prev_output), working_buffer_1);
//        self.c_gate.calc(input, Some(prev_output), working_buffer_2);
//
//        for i in 0..OUT {
//            debug_assert!(cell.get(i).is_some());
//            debug_assert!(prev_cell.get(i).is_some());
//            debug_assert!(working_buffer_1.get(i).is_some());
//            debug_assert!(working_buffer_2.get(i).is_some());
//
//            unsafe {
//                *cell.get_unchecked_mut(i) *= *prev_cell.get_unchecked(i);
//                *cell.get_unchecked_mut(i) +=
//                    *working_buffer_1.get_unchecked(i)
//                    * *working_buffer_2.get_unchecked(i);
//            }
//        }
//
//        // output = o_gate * tanh(cell)
//        self.o_gate.calc(input, Some(prev_output), output);
//
//        for i in 0..OUT {
//            debug_assert!(cell.get(i).is_some());
//            debug_assert!(output.get(i).is_some());
//
//            unsafe {
//                *output.get_unchecked_mut(i) *=
//                    self.tanh.activate(*cell.get_unchecked(i));
//            }
//        }
//    }
//}
//
//#[derive(Debug, Clone, PartialEq)]
//pub struct MLLSTMCache<const OUT: usize, const IN: usize> {
//    input: MathVec<IN>,
//    prev_output: MathVec<OUT>,
//    prev_cell: MathVec<OUT>,
//
//    f_gate_cache: MLCache<OUT, IN>,
//    i_gate_cache: MLCache<OUT, IN>,
//    o_gate_cache: MLCache<OUT, IN>,
//    c_gate_cache: MLCache<OUT, IN>,
//
//    tanh_c: MathVec<OUT>,
//    d_tanh_c: MathVec<OUT>,
//
//    output: MathVec<OUT>,
//    cell: MathVec<OUT>
//}
//
//impl<const OUT: usize, const IN: usize> MLLSTMCache<OUT, IN> {
//    #[inline]
//    pub fn new() -> Self {
//        Self {
//            input: MathVec::<IN>::new(),
//            prev_output: MathVec::<OUT>::new(),
//            prev_cell: MathVec::<OUT>::new(),
//
//            f_gate_cache: MLCache::<OUT, IN>::new(),
//            i_gate_cache: MLCache::<OUT, IN>::new(),
//            o_gate_cache: MLCache::<OUT, IN>::new(),
//            c_gate_cache: MLCache::<OUT, IN>::new(),
//
//            tanh_c: MathVec::<OUT>::new(),
//            d_tanh_c: MathVec::<OUT>::new(),
//
//            output: MathVec::<OUT>::new(),
//            cell: MathVec::<OUT>::new()
//        }
//    }
//
//    #[inline]
//    pub fn calc_feedback(
//        &self,
//        train_out: &MathVec<OUT>,
//        feedback: &mut MathVec<OUT>
//    ) {
//        feedback.copy_from(&self.output);
//        *feedback -= train_out;
//    }
//
//    #[inline]
//    pub fn input(&self) -> &MathVec<IN> {&self.input}
//
//    #[inline]
//    pub fn prev_output(&self) -> &MathVec<OUT> {&self.prev_output}
//
//    #[inline]
//    pub fn prev_cell(&self) -> &MathVec<OUT> {&self.prev_cell}
//
//    #[inline]
//    pub fn f_gate_cache(&self) -> &MLCache<OUT, IN> {&self.f_gate_cache}
//
//    #[inline]
//    pub fn i_gate_cache(&self) -> &MLCache<OUT, IN> {&self.i_gate_cache}
//
//    #[inline]
//    pub fn o_gate_cache(&self) -> &MLCache<OUT, IN> {&self.o_gate_cache}
//
//    #[inline]
//    pub fn c_gate_cache(&self) -> &MLCache<OUT, IN> {&self.c_gate_cache}
//
//    #[inline]
//    pub fn tanh_c(&self) -> &MathVec<OUT> {&self.tanh_c}
//
//    #[inline]
//    pub fn d_tanh_c(&self) -> &MathVec<OUT> {&self.d_tanh_c}
//
//    #[inline]
//    pub fn output(&self) -> &MathVec<OUT> {&self.output}
//
//    #[inline]
//    pub fn cell(&self) -> &MathVec<OUT> {&self.cell}
//}
//
//#[derive(Debug, Clone, PartialEq)]
//pub struct MLLSTM<const OUT: usize, const IN: usize> {
//    f_gate: MLLayer<OUT, IN>,
//    i_gate: MLLayer<OUT, IN>,
//    o_gate: MLLayer<OUT, IN>,
//    c_gate: MLLayer<OUT, IN>,
//    tanh: Activation,
//
//    next_feedback_for_input_f: MathVec<IN>,
//    next_feedback_for_input_i: MathVec<IN>,
//    next_feedback_for_input_o: MathVec<IN>,
//    next_feedback_for_input_c: MathVec<IN>,
//
//    next_feedback_for_prev_output_f: MathVec<OUT>,
//    next_feedback_for_prev_output_i: MathVec<OUT>,
//    next_feedback_for_prev_output_o: MathVec<OUT>,
//    next_feedback_for_prev_output_c: MathVec<OUT>,
//
//    tmp_feedback: MathVec<OUT>,
//}
//
//impl<const OUT: usize, const IN: usize> MLLSTM<OUT, IN> {
//    #[inline]
//    pub fn new(lstm: LSTM<OUT, IN>) -> Self {
//        let LSTM::<OUT, IN> {f_gate, i_gate, o_gate, c_gate, tanh} = lstm;
//
//        Self {
//            f_gate: MLLayer::<OUT, IN>::new(f_gate),
//            i_gate: MLLayer::<OUT, IN>::new(i_gate),
//            o_gate: MLLayer::<OUT, IN>::new(o_gate),
//            c_gate: MLLayer::<OUT, IN>::new(c_gate),
//
//            tanh: tanh,
//
//            next_feedback_for_input_f: MathVec::<IN>::new(),
//            next_feedback_for_input_i: MathVec::<IN>::new(),
//            next_feedback_for_input_o: MathVec::<IN>::new(),
//            next_feedback_for_input_c: MathVec::<IN>::new(),
//
//            next_feedback_for_prev_output_f: MathVec::<OUT>::new(),
//            next_feedback_for_prev_output_i: MathVec::<OUT>::new(),
//            next_feedback_for_prev_output_o: MathVec::<OUT>::new(),
//            next_feedback_for_prev_output_c: MathVec::<OUT>::new(),
//
//            tmp_feedback: MathVec::<OUT>::new()
//        }
//    }
//
//    #[inline]
//    pub fn drop(self) -> LSTM<OUT, IN> {
//        let Self {f_gate, i_gate, o_gate, c_gate, tanh, ..} = self;
//
//        LSTM::<OUT, IN> {
//            f_gate: f_gate.drop(),
//            i_gate: i_gate.drop(),
//            o_gate: o_gate.drop(),
//            c_gate: c_gate.drop(),
//
//            tanh: tanh
//        }
//    }
//
//    #[inline]
//    pub fn clear_study_data(&mut self) {
//        self.f_gate.clear_study_data();
//        self.i_gate.clear_study_data();
//        self.o_gate.clear_study_data();
//        self.c_gate.clear_study_data();
//    }
//
//    pub fn ready(
//        &self,
//        input: &MathVec<IN>,
//        prev_output: &MathVec<OUT>,
//        prev_cell: &MathVec<OUT>,
//        cache: &mut MLLSTMCache<OUT, IN>
//    ) {
//        cache.input.copy_from(input);
//        cache.prev_output.copy_from(prev_output);
//        cache.prev_cell.copy_from(prev_cell);
//
//        self.f_gate.ready(input, Some(prev_output), &mut cache.f_gate_cache);
//        self.i_gate.ready(input, Some(prev_output), &mut cache.i_gate_cache);
//        self.o_gate.ready(input, Some(prev_output), &mut cache.o_gate_cache);
//        self.c_gate.ready(input, Some(prev_output), &mut cache.c_gate_cache);
//
//        for i in 0..OUT {
//            debug_assert!(cache.cell.get(i).is_some());
//            debug_assert!(cache.tanh_c.get(i).is_some());
//            debug_assert!(cache.d_tanh_c.get(i).is_some());
//            debug_assert!(cache.output.get(i).is_some());
//            debug_assert!(prev_cell.get(i).is_some());
//            debug_assert!(cache.f_gate_cache.output.get(i).is_some());
//            debug_assert!(cache.i_gate_cache.output.get(i).is_some());
//            debug_assert!(cache.o_gate_cache.output.get(i).is_some());
//            debug_assert!(cache.c_gate_cache.output.get(i).is_some());
//
//            unsafe {
//                *cache.cell.get_unchecked_mut(i) =
//                    (*prev_cell.get_unchecked(i)
//                        * *cache.f_gate_cache.output.get_unchecked(i))
//                    + (*cache.i_gate_cache.output.get_unchecked(i)
//                        * *cache.c_gate_cache.output.get_unchecked(i));
//
//                *cache.tanh_c.get_unchecked_mut(i) =
//                    self.tanh.activate(*cache.cell.get_unchecked(i));
//
//                *cache.d_tanh_c.get_unchecked_mut(i) =
//                    self.tanh.d_activate(*cache.cell.get_unchecked(i));
//
//                *cache.output.get_unchecked_mut(i) =
//                    *cache.o_gate_cache.output.get_unchecked(i)
//                    * self.tanh.activate(*cache.cell.get_unchecked(i));
//            }
//        }
//    }
//
//    pub fn study(
//        &mut self,
//        feedback: &MathVec<OUT>,
//        cache: &MLLSTMCache<OUT, IN>,
//        next_feedback_for_input: &mut MathVec<IN>,
//        next_feedback_for_prev_output: &mut MathVec<OUT>,
//        next_feedback_for_prev_cell: &mut MathVec<OUT>
//    ) {
//        self.study_f_gate(feedback, cache);
//        self.study_i_gate(feedback, cache);
//        self.study_o_gate(feedback, cache);
//        self.study_c_gate(feedback, cache);
//
//        next_feedback_for_input.copy_from(&self.next_feedback_for_input_o);
//        *next_feedback_for_input += &self.next_feedback_for_input_f;
//        *next_feedback_for_input += &self.next_feedback_for_input_i;
//        *next_feedback_for_input += &self.next_feedback_for_input_c;
//
//        next_feedback_for_prev_output.copy_from(
//            &self.next_feedback_for_prev_output_o
//        );
//        *next_feedback_for_prev_output +=
//            &self.next_feedback_for_prev_output_f;
//        *next_feedback_for_prev_output +=
//            &self.next_feedback_for_prev_output_i;
//        *next_feedback_for_prev_output +=
//            &self.next_feedback_for_prev_output_c;
//
//        self.grad_with_prev_cell(feedback, cache, next_feedback_for_prev_cell);
//    }
//
//    fn study_f_gate(
//        &mut self,
//        feedback: &MathVec<OUT>,
//        cache: &MLLSTMCache<OUT, IN>
//    ) {
//        for i in 0..OUT {
//            debug_assert!(feedback.get(i).is_some());
//            debug_assert!(self.tmp_feedback.get(i).is_some());
//            debug_assert!(cache.o_gate_cache.output.get(i).is_some());
//            debug_assert!(cache.d_tanh_c.get(i).is_some());
//            debug_assert!(cache.cell.get(i).is_some());
//
//            unsafe {
//                *self.tmp_feedback.get_unchecked_mut(i) =
//                    *feedback.get_unchecked(i)
//                    * *cache.o_gate_cache.output.get_unchecked(i)
//                    * *cache.d_tanh_c.get_unchecked(i)
//                    * *cache.cell.get_unchecked(i);
//            }
//        }
//
//        self.f_gate.study(
//            &self.tmp_feedback,
//            &cache.f_gate_cache,
//            &mut self.next_feedback_for_input_f,
//            Some(&mut self.next_feedback_for_prev_output_f)
//        );
//    }
//
//    fn study_i_gate(
//        &mut self,
//        feedback: &MathVec<OUT>,
//        cache: &MLLSTMCache<OUT, IN>
//    ) {
//        for i in 0..OUT {
//            debug_assert!(feedback.get(i).is_some());
//            debug_assert!(self.tmp_feedback.get(i).is_some());
//            debug_assert!(cache.o_gate_cache.output.get(i).is_some());
//            debug_assert!(cache.d_tanh_c.get(i).is_some());
//            debug_assert!(cache.c_gate_cache.output.get(i).is_some());
//
//            unsafe {
//                *self.tmp_feedback.get_unchecked_mut(i) =
//                    *feedback.get_unchecked(i)
//                    * *cache.o_gate_cache.output.get_unchecked(i)
//                    * *cache.d_tanh_c.get_unchecked(i)
//                    * *cache.c_gate_cache.output.get_unchecked(i);
//            }
//        }
//
//        self.i_gate.study(
//            &self.tmp_feedback,
//            &cache.f_gate_cache,
//            &mut self.next_feedback_for_input_i,
//            Some(&mut self.next_feedback_for_prev_output_i)
//        );
//    }
//
//    fn study_o_gate(
//        &mut self,
//        feedback: &MathVec<OUT>,
//        cache: &MLLSTMCache<OUT, IN>
//    ) {
//        for i in 0..OUT {
//            debug_assert!(feedback.get(i).is_some());
//            debug_assert!(self.tmp_feedback.get(i).is_some());
//            debug_assert!(cache.tanh_c.get(i).is_some());
//            unsafe {
//                *self.tmp_feedback.get_unchecked_mut(i) =
//                    *feedback.get_unchecked(i)
//                    * *cache.tanh_c.get_unchecked(i);
//            }
//        }
//
//        self.o_gate.study(
//            &self.tmp_feedback,
//            &cache.f_gate_cache,
//            &mut self.next_feedback_for_input_o,
//            Some(&mut self.next_feedback_for_prev_output_o)
//        );
//    }
//
//    fn study_c_gate(
//        &mut self,
//        feedback: &MathVec<OUT>,
//        cache: &MLLSTMCache<OUT, IN>
//    ) {
//        for i in 0..OUT {
//            debug_assert!(feedback.get(i).is_some());
//            debug_assert!(self.tmp_feedback.get(i).is_some());
//            debug_assert!(cache.o_gate_cache.output.get(i).is_some());
//            debug_assert!(cache.d_tanh_c.get(i).is_some());
//            debug_assert!(cache.i_gate_cache.output.get(i).is_some());
//
//            unsafe {
//                *self.tmp_feedback.get_unchecked_mut(i) =
//                    *feedback.get_unchecked(i)
//                    * *cache.o_gate_cache.output.get_unchecked(i)
//                    * *cache.d_tanh_c.get_unchecked(i)
//                    * *cache.i_gate_cache.output.get_unchecked(i);
//            }
//        }
//
//        self.c_gate.study(
//            &self.tmp_feedback,
//            &cache.f_gate_cache,
//            &mut self.next_feedback_for_input_c,
//            Some(&mut self.next_feedback_for_prev_output_c)
//        );
//    }
//
//    fn grad_with_prev_cell(
//        &self,
//        feedback: &MathVec<OUT>,
//        cache: &MLLSTMCache<OUT, IN>,
//        next_feedback_for_prev_cell: &mut MathVec<OUT>
//    ) {
//        for i in 0..OUT {
//            debug_assert!(feedback.get(i).is_some());
//            debug_assert!(next_feedback_for_prev_cell.get(i).is_some());
//            debug_assert!(cache.o_gate_cache.output.get(i).is_some());
//            debug_assert!(cache.d_tanh_c.get(i).is_some());
//            debug_assert!(cache.f_gate_cache.output.get(i).is_some());
//
//            unsafe {
//                *next_feedback_for_prev_cell.get_unchecked_mut(i) =
//                    *feedback.get_unchecked(i)
//                    * *cache.o_gate_cache.output.get_unchecked(i)
//                    * *cache.d_tanh_c.get_unchecked(i)
//                    * *cache.f_gate_cache.output.get_unchecked(i);
//            }
//        }
//    }
//
//    #[inline]
//    pub fn update(&mut self, rate: f32) {
//        self.f_gate.update(rate);
//        self.i_gate.update(rate);
//        self.o_gate.update(rate);
//        self.c_gate.update(rate);
//    }
//}
//
//#[derive(Debug, Clone, PartialEq)]
//pub struct Encoder<const OUT: usize, const IN: usize> {
//    body: LSTM<OUT, IN>
//}
//
//impl<const OUT: usize, const IN: usize> Encoder<OUT, IN> {
//    #[inline]
//    pub fn new() -> Self {
//        Self {
//            body: LSTM::<OUT, IN>::new()
//        }
//    }
//
//    #[inline]
//    pub fn body(&self) -> &LSTM<OUT, IN> {&self.body}
//
//    pub fn calc(
//        &self,
//        input: &[MathVec<IN>],
//        prev_output: &MathVec<OUT>,
//        prev_cell: &MathVec<OUT>,
//        output: &mut MathVec<OUT>,
//        cell: &mut MathVec<OUT>,
//        working_buffer_1: &mut MathVec<OUT>,
//        working_buffer_2: &mut MathVec<OUT>,
//        working_buffer_3: &mut MathVec<OUT>,
//        working_buffer_4: &mut MathVec<OUT>
//    ) {
//        working_buffer_1.copy_from(prev_output);
//        let prev_output = working_buffer_1;
//
//        working_buffer_2.copy_from(prev_cell);
//        let prev_cell = working_buffer_2;
//
//        input.iter().for_each(|input| {
//            self.body.calc(
//                input,
//                prev_output,
//                prev_cell,
//                output,
//                cell,
//                working_buffer_3,
//                working_buffer_4
//            );
//
//            prev_output.copy_from(output);
//            prev_cell.copy_from(cell);
//        });
//    }
//}
