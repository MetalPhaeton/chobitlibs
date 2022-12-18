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

#![allow(dead_code)]

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
    mem::size_of
};

#[inline]
fn abs(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7fffffff)
}

#[inline]
fn sqrt(x: f32) -> f32 {
    const MAGIC_32: u32 = 0x5f3759df;

    let a = x * 0.5;
    let y = f32::from_bits(MAGIC_32 - (x.to_bits() >> 1));

    y * (1.5 - (a * y * y))
}

macro_rules! pointwise_op {
    ($self:expr, $other:expr, $ops:tt) => {{
        for i in 0..N {$self.body[i] $ops $other.body[i];}
    }};
}

macro_rules! scalar_op {
    ($self:expr, $other:expr, $ops:tt) => {{
        for i in 0..N {$self.body[i] $ops $other;}
    }};
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MathVec<const N: usize> {
    body: Box<[f32]>
}

impl<const N: usize> MathVec<N> {
    #[inline]
    pub fn new() -> Self {
        Self {body: vec![f32::default(); N].into_boxed_slice()}
    }

    #[inline]
    pub fn as_slice(&self) -> &[f32] {&*self.body}

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [f32] {&mut *self.body}

    #[inline]
    pub fn clear(&mut self) {self.body.fill(f32::default());}

    #[inline]
    pub fn pointwise_mul(&self, other: &Self) -> Self {
        let mut ret = self.clone();

        pointwise_op!(ret, other, *=);

        ret
    }

    #[inline]
    pub fn pointwise_mul_assign(&mut self, other: &Self) {
        pointwise_op!(self, other, *=);
    }

    #[inline]
    pub fn pointwise_div(&self, other: &Self) -> Self {
        let mut ret = self.clone();

        pointwise_op!(ret, other, /=);

        ret
    }

    #[inline]
    pub fn pointwise_div_assign(&mut self, other: &Self) {
        pointwise_op!(self, other, /=);
    }

    #[inline]
    pub fn pointwise_rem(&self, other: &Self) -> Self {
        let mut ret = self.clone();

        pointwise_op!(ret, other, %=);

        ret
    }

    #[inline]
    pub fn pointwise_rem_assign(&mut self, other: &Self) {
        pointwise_op!(self, other, %=);
    }

    #[inline]
    pub fn copy_from(&mut self, other: &Self) {
        self.body.copy_from_slice(&*other.body);
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
            ret += self.body[i] * other.body[i];
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

macro_rules! from_label_body {
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
    #[inline]
    pub fn to_u8_label(&self) -> u8 {
        to_label_body!(self, u8)
    }

    #[inline]
    pub fn from_u8_label(&mut self, mut label: u8) {
        from_label_body!(self, u8, label)
    }
}

impl MathVec<16> {
    #[inline]
    pub fn to_u16_label(&self) -> u16 {
        to_label_body!(self, u16)
    }

    #[inline]
    pub fn from_u16_label(&mut self, mut label: u16) {
        from_label_body!(self, u16, label)
    }
}

impl MathVec<32> {
    #[inline]
    pub fn to_u32_label(&self) -> u32 {
        to_label_body!(self, u32)
    }

    #[inline]
    pub fn from_u32_label(&mut self, mut label: u32) {
        from_label_body!(self, u32, label)
    }
}

impl MathVec<64> {
    #[inline]
    pub fn to_u64_label(&self) -> u64 {
        to_label_body!(self, u64)
    }

    #[inline]
    pub fn from_u64_label(&mut self, mut label: u64) {
        from_label_body!(self, u64, label)
    }
}

impl MathVec<128> {
    #[inline]
    pub fn to_u128_label(&self) -> u128 {
        to_label_body!(self, u128)
    }

    #[inline]
    pub fn from_u128_label(&mut self, mut label: u128) {
        from_label_body!(self, u128, label)
    }
}

/// Weights of a linear function.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Weights<const N: usize> {
    w: MathVec<N>,
    b: f32
}

impl<const N: usize> Weights<N> {
    #[inline]
    pub fn new(w: MathVec<N>, b: f32) -> Self {
        Self {w: w, b: b}
    }

    #[inline]
    pub fn w(&self) -> &MathVec<N> {&self.w}

    #[inline]
    pub fn b(&self) -> f32 {self.b}

    #[inline]
    pub fn w_mut(&mut self) -> &mut MathVec<N> {&mut self.w}

    #[inline]
    pub fn b_mut(&mut self) -> &mut f32 {&mut self.b}

    #[inline]
    pub fn clear(&mut self) {
        self.w.clear();
        self.b = f32::default();
    }

    #[inline]
    pub fn pointwise_mul(&self, other: &Self) -> Self {
        Weights::<N> {
            w: self.w.pointwise_mul(&other.w),
            b: self.b * other.b
        }
    }

    #[inline]
    pub fn pointwise_mul_assign(&mut self, other: &Self) {
        self.w.pointwise_mul_assign(&other.w);
        self.b *= other.b;
    }

    #[inline]
    pub fn pointwise_div(&self, other: &Self) -> Self {
        Weights::<N> {
            w: self.w.pointwise_div(&other.w),
            b: self.b / other.b
        }
    }

    #[inline]
    pub fn pointwise_div_assign(&mut self, other: &Self) {
        self.w.pointwise_div_assign(&other.w);
        self.b /= other.b;
    }

    #[inline]
    pub fn pointwise_rem(&self, other: &Self) -> Self {
        Weights::<N> {
            w: self.w.pointwise_rem(&other.w),
            b: self.b % other.b
        }
    }

    #[inline]
    pub fn pointwise_rem_assign(&mut self, other: &Self) {
        self.w.pointwise_rem_assign(&other.w);
        self.b %= other.b;
    }

    #[inline]
    pub fn copy_from(&mut self, other: &Self) {
        self.w.copy_from(&other.w);
        self.b = other.b;
    }
}

impl<const N: usize> Default for Weights<N> {
    #[inline]
    fn default() -> Self {
        Self {
            w: MathVec::default(),
            b: f32::default()
        }
    }
}

impl<const N: usize> Add<&Weights<N>> for &Weights<N> {
    type Output = Weights<N>;

    #[inline]
    fn add(self, other: &Weights<N>) -> Weights<N> {
        Weights::<N> {
            w: &self.w + &other.w,
            b: self.b + other.b
        }
    }
}

impl<const N: usize> AddAssign<&Weights<N>> for Weights<N> {
    #[inline]
    fn add_assign(&mut self, other: &Weights<N>) {
        self.w += &other.w;
        self.b += other.b;
    }
}

impl<const N: usize> Sub<&Weights<N>> for &Weights<N> {
    type Output = Weights<N>;

    #[inline]
    fn sub(self, other: &Weights<N>) -> Weights<N> {
        Weights::<N> {
            w: &self.w - &other.w,
            b: self.b - other.b
        }
    }
}

impl<const N: usize> SubAssign<&Weights<N>> for Weights<N> {
    #[inline]
    fn sub_assign(&mut self, other: &Weights<N>) {
        self.w -= &other.w;
        self.b -= other.b;
    }
}

impl<const N: usize> Mul<f32> for &Weights<N> {
    type Output = Weights<N>;

    #[inline]
    fn mul(self, other: f32) -> Weights<N> {
        Weights::<N> {
            w: &self.w * other,
            b: self.b * other
        }
    }
}

impl<const N: usize> MulAssign<f32> for Weights<N> {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.w *= other;
        self.b *= other;
    }
}

impl<const N: usize> Div<f32> for &Weights<N> {
    type Output = Weights<N>;

    #[inline]
    fn div(self, other: f32) -> Weights<N> {
        Weights::<N> {
            w: &self.w / other,
            b: self.b / other
        }
    }
}

impl<const N: usize> DivAssign<f32> for Weights<N> {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.w /= other;
        self.b /= other;
    }
}

