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

use alloc::{boxed::Box, vec};

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
    slice::{from_raw_parts, from_raw_parts_mut}
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
            output.as_mut_ptr().copy_from(self.ptr_b, OUT)
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
                        * *input.get_unchecked(j)
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
        feedback: &MathVec<OUT>,
        grad: &mut MathVec<IN>
    ) {
        grad.clear();

        for i in 0..OUT {
            let weights = unsafe {(*self.ptr_i.add(i)).as_slice()};

            for j in 0..IN {
                unsafe {
                    *grad.get_unchecked_mut(j) +=
                        *feedback.get_unchecked(i)
                        * *weights.get_unchecked(j);
                }
            }
        }
    }

    pub fn grad_with_state(
        &self,
        feedback: &MathVec<OUT>,
        grad: &mut MathVec<OUT>
    ) {
        grad.clear();

        for i in 0..OUT {
            let weights = unsafe {(*self.ptr_s.add(i)).as_slice()};

            for j in 0..OUT {
                unsafe {
                    *grad.get_unchecked_mut(j) +=
                        *feedback.get_unchecked(i)
                        * *weights.get_unchecked(j);
                }
            }
        }
    }

    pub fn grad_with_weights(
        feedback: &MathVec<OUT>,
        input: &MathVec<IN>,
        state: Option<&MathVec<OUT>>,
        grad: &mut Self
    ) {
        grad.clear();

        Self::grad_with_weights_b(feedback, grad);
        Self::grad_with_weights_i(feedback, input, grad);

        if let Some(state) = state {
            Self::grad_with_weights_s(feedback, state, grad);
        }
    }

    #[inline]
    fn grad_with_weights_b(feedback: &MathVec<OUT>, grad: &mut Self) {
        unsafe {feedback.as_ptr().copy_to(grad.mut_ptr_b, OUT)}
    }

    #[inline]
    fn grad_with_weights_i(
        feedback: &MathVec<OUT>,
        input: &MathVec<IN>,
        grad: &mut Self
    ) {
        for i in 0..OUT {
            let grad_slice = unsafe {(*grad.mut_ptr_i.add(i)).as_mut_slice()};

            for j in 0..IN {
                unsafe {
                    *grad_slice.get_unchecked_mut(j) +=
                        *feedback.get_unchecked(i)
                        * *input.get_unchecked(j);
                }
            }
        }
    }

    #[inline]
    fn grad_with_weights_s(
        feedback: &MathVec<OUT>,
        state: &MathVec<OUT>,
        grad: &mut Self
    ) {
        for i in 0..OUT {
            let grad_slice = unsafe {(*grad.mut_ptr_s.add(i)).as_mut_slice()};

            for j in 0..OUT {
                unsafe {
                    *grad_slice.get_unchecked_mut(j) +=
                        *feedback.get_unchecked(i)
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

///// Neuron that is a part of AI.
/////
///// `N` : Dimension.
//#[derive(Debug, Clone, PartialEq)]
//pub struct Neuron<const N: usize> {
//    weights: Weights<N>,
//    activation: Activation,
//
//    total_grad: Weights<N>,
//    momentum_1: Weights<N>,
//    momentum_2: f32,
//
//    grad_w: Weights<N>,
//    grad_x: MathVec<N>,
//
//    total_grad_2: Weights<N>,
//
//    cache_input: MathVec<N>,
//    cache_middle_value: f32,
//    cache_output: f32
//}
//
//impl<const N: usize> Neuron<N> {
//    /// Creates Neuron.
//    ///
//    /// * `activation` : Activation function.
//    /// * _Return_ : Neuron.
//    #[inline]
//    pub fn new(activation: Activation) -> Self {
//        Self {
//            weights: Weights::<N>::default(),
//            activation: activation,
//
//            total_grad: Weights::<N>::default(),
//            momentum_1: Weights::<N>::default(),
//            momentum_2: 0.0,
//
//            grad_w: Weights::<N>::default(),
//            grad_x: MathVec::<N>::default(),
//
//            total_grad_2: Weights::<N>::default(),
//
//            cache_input: MathVec::<N>::default(),
//            cache_middle_value: 0.0,
//            cache_output: 0.0
//        }
//    }
//
//    /// Gets immutable weights.
//    ///
//    /// * _Return_ : Weights.
//    #[inline]
//    pub fn weights(&self) -> &Weights<N> {&self.weights}
//
//    /// Gets mutable weights.
//    ///
//    /// * _Return_ : Weights.
//    #[inline]
//    pub fn weights_mut(&mut self) -> &mut Weights<N> {&mut self.weights}
//
//    /// Gets immutable activation function.
//    ///
//    /// * _Return_ : Activation function.
//    #[inline]
//    pub fn activation(&self) -> Activation {self.activation}
//
//    /// Gets mutable activation function.
//    ///
//    /// * _Return_ : Activation function.
//    #[inline]
//    pub fn activation_mut (&mut self) -> &mut Activation {&mut self.activation}
//
//    /// Gets immutable total gradients.
//    ///
//    /// * _Return_ : Total gradients.
//    #[inline]
//    pub fn total_grad(&self) -> &Weights<N> {&self.total_grad}
//
//    /// Gets mutable total gradients.
//    ///
//    /// * _Return_ : Total gradients.
//    #[inline]
//    pub fn total_grad_mut(&mut self) -> &mut Weights<N> {&mut self.total_grad}
//
//    /// Gets immutable momentum of gradients.
//    ///
//    /// * _Return_ : Momentum.
//    #[inline]
//    pub fn momentum_1(&self) -> &Weights<N> {&self.momentum_1}
//
//    /// Gets mutable momentum of gradients.
//    ///
//    /// * _Return_ : Momentum.
//    #[inline]
//    pub fn momentum_1_mut(&mut self) -> &mut Weights<N> {&mut self.momentum_1}
//
//    /// Gets immutable momentum for learning rate.
//    ///
//    /// * _Return_ : Momentum.
//    #[inline]
//    pub fn momentum_2(&self) -> f32 {self.momentum_2}
//
//    /// Sets momentum for learning rate.
//    ///
//    /// * 'momentum_2' : New momentum.
//    #[inline]
//    pub fn set_momentum_2(&mut self, momentum_2: f32) {
//        self.momentum_2 = abs(momentum_2);
//    }
//
//    /// Gets cache of input
//    ///
//    /// * _Return_ : Cache of input.
//    #[inline]
//    pub fn cache_input(&self) -> &MathVec<N> {&self.cache_input}
//
//    /// Gets cache of output value before activating.
//    ///
//    /// * _Return_ : Cache of output value before activating.
//    #[inline]
//    pub fn cache_middle_value(&self) -> f32 {self.cache_middle_value}
//
//
//    /// Gets cache of output.
//    ///
//    /// * _Return_ : Cache of output.
//    #[inline]
//    pub fn cache_output(&self) -> f32 {self.cache_output}
//
//    /// Calculates input by linear function and activation function.
//    ///
//    /// * `input` : Input vector.
//    /// * _Return_ : Output number.
//    #[inline]
//    pub fn calc(&mut self, input: &MathVec<N>) -> f32 {
//        self.cache_input.copy_from(input);
//        self.cache_middle_value = &self.weights * input;
//
//        self.cache_output = self.activation.activate(self.cache_middle_value);
//
//        self.cache_output
//    }
//
//    /// Forgets data of gradients and momenta.
//    #[inline]
//    pub fn clear_study_data(&mut self) {
//        self.total_grad.clear();
//        self.momentum_1.clear();
//        self.momentum_2 = 0.0;
//    }
//
//    /// Studies gradients by Backpropagation.
//    ///
//    /// It only studies gradients. It doesn't update weights yet.  
//    /// __Note: Must call [calc](Neuron::calc) before calling this function.__
//    ///
//    /// * `feedback` : Gradient of loss function that is propagated back from next layer.
//    /// * _Return_ : Gradient of loss function that should propagate to previous layer.
//    ///
//    /// ```text
//    ///               feedback                      Return
//    /// Next_Neuron ------------> This_Neuron    ------------> Previous_Neuron
//    /// Next_Neuron ---|     |--> Sibling_Neuron ---|     |--> Previous_Neuron
//    /// Next_Neuron ---|     |--> Sibling_Neuron ---|     |--> Previous_Neuron
//    /// Next_Neuron ---/     \--> Sibling_Neuron ---/     \--> Previous_Neuron
//    /// ```
//    pub fn study(&mut self, feedback: f32) -> &MathVec<N> {
//        let factor =
//            feedback * self.activation.d_activate(self.cache_middle_value);
//
//        self.grad_w.w_mut().copy_from(&self.cache_input);
//        *self.grad_w.w_mut() *= factor;
//        *self.grad_w.b_mut() = factor;
//
//        self.grad_x.copy_from(self.weights.w());
//        self.grad_x *= factor;
//
//        self.total_grad += &self.grad_w;
//
//        &self.grad_x
//    }
//
//    /// Updates Weights to use studied gradients.
//    ///
//    /// * `rate` : Learning rate.
//    pub fn update(&mut self, rate: f32) {
//        self.next_momentum_1();
//        self.next_momentum_2();
//
//        let rate = self.calc_rate(rate);
//
//        self.total_grad.copy_from(&self.momentum_1);
//        self.total_grad *= rate;
//
//        self.weights -= &self.total_grad;
//
//        self.total_grad.clear();
//    }
//
//    #[inline]
//    fn next_momentum_1(&mut self) {
//        const BETA: f32 = 0.9;
//        const BETA_INV: f32 = 1.0 - BETA;
//
//        self.momentum_1 *= BETA;
//
//        self.total_grad_2.copy_from(&self.total_grad);
//        self.total_grad_2 *= BETA_INV;
//
//        self.momentum_1 += &self.total_grad_2;
//    }
//
//    #[inline]
//    fn next_momentum_2(&mut self) {
//        const BETA: f32 = 0.999;
//        const BETA_INV: f32 = 1.0 - BETA;
//
//        self.momentum_2 *= BETA;
//        self.momentum_2 += (&self.total_grad * &self.total_grad) * BETA_INV;
//    }
//
//    #[inline]
//    fn calc_rate(&self, rate: f32) -> f32 {
//        rate / (sqrt(self.momentum_2) + f32::EPSILON)
//    }
//}
//
///// Layer of AI.
/////
///// * `OUT` : Dimension of output
///// * `IN` : Dimension of input.
//#[derive(Debug, Clone, PartialEq)]
//pub struct Layer<const OUT: usize, const IN: usize> {
//    neurons: Box<[Neuron<IN>]>,
//
//    feedback_next: MathVec<IN>,
//
//    cache_input: MathVec<IN>,
//    cache_output: MathVec<OUT>
//}
//
//impl<const OUT: usize, const IN: usize> Layer<OUT, IN> {
//    /// Creates Layer.
//    ///
//    /// * `activation` : Activation function.
//    /// * _Return_ : Layer.
//    #[inline]
//    pub fn new(acitvation: Activation) -> Self {
//        Self {
//            neurons:
//                vec![Neuron::<IN>::new(acitvation); OUT].into_boxed_slice(),
//
//            feedback_next: MathVec::<IN>::default(),
//
//            cache_input: MathVec::<IN>::default(),
//            cache_output: MathVec::<OUT>::default()
//        }
//    }
//
//    /// Gets immutable neurons.
//    ///
//    /// _Return_ : Neurons.
//    #[inline]
//    pub fn neurons(&self) -> &[Neuron<IN>] {&*self.neurons}
//
//    /// Gets mutable neurons.
//    ///
//    /// _Return_ : Neurons.
//    #[inline]
//    pub fn neurons_mut(&mut self) -> &mut [Neuron<IN>] {&mut *self.neurons}
//
//    /// Gets cache of input.
//    ///
//    /// * _Return_ : Cache of input.
//    #[inline]
//    pub fn cache_input(&self) -> &MathVec<IN> {&self.cache_input}
//
//    /// Gets cache of output.
//    ///
//    /// * _Return_ : Cache of output.
//    #[inline]
//    pub fn cache_output(&self) -> &MathVec<OUT> {&self.cache_output}
//
//    /// Calculates input.
//    ///
//    /// * `input` : Input vector.
//    /// * _Return_ : Output vector.
//    #[inline]
//    pub fn calc(&mut self, input: &MathVec<IN>) -> &MathVec<OUT> {
//        self.cache_input.copy_from(input);
//
//        for i in 0..OUT {
//            self.cache_output[i] = self.neurons[i].calc(input);
//        }
//
//        &self.cache_output
//    }
//
//    /// Forgets data of gradients and momenta.
//    #[inline]
//    pub fn clear_study_data(&mut self) {
//        self.neurons.iter_mut().for_each(
//            |neuron| {neuron.clear_study_data()}
//        );
//    }
//
//    /// Studies gradients.
//    ///
//    /// It only studies gradients. It doesn't update weights yet.  
//    /// __Note: Must call [calc](Layer::calc) before calling this function.__
//    ///
//    /// See [Neuron::study] for details.
//    ///
//    /// * `feedback` : Feedback from next layer. See [Neuron::study] for details.
//    /// * _Return_ : Feedback to previous layer. See [Neuron::study] for details.
//    #[inline]
//    pub fn study(&mut self, feedback: &MathVec<OUT>) -> &MathVec<IN> {
//        self.feedback_next.clear();
//
//        for i in 0..OUT {
//            let feedback_next_2 =
//                self.neurons[i].study(feedback[i]);
//
//            self.feedback_next += feedback_next_2;
//        }
//
//        &self.feedback_next
//    }
//
//    /// Updates Weights to use studied gradients.
//    ///
//    /// * `rate` : Learning rate.
//    #[inline]
//    pub fn update(&mut self, rate: f32) {
//        self.neurons.iter_mut().for_each(|neuron| neuron.update(rate));
//    }
//}
//
///// Neural network.
/////
///// * `OUT` : Dimension of output.
///// * `MIDDLE` : Dimension of middle layer.
///// * `IN` : Dimension of input.
//#[derive(Debug, Clone, PartialEq)]
//pub struct ChobitAI<const OUT: usize, const MIDDLE: usize, const IN: usize> {
//    output_layer: Layer<OUT, MIDDLE>,
//    middle_layer: Layer<MIDDLE, IN>,
//
//    feedback_next: MathVec<IN>,
//    feedback_2: MathVec<OUT>,
//
//    cache_input: MathVec<IN>,
//    cache_middle_layer_output: MathVec<MIDDLE>,
//    cache_output: MathVec<OUT>
//}
//
//impl<
//    const OUT: usize,
//    const MIDDLE: usize,
//    const IN: usize
//> ChobitAI<OUT, MIDDLE, IN> {
//    /// Creates ChobitAI.
//    ///
//    /// * `activation` : Activation function.
//    /// * _Return_ : ChobitAI.
//    #[inline]
//    pub fn new(activation: Activation) -> Self {
//        Self {
//            output_layer: Layer::<OUT, MIDDLE>::new(activation),
//            middle_layer: Layer::<MIDDLE, IN>::new(Activation::ReLU),
//
//            feedback_next: MathVec::<IN>::new(),
//            feedback_2: MathVec::<OUT>::new(),
//
//            cache_input: MathVec::<IN>::new(),
//            cache_middle_layer_output: MathVec::<MIDDLE>::new(),
//            cache_output: MathVec::<OUT>::new()
//        }
//    }
//
//    /// Gets immutable output layer
//    ///
//    /// * _Return_ : Output layer.
//    #[inline]
//    pub fn output_layer(&self) -> &Layer<OUT, MIDDLE> {&self.output_layer}
//
//    /// Gets mutable output layer
//    ///
//    /// * _Return_ : Output layer.
//    #[inline]
//    pub fn output_layer_mut(&mut self) -> &mut Layer<OUT, MIDDLE> {
//        &mut self.output_layer
//    }
//
//    /// Gets immutable middle layer
//    ///
//    /// * _Return_ : Middle layer.
//    #[inline]
//    pub fn middle_layer(&self) -> &Layer<MIDDLE, IN> {&self.middle_layer}
//
//    /// Gets mutable middle layer
//    ///
//    /// * _Return_ : Middle layer.
//    #[inline]
//    pub fn middle_layer_mut(&mut self) -> &mut Layer<MIDDLE, IN> {
//        &mut self.middle_layer
//    }
//
//    /// Gets cache of input.
//    ///
//    /// * _Return_ : Cache of input.
//    #[inline]
//    pub fn cache_input(&self) -> &MathVec<IN> {&self.cache_input}
//
//    /// Gets cache of output of middle layer.
//    ///
//    /// * _Return_ : Cache of output of middle layer.
//    #[inline]
//    pub fn cache_middle_layer_output(&self) -> &MathVec<MIDDLE> {
//        &self.cache_middle_layer_output
//    }
//
//    /// Gets cache of output.
//    ///
//    /// * _Return_ : Cache of output.
//    #[inline]
//    pub fn cache_output(&self) -> &MathVec<OUT> {&self.cache_output}
//
//    /// Calcurates input.
//    ///
//    /// * `input` : Input vector.
//    /// * _Return_ : Output vector.
//    #[inline]
//    pub fn calc(&mut self, input: &MathVec<IN>) -> &MathVec<OUT> {
//        self.cache_input.copy_from(input);
//
//        self.cache_middle_layer_output.copy_from(
//            &self.middle_layer.calc(input)
//        );
//
//        self.cache_output.copy_from(
//            self.output_layer.calc(&self.cache_middle_layer_output)
//        );
//
//        &self.cache_output
//    }
//
//    /// Forgets data of gradients and momenta.
//    #[inline]
//    pub fn clear_study_data(&mut self) {
//        self.output_layer.clear_study_data();
//        self.middle_layer.clear_study_data();
//    }
//
//    /// Studies Gradients by feedback.
//    ///
//    /// It only studies gradients. It doesn't update weights yet.  
//    /// __Note: Must call [calc](ChobitAI::calc) before calling this function.__  
//    /// See [Neuron::study] for details.
//    ///
//    /// * `feedback` : Feedback from next AI.
//    /// * _Return_ : Feedback for previous AI.
//    pub fn study_by_feedback(
//        &mut self,
//        feedback: &MathVec<OUT>
//    ) -> &MathVec<IN> {
//        self.feedback_next.copy_from(
//            self.middle_layer.study(
//                self.output_layer.study(feedback)
//            )
//        );
//
//        &self.feedback_next
//    }
//
//    /// Studies Gradients from train data.
//    ///
//    /// It only studies gradients. It doesn't update weights yet.  
//    /// See [Neuron::study] for details.
//    ///
//    /// * `train_out` : Output of training data.
//    /// * `train_in` : Input of training data.
//    /// * _Return_ : Feedback for previous AI.
//    pub fn study(
//        &mut self,
//        train_out: &MathVec<OUT>,
//        train_in: &MathVec<IN>
//    ) -> &MathVec<IN> {
//        let _ = self.calc(train_in);
//
//        self.feedback_2.copy_from(&self.cache_output);
//
//        self.feedback_2 -= train_out;
//
//        self.feedback_next.copy_from(
//            self.middle_layer.study(self.output_layer.study(&self.feedback_2))
//        );
//
//        &self.feedback_next
//    }
//
//    /// Updates Weights to use studied gradients.
//    ///
//    /// * `rate` : Learning rate.
//    #[inline]
//    pub fn update(&mut self, rate: f32) {
//        self.output_layer.update(rate);
//        self.middle_layer.update(rate);
//    }
//}
//