impl<const N: usize> Rem<f32> for &Weights<N> {
    type Output = Weights<N>;

    #[inline]
    fn rem(self, other: f32) -> Weights<N> {
        Weights::<N> {
            w: &self.w % other,
            b: self.b % other
        }
    }
}

impl<const N: usize> RemAssign<f32> for Weights<N> {
    #[inline]
    fn rem_assign(&mut self, other: f32) {
        self.w %= other;
        self.b %= other;
    }
}

impl<const N: usize> Mul<&Weights<N>> for &Weights<N> {
    type Output = f32;

    #[inline]
    fn mul(self, other: &Weights<N>) -> f32 {
        (&self.w * &other.w) + (self.b * other.b)
    }
}

impl<const N: usize> Mul<&MathVec<N>> for &Weights<N> {
    type Output = f32;

    #[inline]
    fn mul(self, other: &MathVec<N>) -> f32 {
        (&self.w * other) + self.b
    }
}

impl<const N: usize> Mul<&Weights<N>> for &MathVec<N> {
    type Output = f32;

    #[inline]
    fn mul(self, other: &Weights<N>) -> f32 {
        other * self
    }
}

/// Activation function for Neuron.
///
/// See [Neuron] for details.
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

/// Neuron that is a part of AI.
#[derive(Debug, Clone, PartialEq)]
pub struct Neuron<const N: usize> {
    weights: Weights<N>,
    activation: Activation,

    total_grad: Weights<N>,
    study_count: f32,
    mome_1: Weights<N>,
    mome_2: f32,

    grad_w: Weights<N>,
    grad_x: MathVec<N>,

    total_grad_2: Weights<N>,
    mome_1_hat: Weights<N>
}

impl<const N: usize> Neuron<N> {
    /// Creates Neuron.
    ///
    /// * `weights` : Initial weights.
    /// * `activation` : Activation function.
    /// * _Return_ : Instance.
    #[inline]
    pub fn new(activation: Activation) -> Self {
        Self {
            weights: Weights::<N>::default(),
            activation: activation,

            total_grad: Weights::<N>::default(),
            study_count: 0.0,
            mome_1: Weights::<N>::default(),
            mome_2: 0.0,

            grad_w: Weights::<N>::default(),
            grad_x: MathVec::<N>::default(),

            total_grad_2: Weights::<N>::default(),
            mome_1_hat: Weights::<N>::default()
        }
    }

    /// Gets Weights.
    ///
    /// * _Return_ : Weights.
    #[inline]
    pub fn weights(&self) -> &Weights<N> {&self.weights}

    #[inline]
    pub fn weights_mut(&mut self) -> &mut Weights<N> {&mut self.weights}

    /// Gets activation function.
    ///
    /// * _Return_ : Activation function.
    #[inline]
    pub fn activation(&self) -> Activation {self.activation}

    #[inline]
    pub fn activation_mut (&mut self) -> &mut Activation {&mut self.activation}

    /// Gets total gradients.
    ///
    /// * _Return_ : Total gradients.
    #[inline]
    pub fn total_grad(&self) -> &Weights<N> {&self.total_grad}

    #[inline]
    pub fn total_grad_mut(&mut self) -> &mut Weights<N> {&mut self.total_grad}

    /// Gets a count how many times it have been studying.
    ///
    /// * _Return_ : A count.
    #[inline]
    pub fn study_count(&self) -> f32 {self.study_count}

    #[inline]
    pub fn set_study_count(&mut self, count: f32) {
        self.study_count = count.max(0.0) as usize as f32;
    }

    /// Gets 1st momentum for Adam.
    ///
    /// * _Return_ : 1st momentum.
    #[inline]
    pub fn mome_1(&self) -> &Weights<N> {&self.mome_1}

    #[inline]
    pub fn mome_1_mut(&mut self) -> &mut Weights<N> {&mut self.mome_1}

    /// Gets 2nd momentum for Adam.
    ///
    /// * _Return_ : 2nd momentum.
    #[inline]
    pub fn mome_2(&self) -> f32 {self.mome_2}

    #[inline]
    pub fn set_mome_2(&mut self, mome_2: f32) {
        self.mome_2 = abs(mome_2);
    }

    /// Calculates input by linear function and activation function.
    ///
    /// * `input` : Input vector.
    /// * _Return_ : Output number.
    #[inline]
    pub fn calc(&self, input: &MathVec<N>) -> f32 {
        self.activation.activate(&self.weights * input)
    }

    /// Forgets data of gradients and momenta.
    #[inline]
    pub fn clear_study_data(&mut self) {
        self.total_grad.clear();
        self.study_count = 0.0;
        self.mome_1.clear();
        self.mome_2 = 0.0;
    }

    /// Studies gradients by Backpropagation.
    /// But not updates weights yet.
    ///
    /// * `feedback` : Gradient of loss function that is propagated back from next layer.
    /// * `input` : Input vector.
    /// * _Return_ : Gradient of loss function that should propagate to previous layer.
    ///
    /// ```text
    ///               feedback                      Return
    /// Next_Neuron ------------> This_Neuron    ------------> Previous_Neuron
    /// Next_Neuron ---|     |--> Sibling_Neuron ---|     |--> Previous_Neuron
    /// Next_Neuron ---|     |--> Sibling_Neuron ---|     |--> Previous_Neuron
    /// Next_Neuron ---/     \--> Sibling_Neuron ---/     \--> Previous_Neuron
    /// ```
    #[inline]
    pub fn study(&mut self, feedback: f32, input: &MathVec<N>) -> &MathVec<N> {
        self.calc_grad(feedback, input);

        self.total_grad += &self.grad_w;
        self.study_count += 1.0;

        &self.grad_x
    }

    fn calc_grad(&mut self, feedback: f32, input: &MathVec<N>) {
        let factor =
            feedback * self.activation.d_activate(&self.weights * input);

        self.grad_w.w_mut().copy_from(input);
        *self.grad_w.w_mut() *= factor;
        *self.grad_w.b_mut() = factor;

        self.grad_x.copy_from(self.weights.w());
        self.grad_x *= factor;
    }

    /// Updates Weights to use studied gradients.
    ///
    /// * `rate` : Learning rate.
    pub fn update(&mut self, rate: f32) {
        self.total_grad /= self.study_count;

        self.next_mome_1();
        self.next_mome_2();

        self.calc_mome_1_hat();
        let mome_2 = self.mome_2_hat();

        self.update_weights(mome_2, rate);

        self.total_grad.clear();
        self.study_count = 0.0;
    }

    #[inline]
    fn next_mome_1(&mut self) {
        const BETA: f32 = 0.9;
        const BETA_INV: f32 = 1.0 - BETA;

        self.total_grad_2.copy_from(&self.mome_1);

        self.mome_1 *= BETA;

        self.total_grad_2.copy_from(&self.total_grad);
        self.total_grad_2 *= BETA_INV;

        self.mome_1 += &self.total_grad_2;
    }

    #[inline]
    fn next_mome_2(&mut self) {
        const BETA: f32 = 0.999;
        const BETA_INV: f32 = 1.0 - BETA;

        self.mome_2 *= BETA;
        self.mome_2 += (&self.total_grad * &self.total_grad) * BETA_INV;
    }

    #[inline]
    fn calc_mome_1_hat(&mut self) {
        const BETA: f32 = 0.9;
        const BETA_INV: f32 = 1.0 - BETA;

        self.mome_1_hat.copy_from(&self.mome_1);
        self.mome_1_hat /= BETA_INV;
    }

    #[inline]
    fn mome_2_hat(&self) -> f32 {
        const BETA: f32 = 0.999;
        const BETA_INV: f32 = 1.0 - BETA;

        self.mome_2 / BETA_INV
    }

    #[inline]
    fn update_weights(&mut self, mome_2: f32, rate: f32) {
        self.mome_1_hat /= sqrt(mome_2) + f32::EPSILON;
        self.mome_1_hat *= rate;

        self.weights -= &self.mome_1_hat;
    }
}

/// Layer of AI.
///
/// * `OUT` : Dimension of output. It equals a number of Neurons.
/// * `IN` : Dimension of input. It equals a number of weights per one Neuron.
#[derive(Debug, Clone, PartialEq)]
pub struct Layer<const OUT: usize, const IN: usize> {
    neurons: Box<[Neuron<IN>]>,

    feedback_next: MathVec<IN>,
}

impl<const OUT: usize, const IN: usize> Layer<OUT, IN> {
    /// Creates Layer.
    ///
    #[inline]
    pub fn new(acitvation: Activation) -> Self {
        Self {
            neurons:
                vec![Neuron::<IN>::new(acitvation); OUT].into_boxed_slice(),

            feedback_next: MathVec::<IN>::default()
        }
    }

    /// Gets neurons.
    ///
    /// _Return_ : neurons.
    #[inline]
    pub fn neurons(&self) -> &[Neuron<IN>] {&*self.neurons}

    #[inline]
    pub fn neurons_mut(&mut self) -> &mut [Neuron<IN>] {&mut *self.neurons}

    /// Calculates input.
    ///
    /// * `input` : Input vector.
    /// * `output` : Output vector.
    #[inline]
    pub fn calc(&self, input: &MathVec<IN>, output: &mut MathVec<OUT>) {
        for i in 0..OUT {
            output[i] = self.neurons[i].calc(input);
        }
    }

    /// Studies gradients.
    ///
    /// See [Neuron::study] for details.
    ///
    /// * `feedback` : Feedback from next layer. See [Neuron::study] for details.
    /// * `input` : Input vector
    /// * _Return_ : Feedback to previous layer. See [Neuron::study] for details.
    #[inline]
    pub fn study(
        &mut self,
        feedback: &MathVec<OUT>,
        input: &MathVec<IN>,
    ) -> &MathVec<IN> {
        self.feedback_next.clear();

        for i in 0..OUT {
            let feedback_next_2 = self.neurons[i].study(feedback[i], input);

            self.feedback_next += feedback_next_2;
        }

        &self.feedback_next
    }

    /// Updates Weights to use studied gradients.
    ///
    /// * `rate` : Learning rate.
    #[inline]
    pub fn update(&mut self, rate: f32) {
        self.neurons.iter_mut().for_each(|neuron| neuron.update(rate));
    }
}

/// Neural network.
///
/// * `OUT` : Dimension of output.
/// * `MIDDLE` : Dimension of middle layer.
/// * `IN` : Dimension of input.
#[derive(Debug, Clone, PartialEq)]
pub struct ChobitAI<const OUT: usize, const MIDDLE: usize, const IN: usize> {
    output_layer: Layer<OUT, MIDDLE>,
    middle_layer: Layer<MIDDLE, IN>,

    tmp_out: MathVec<OUT>,
    tmp_middle: MathVec<MIDDLE>,
    feedback_next: MathVec<IN>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> ChobitAI<OUT, MIDDLE, IN> {
    /// Creates ChobitAI.
    #[inline]
    pub fn new(activation: Activation) -> Self {
        Self {
            output_layer: Layer::<OUT, MIDDLE>::new(activation),
            middle_layer: Layer::<MIDDLE, IN>::new(Activation::ReLU),

            tmp_out: MathVec::<OUT>::new(),
            tmp_middle: MathVec::<MIDDLE>::new(),
            feedback_next: MathVec::<IN>::new()
        }
    }

    /// Gets Output Layer
    ///
    /// * _Return_ : Output layer.
    #[inline]
    pub fn output_layer(&self) -> &Layer<OUT, MIDDLE> {&self.output_layer}

    #[inline]
    pub fn output_layer_mut(&mut self) -> &mut Layer<OUT, MIDDLE> {
        &mut self.output_layer
    }

    /// Gets Middle Layer
    ///
    /// * _Return_ : Middle layer.
    #[inline]
    pub fn middle_layer(&self) -> &Layer<MIDDLE, IN> {&self.middle_layer}

    #[inline]
    pub fn middle_layer_mut(&mut self) -> &mut Layer<MIDDLE, IN> {
        &mut self.middle_layer
    }

    /// Calcurates input.
    ///
    /// * `input` : Input vector;
    /// * `output` : Output vector;
    #[inline]
    pub fn calc(&mut self, input: &MathVec<IN>, output: &mut MathVec<OUT>) {
        self.middle_layer.calc(input, &mut self.tmp_middle);
        self.output_layer.calc(&self.tmp_middle, output);
    }

    /// Studies Gradients with training data.
    ///
    /// It only studies gradients. It doesn't update weights yet.
    ///
    /// * `train_out` : Output of training data.
    /// * `train_in` : Input of training data.
    pub fn study_by_feedback(
        &mut self,
        feedback: &MathVec<OUT>,
        train_in: &MathVec<IN>
    ) -> &MathVec<IN> {
        self.middle_layer.calc(train_in, &mut self.tmp_middle);

        self.feedback_next.copy_from(
            self.middle_layer.study(
                self.output_layer.study(
                    feedback,
                    &self.tmp_middle
                ),
                train_in
            )
        );

        &self.feedback_next
    }

    pub fn study(
        &mut self,
        train_out: &MathVec<OUT>,
        train_in: &MathVec<IN>
    ) -> &MathVec<IN> {
        self.middle_layer.calc(train_in, &mut self.tmp_middle);
        self.output_layer.calc(&self.tmp_middle, &mut self.tmp_out);

        self.tmp_out -= train_out;

        self.feedback_next.copy_from(
            self.middle_layer.study(
                self.output_layer.study(
                    &self.tmp_out,
                    &self.tmp_middle
                ),
                train_in
            )
        );

        &self.feedback_next
    }

    /// Updates Weights to use studied gradients.
    ///
    /// * `rate` : Learning rate.
    #[inline]
    pub fn update(&mut self, rate: f32) {
        self.output_layer.update(rate);
        self.middle_layer.update(rate);
    }
}
