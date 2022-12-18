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

    tmp_weights_1: Weights<N>,
    tmp_weights_2: Weights<N>,
    tmp_vec: MathVec<N>
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

            tmp_weights_1: Weights::<N>::default(),
            tmp_weights_2: Weights::<N>::default(),
            tmp_vec: MathVec::<N>::default()
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

        self.total_grad += &self.tmp_weights_1;
        self.study_count += 1.0;

        &self.tmp_vec
    }

    fn calc_grad(&mut self, feedback: f32, input: &MathVec<N>) {
        let factor =
            feedback * self.activation.d_activate(&self.weights * input);

        self.tmp_weights_1.w_mut().copy_from(input);
        *self.tmp_weights_1.w_mut() *= factor;
        *self.tmp_weights_1.b_mut() = factor;

        self.tmp_vec.copy_from(self.weights.w());
        self.tmp_vec *= factor;
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

        self.tmp_weights_1.copy_from(&self.mome_1);

        self.mome_1 *= BETA;

        self.tmp_weights_1.copy_from(&self.total_grad);
        self.tmp_weights_1 *= BETA_INV;

        self.mome_1 += &self.tmp_weights_1;
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

        self.tmp_weights_2.copy_from(&self.mome_1);
        self.tmp_weights_2 /= BETA_INV;
    }

    #[inline]
    fn mome_2_hat(&self) -> f32 {
        const BETA: f32 = 0.999;
        const BETA_INV: f32 = 1.0 - BETA;

        self.mome_2 / BETA_INV
    }

    #[inline]
    fn update_weights(&mut self, mome_2: f32, rate: f32) {
        self.tmp_weights_2 /= sqrt(mome_2) + f32::EPSILON;
        self.tmp_weights_2 *= rate;

        self.weights -= &self.tmp_weights_2;
    }
}

/// Layer of AI.
///
/// * `OUT` : Dimension of output. It equals a number of Neurons.
/// * `IN` : Dimension of input. It equals a number of weights per one Neuron.
#[derive(Debug, Clone, PartialEq)]
pub struct Layer<const OUT: usize, const IN: usize> {
    neurons: Box<[Neuron<IN>]>,

    tmp_in: MathVec<IN>,
}

impl<const OUT: usize, const IN: usize> Layer<OUT, IN> {
    /// Creates Layer.
    ///
    #[inline]
    pub fn new(acitvation: Activation) -> Self {
        Self {
            neurons:
                vec![Neuron::<IN>::new(acitvation); OUT].into_boxed_slice(),

            tmp_in: MathVec::<IN>::default()
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
        self.tmp_in.clear();

        for i in 0..OUT {
            let feedback_next = self.neurons[i].study(feedback[i], input);

            self.tmp_in += feedback_next;
        }

        &self.tmp_in
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
    tmp_in: MathVec<IN>
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
            tmp_in: MathVec::<IN>::new()
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

        self.tmp_in.copy_from(
            self.middle_layer.study(
                self.output_layer.study(
                    feedback,
                    &self.tmp_middle
                ),
                train_in
            )
        );

        &self.tmp_in
    }

    pub fn study(
        &mut self,
        train_out: &MathVec<OUT>,
        train_in: &MathVec<IN>
    ) -> &MathVec<IN> {
        self.middle_layer.calc(train_in, &mut self.tmp_middle);
        self.output_layer.calc(&self.tmp_middle, &mut self.tmp_out);

        self.tmp_out -= train_out;

        self.tmp_in.copy_from(
            self.middle_layer.study(
                self.output_layer.study(
                    &self.tmp_out,
                    &self.tmp_middle
                ),
                train_in
            )
        );

        &self.tmp_in
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

//#[derive(Debug, Clone, PartialEq)]
//pub struct GRUNeuron<const N: usize> {
//    z_weights: Box<(Weights<N>, f32)>,
//    r_weights: Box<(Weights<N>, f32)>,
//    h_weights: Box<(Weights<N>, f32)>,
//
//    z_total_grad: Box<(Weights<N>, f32)>,
//    r_total_grad: Box<(Weights<N>, f32)>,
//    h_total_grad: Box<(Weights<N>, f32)>,
//
//    study_count: f32,
//
//    z_mome_1: Box<(Weights<N>, f32)>,
//    r_mome_1: Box<(Weights<N>, f32)>,
//    h_mome_1: Box<(Weights<N>, f32)>,
//
//    z_mome_2: f32,
//    r_mome_2: f32,
//    h_mome_2: f32
//}
//
//impl<const N: usize> GRUNeuron<N> {
//    #[inline]
//    pub fn new(
//        z_weights: (Weights<N>, f32),
//        r_weights: (Weights<N>, f32),
//        h_weights: (Weights<N>, f32)
//    ) -> Self {
//        Self {
//            z_weights: Box::new(z_weights),
//            r_weights: Box::new(r_weights),
//            h_weights: Box::new(h_weights),
//
//            z_total_grad: Box::new((Weights::<N>::default(), 0.0)),
//            r_total_grad: Box::new((Weights::<N>::default(), 0.0)),
//            h_total_grad: Box::new((Weights::<N>::default(), 0.0)),
//
//            study_count: 0.0,
//
//            z_mome_1: Box::new((Weights::<N>::default(), 0.0)),
//            r_mome_1: Box::new((Weights::<N>::default(), 0.0)),
//            h_mome_1: Box::new((Weights::<N>::default(), 0.0)),
//
//            z_mome_2: 0.0,
//            r_mome_2: 0.0,
//            h_mome_2: 0.0
//        }
//    }
//
//    #[inline]
//    pub fn with_study_data(
//        z_weights: (Weights<N>, f32),
//        r_weights: (Weights<N>, f32),
//        h_weights: (Weights<N>, f32),
//        z_total_grad: (Weights<N>, f32),
//        r_total_grad: (Weights<N>, f32),
//        h_total_grad: (Weights<N>, f32),
//        study_count: f32,
//        z_mome_1: (Weights<N>, f32),
//        r_mome_1: (Weights<N>, f32),
//        h_mome_1: (Weights<N>, f32),
//        z_mome_2: f32,
//        r_mome_2: f32,
//        h_mome_2: f32
//    ) -> Self {
//        Self {
//            z_weights: Box::new(z_weights),
//            r_weights: Box::new(r_weights),
//            h_weights: Box::new(h_weights),
//            z_total_grad: Box::new(z_total_grad),
//            r_total_grad: Box::new(r_total_grad),
//            h_total_grad: Box::new(h_total_grad),
//            study_count: study_count,
//            z_mome_1: Box::new(z_mome_1),
//            r_mome_1: Box::new(r_mome_1),
//            h_mome_1: Box::new(h_mome_1),
//            z_mome_2: z_mome_2,
//            r_mome_2: r_mome_2,
//            h_mome_2: h_mome_2
//        }
//    }
//
//    #[inline]
//    pub fn z_weights(&self) -> &(Weights<N>, f32) {&*self.z_weights}
//
//    #[inline]
//    pub fn r_weights(&self) -> &(Weights<N>, f32) {&*self.r_weights}
//
//    #[inline]
//    pub fn h_weights(&self) -> &(Weights<N>, f32) {&*self.h_weights}
//
//    #[inline]
//    pub fn z_total_grad(&self) -> &(Weights<N>, f32) {&*self.z_total_grad}
//
//    #[inline]
//    pub fn r_total_grad(&self) -> &(Weights<N>, f32) {&*self.r_total_grad}
//
//    #[inline]
//    pub fn h_total_grad(&self) -> &(Weights<N>, f32) {&*self.h_total_grad}
//
//    #[inline]
//    pub fn study_count(&self) -> f32 {self.study_count}
//
//    #[inline]
//    pub fn z_mome_1(&self) -> &(Weights<N>, f32) {&*self.z_mome_1}
//
//    #[inline]
//    pub fn r_mome_1(&self) -> &(Weights<N>, f32) {&*self.r_mome_1}
//
//    #[inline]
//    pub fn h_mome_1(&self) -> &(Weights<N>, f32) {&*self.h_mome_1}
//
//    #[inline]
//    pub fn z_mome_2(&self) -> f32 {self.z_mome_2}
//
//    #[inline]
//    pub fn r_mome_2(&self) -> f32 {self.r_mome_2}
//
//    #[inline]
//    pub fn h_mome_2(&self) -> f32 {self.h_mome_2}
//
//    #[inline]
//    pub fn calc(&self, input: &[f32; N], state: f32) -> f32 {
//        let z_output = Activation::Sigmoid.activate(
//            self.calc_z(input, state)
//        );
//
//        let r_output = Activation::Sigmoid.activate(
//            self.calc_r(input, state)
//        );
//
//        let h_output = Activation::SoftSign.activate(
//            self.calc_h(input, state * r_output)
//        );
//
//        let z_inv_output = 1.0 - z_output;
//
//        (h_output * z_output) + (state * z_inv_output)
//    }
//
//    #[inline]
//    fn calc_z(&self, input: &[f32; N], state: f32) -> f32 {
//        (self.z_weights.0 * *input) + (self.z_weights.1 * state)
//    }
//
//    #[inline]
//    fn calc_r(&self, input: &[f32; N], state: f32) -> f32 {
//        (self.r_weights.0 * *input) + (self.r_weights.1 * state)
//    }
//
//    #[inline]
//    fn calc_h(&self, input: &[f32; N], state: f32) -> f32 {
//        (self.h_weights.0 * *input) + (self.h_weights.1 * state)
//    }
//
//    #[inline]
//    pub fn clear_study_data(&mut self) {
//        self.z_total_grad.0.clear();
//        self.r_total_grad.0.clear();
//        self.h_total_grad.0.clear();
//        self.z_total_grad.1 = 0.0;
//        self.r_total_grad.1 = 0.0;
//        self.h_total_grad.1 = 0.0;
//
//        self.study_count = 0.0;
//
//        self.z_mome_1.0.clear();
//        self.r_mome_1.0.clear();
//        self.h_mome_1.0.clear();
//        self.z_mome_1.1 = 0.0;
//        self.r_mome_1.1 = 0.0;
//        self.h_mome_1.1 = 0.0;
//
//        self.z_mome_2 = 0.0;
//        self.r_mome_2 = 0.0;
//        self.h_mome_2 = 0.0;
//    }
//
//    pub fn study(
//        &mut self,
//        feedback: f32,
//        input: &[f32; N],
//        state: f32
//    ) -> ([f32; N], f32) {
//
//        let z = self.calc_z(input, state);
//        let r = self.calc_r(input, state);
//        let h = self.calc_h(
//            input,
//            state * Activation::Sigmoid.activate(r)
//        );
//
//        let z_inv_output = 1.0 - Activation::Sigmoid.activate(z);
//
//        let d_sig_z = Activation::Sigmoid.d_activate(z);
//        let d_sig_r = Activation::Sigmoid.d_activate(r);
//        let d_tanh = Activation::SoftSign.d_activate(h);
//
//        let sig_z = Activation::Sigmoid.activate(z);
//        let sig_r = Activation::Sigmoid.activate(r);
//        let tanh = Activation::SoftSign.activate(h);
//
//        assert!(!self.z_weights.1.is_nan());
//        assert!(!self.r_weights.1.is_nan());
//        assert!(!self.h_weights.1.is_nan());
//
//        self.study_z_weights(
//            feedback,
//            input,
//            state,
//            tanh,
//            d_sig_z
//        );
//
//        self.study_r_weights(
//            feedback,
//            input,
//            state,
//            sig_z,
//            d_tanh,
//            self.h_weights.1,
//            d_sig_r
//        );
//
//        self.study_h_weights(
//            feedback,
//            input,
//            state,
//            sig_z,
//            d_tanh,
//            sig_r
//        );
//
//        let feedback_for_input = self.gen_feedback_for_input(
//            feedback,
//            state,
//            d_sig_z,
//            tanh,
//            sig_z,
//            d_tanh,
//            d_sig_r
//        );
//
//        let feedback_for_state = self.gen_feedback_for_state(
//            feedback,
//            state,
//            z_inv_output,
//            d_sig_z,
//            tanh,
//            sig_z,
//            d_tanh,
//            sig_r,
//            d_sig_r
//        );
//
//        self.study_count += 1.0;
//
//        (feedback_for_input, feedback_for_state)
//    }
//
//    fn study_z_weights(
//        &mut self,
//        feedback: f32,
//        input: &[f32; N],
//        state: f32,
//        tanh: f32,
//        d_sig_z: f32
//    ) {
//        let factor = feedback * (tanh - state) * d_sig_z;
//
//        let mut grad_w = Weights::<N>::new(input, 1.0);
//        grad_w *= factor;
//
//        let grad_h = factor * state;
//
//        self.z_total_grad.0 += grad_w;
//        self.z_total_grad.1 += grad_h;
//    }
//
//    fn study_r_weights(
//        &mut self,
//        feedback: f32,
//        input: &[f32; N],
//        state: f32,
//        sig_z: f32,
//        d_tanh: f32,
//        v_h: f32,
//        d_sig_r: f32
//    ) {
//        let factor = feedback * sig_z * d_tanh * v_h * d_sig_r;
//
//        let mut grad_w = Weights::<N>::new(input, 1.0);
//        grad_w *= factor;
//
//        let grad_h = factor * state;
//
//        self.r_total_grad.0 += grad_w;
//        self.r_total_grad.1 += grad_h;
//    }
//
//    fn study_h_weights(
//        &mut self,
//        feedback: f32,
//        input: &[f32; N],
//        state: f32,
//        sig_z: f32,
//        d_tanh: f32,
//        sig_r: f32
//    ) {
//        let factor = feedback * sig_z * d_tanh;
//
//        let mut grad_w = Weights::<N>::new(input, 1.0);
//        grad_w *= factor;
//
//        let grad_h = factor * state * sig_r;
//
//        self.h_total_grad.0 += grad_w;
//        self.h_total_grad.1 += grad_h;
//    }
//
//    fn gen_feedback_for_input(
//        &self,
//        feedback: f32,
//        state: f32,
//        d_sig_z: f32,
//        tanh: f32,
//        sig_z: f32,
//        d_tanh: f32,
//        d_sig_r: f32
//    ) -> [f32; N] {
//        let mut ret: [f32; N] = [0.0; N];
//
//        let v_h = self.h_weights.1;
//
//        for i in 0..N {
//            let w_z = self.z_weights.0.w[i];
//            let w_r = self.r_weights.0.w[i];
//            let w_h = self.h_weights.0.w[i];
//
//            let grad_3 = d_tanh * (w_h + (v_h * state * d_sig_r * w_r));
//            let grad_2 = (tanh * d_sig_z * w_z) + (sig_z * grad_3);
//            let grad_1 = -state * d_sig_z * w_z;
//            let grad = feedback * (grad_1 + grad_2);
//
//            ret[i] = grad;
//        }
//
//        ret
//    }
//
//    fn gen_feedback_for_state(
//        &self,
//        feedback: f32,
//        state: f32,
//        z_inv_output: f32,
//        d_sig_z: f32,
//        tanh: f32,
//        sig_z: f32,
//        d_tanh: f32,
//        sig_r: f32,
//        d_sig_r: f32
//    ) -> f32 {
//        let v_z = self.z_weights.1;
//        let v_r = self.r_weights.1;
//        let v_h = self.h_weights.1;
//
//        let grad_3 = sig_r + (state * d_sig_r * v_r);
//        let grad_2 = (tanh * d_sig_z * v_z) + (sig_z * d_tanh * v_h * grad_3);
//        let grad_1 = z_inv_output - (state * d_sig_z * v_z);
//
//        feedback * (grad_1 + grad_2)
//    }
//
//    pub fn update(&mut self, rate: f32) {
//        if self.study_count < 0.5 {return;}
//
//        let z_grad_w = self.z_total_grad.0 / self.study_count;
//        let z_grad_h = self.z_total_grad.1 / self.study_count;
//
//        let r_grad_w = self.r_total_grad.0 / self.study_count;
//        let r_grad_h = self.r_total_grad.1 / self.study_count;
//
//        let h_grad_w = self.h_total_grad.0 / self.study_count;
//        let h_grad_h = self.h_total_grad.1 / self.study_count;
//
//        Self::next_mome_1(&mut *self.z_mome_1, &z_grad_w, z_grad_h);
//        Self::next_mome_2(&mut self.z_mome_2, &z_grad_w, z_grad_h);
//
//        Self::next_mome_1(&mut *self.r_mome_1, &r_grad_w, r_grad_h);
//        Self::next_mome_2(&mut self.r_mome_2, &r_grad_w, r_grad_h);
//
//        Self::next_mome_1(&mut *self.h_mome_1, &h_grad_w, h_grad_h);
//        Self::next_mome_2(&mut self.h_mome_2, &h_grad_w, h_grad_h);
//
//        let z_mome_1 = Self::mome_1_hat(&*self.z_mome_1);
//        let z_mome_2 = Self::mome_2_hat(self.z_mome_2);
//
//        let r_mome_1 = Self::mome_1_hat(&*self.r_mome_1);
//        let r_mome_2 = Self::mome_2_hat(self.r_mome_2);
//
//        let h_mome_1 = Self::mome_1_hat(&*self.h_mome_1);
//        let h_mome_2 = Self::mome_2_hat(self.h_mome_2);
//
//        let z_d_weight = Self::d_weights(&z_mome_1, z_mome_2, rate);
//        let r_d_weight = Self::d_weights(&r_mome_1, r_mome_2, rate);
//        let h_d_weight = Self::d_weights(&h_mome_1, h_mome_2, rate);
//
//        self.z_weights.0 -= z_d_weight.0;
//        self.z_weights.1 -= z_d_weight.1;
//
//        self.r_weights.0 -= r_d_weight.0;
//        self.r_weights.1 -= r_d_weight.1;
//
//        self.h_weights.0 -= h_d_weight.0;
//        self.h_weights.1 -= h_d_weight.1;
//
//        self.z_total_grad.0.clear();
//        self.r_total_grad.0.clear();
//        self.h_total_grad.0.clear();
//        self.z_total_grad.1 = 0.0;
//        self.r_total_grad.1 = 0.0;
//        self.h_total_grad.1 = 0.0;
//
//        self.study_count = 0.0;
//    }
//
//    #[inline]
//    fn next_mome_1(
//        mome_1: &mut (Weights<N>, f32),
//        grad_w: &Weights<N>,
//        grad_h: f32
//    ) {
//        const BETA: f32 = 0.9;
//        const BETA_INV: f32 = 1.0 - BETA;
//
//        mome_1.0 *= BETA;
//        mome_1.0 += *grad_w * BETA_INV;
//        mome_1.1 *= BETA;
//        mome_1.1 += grad_h * BETA_INV;
//    }
//
//    #[inline]
//    fn next_mome_2(
//        mome_2: &mut f32,
//        grad_w: &Weights<N>,
//        grad_h: f32
//    ) {
//        const BETA: f32 = 0.999;
//        const BETA_INV: f32 = 1.0 - BETA;
//
//        *mome_2 *= BETA;
//        *mome_2 += ((*grad_w * *grad_w) + (grad_h * grad_h)) * BETA_INV;
//    }
//
//    #[inline]
//    fn mome_1_hat(mome_1: &(Weights<N>, f32)) -> (Weights<N>, f32) {
//        const BETA: f32 = 0.9;
//        const BETA_INV: f32 = 1.0 - BETA;
//
//        (mome_1.0 / BETA_INV, mome_1.1 / BETA_INV)
//    }
//
//    #[inline]
//    fn mome_2_hat(mome_2: f32) -> f32 {
//        const BETA: f32 = 0.999;
//        const BETA_INV: f32 = 1.0 - BETA;
//
//        mome_2 / BETA_INV
//    }
//
//    #[inline]
//    fn d_weights(
//        mome_1: &(Weights<N>, f32),
//        mome_2: f32,
//        rate: f32
//    ) -> (Weights<N>, f32) {
//        const EPSILION: f32 = 1.0e-8;
//
//        let factor = sqrt(mome_2) + EPSILION;
//
//        (
//            (mome_1.0 / factor) * rate,
//            (mome_1.1 / factor) * rate
//        )
//    }
//}
//
//#[derive(Debug, Clone, PartialEq)]
//pub struct GRULayer<const OUT: usize, const IN: usize> {
//    neurons: Box<[GRUNeuron<IN>; OUT]>,
//    mixer: Layer<OUT, OUT>
//}
//
//impl<const OUT: usize, const IN: usize> GRULayer<OUT, IN> {
//    #[inline]
//    pub fn new(
//        neurons: [GRUNeuron<IN>; OUT],
//        mixer_weights: [Weights<OUT>; OUT]
//    ) -> Self {
//        Self {
//            neurons: Box::new(neurons),
//            mixer: Layer::<OUT, OUT>::new(mixer_weights.iter().map(
//                |weights| Neuron::<OUT>::new(*weights, Activation::SoftSign)
//            ).collect::<Vec<Neuron<OUT>>>().try_into().unwrap())
//        }
//    }
//
//    #[inline]
//    pub fn neurons(&self) -> &[GRUNeuron<IN>; OUT] {&*self.neurons}
//
//    #[inline]
//    pub fn mixer(&self) -> &Layer<OUT, OUT> {&self.mixer}
//
//    #[inline]
//    fn calc_gru(&self, input: &[f32; IN], state: &[f32; OUT]) -> [f32; OUT] {
//        let mut ret = [0.0f32; OUT];
//
//        for i in 0..OUT {
//            ret[i] = self.neurons[i].calc(input, state[i]);
//        }
//
//        ret
//    }
//
//    #[inline]
//    pub fn calc(&self, input: &[f32; IN], state: &[f32; OUT]) -> [f32; OUT] {
//        self.mixer.calc(&self.calc_gru(input, state))
//    }
//
//    pub fn study(
//        &mut self,
//        feedback: &[f32; OUT],
//        input: &[f32; IN],
//        state: &[f32; OUT]
//    ) -> ([f32; IN], [f32; OUT]) {
//        let feedback =
//            self.mixer.study(feedback, &self.calc_gru(input, state));
//
//        let mut ret_1 = [0.0f32; IN];
//        let mut ret_2 = [0.0f32; OUT];
//
//        for i in 0..OUT {
//            let feedback_next = self.neurons[i].study(
//                feedback[i],
//                input,
//                state[i]
//            );
//
//            for j in 0..IN {
//                ret_1[j] += feedback_next.0[j];
//            }
//
//            ret_2[i] = feedback_next.1;
//        }
//
//        (ret_1, ret_2)
//    }
//
//    #[inline]
//    pub fn update(&mut self, rate: f32) {
//        self.neurons.iter_mut().for_each(|block| block.update(rate));
//        self.mixer.update(rate);
//    }
//}
//
//#[derive(Debug, Clone, PartialEq)]
//pub struct ChobitEncoder<const OUT: usize, const IN: usize> {
//    layer: GRULayer<OUT, IN>
//}
//
//impl<const OUT: usize, const IN: usize> ChobitEncoder<OUT, IN> {
//    #[inline]
//    pub fn new(layer: GRULayer<OUT, IN>) -> Self {
//        Self {
//            layer: layer
//        }
//    }
//
//    #[inline]
//    pub fn layer(&self) -> &GRULayer<OUT, IN> {&self.layer}
//
//    #[inline]
//    pub fn calc<'a>(
//        &self,
//        inputs: &mut impl Iterator<Item = &'a [f32; IN]>,
//        initial_state: &[f32; OUT]
//    ) -> [f32; OUT]{
//        let mut state = *initial_state;
//
//        inputs.for_each(|input| {
//            state = self.layer.calc(input, &state);
//        });
//
//        state
//    }
//
//    pub fn study_from_decorder<'a>(
//        &mut self,
//        feedback: &[f32; OUT],
//        train_in: &mut impl Iterator<Item = &'a [f32; IN]>,
//        initial_state: &[f32; OUT]
//    ) -> Option<[f32; OUT]> {
//        let input = train_in.next()?;
//
//        let state = self.layer.calc(input, initial_state);
//
//        match self.study_from_decorder(feedback, train_in, &state) {
//            Some(feedback) => {
//                let (_, feedback_next) =
//                    self.layer.study(&feedback, input, initial_state);
//
//                Some(feedback_next)
//            },
//
//            // Last input
//            None => {
//                let (_, feedback_next) =
//                    self.layer.study(feedback, input, initial_state);
//
//                Some(feedback_next)
//            }
//        }
//    }
//
//    pub fn study<'a>(
//        &mut self,
//        train_out: &[f32; OUT],
//        train_in: &mut impl Iterator<Item = &'a [f32; IN]>,
//        initial_state: &[f32; OUT]
//    ) -> Option<[f32; OUT]> {
//        let input = train_in.next()?;
//
//        let state = self.layer.calc(input, initial_state);
//
//        match self.study(train_out, train_in, &state) {
//            Some(feedback) => {
//                let (_, feedback_next) =
//                    self.layer.study(&feedback, input, initial_state);
//
//                Some(feedback_next)
//            },
//
//            // Last input
//            None => {
//                let mut loss = [0.0f32; OUT];
//
//                for i in 0..OUT {
//                    loss[i] = state[i] - train_out[i];
//                }
//
//                let (_, feedback_next) =
//                    self.layer.study(&loss, input, initial_state);
//
//                Some(feedback_next)
//            }
//        }
//    }
//
//    pub fn update(&mut self, rate: f32) {
//        self.layer.update(rate);
//    }
//}
//
//#[derive(Debug, Clone, PartialEq)]
//pub struct ChobitDecoder<const OUT: usize, const IN: usize> {
//    gru_layer: GRULayer<OUT, IN>,
//    output_layer: Layer<OUT, OUT>
//}
//
//pub struct ChobitDecoderIter<'a, const OUT: usize, const IN: usize> {
//    decoder: &'a ChobitDecoder<OUT, IN>,
//    input: [f32; IN],
//
//    state: [f32; OUT]
//}
//
//impl<const OUT: usize, const IN: usize> ChobitDecoder<OUT, IN> {
//    pub fn new(
//        gru_layer: GRULayer<OUT, IN>,
//        output_layer: Layer<OUT, OUT>
//    ) -> Self {
//        Self {
//            gru_layer: gru_layer,
//            output_layer: output_layer
//        }
//    }
//
//    #[inline]
//    pub fn gru_layer(&self) -> &GRULayer<OUT, IN> {&self.gru_layer}
//
//    #[inline]
//    pub fn output_layer(&self) -> &Layer<OUT, OUT> {&self.output_layer}
//
//    #[inline]
//    pub fn start_calc<'a>(
//        &'a self,
//        input: &[f32; IN]
//    ) -> ChobitDecoderIter<'a, OUT, IN> {
//        ChobitDecoderIter::<OUT, IN> {
//            decoder: self,
//            input: *input,
//            state: [0.0; OUT]
//        }
//    }
//
//    #[inline]
//    pub fn study<'a>(
//        &mut self,
//        train_out: &mut impl Iterator<Item = &'a [f32; OUT]>,
//        train_in: &[f32; IN]
//    ) -> Option<[f32; IN]> {
//        let (feedback_next, _) =
//            self.study_core(train_out, train_in, &[0.0; OUT])?;
//
//        Some(feedback_next)
//    }
//
//    fn study_core<'a>(
//        &mut self,
//        train_out: &mut impl Iterator<Item = &'a [f32; OUT]>,
//        train_in: &[f32; IN],
//        initial_state: &[f32; OUT]
//    ) -> Option<([f32; IN], [f32; OUT])> {
//        let t_output = train_out.next()?;
//
//        let state = self.gru_layer.calc(train_in, initial_state);
//        let output_value = self.output_layer.calc(&state);
//
//        let mut loss = [0.0f32; OUT];
//
//        for i in 0..OUT {
//            loss[i] = output_value[i] - t_output[i];
//        }
//
//        match self.study_core(train_out, train_in, &state) {
//            Some((mut feedback_for_input, mut feedback)) => {
//                let feedback_2 = self.output_layer.study(&loss, &state);
//                for i in 0..OUT {
//                    feedback[i] += feedback_2[i];
//                }
//
//                let (feedback_for_input_2, feedback_next) =
//                    self.gru_layer.study(&feedback, train_in, initial_state);
//
//                for i in 0..IN {
//                    feedback_for_input[i] += feedback_for_input_2[i];
//                }
//
//                Some((feedback_for_input, feedback_next))
//            },
//
//            // Last output
//            None => {
//                let feedback = self.output_layer.study(&loss, &state);
//
//                let (feedback_for_input, feedback_next) =
//                    self.gru_layer.study(&feedback, train_in, initial_state);
//
//                Some((feedback_for_input, feedback_next))
//            }
//        }
//    }
//
//    pub fn update(&mut self, rate: f32) {
//        self.gru_layer.update(rate);
//        self.output_layer.update(rate);
//    }
//}
//
//impl<'a, const OUT: usize, const IN: usize> ChobitDecoderIter<'a, OUT, IN> {
//    #[inline]
//    pub fn calc_next(&mut self) -> [f32; OUT] {
//        self.state = self.decoder.gru_layer.calc(&self.input, &self.state);
//        self.decoder.output_layer.calc(&self.state)
//    }
//}
//
//impl<
//    'a,
//    const OUT: usize,
//    const IN: usize
//> Iterator for ChobitDecoderIter<'a, OUT, IN> {
//    type Item = [f32; OUT];
//
//    #[inline]
//    fn next(&mut self) -> Option<[f32; OUT]> {Some(self.calc_next())}
//}
//
//pub struct ChobitSeqAI<
//    const OUT: usize,
//    const MIDDLE: usize,
//    const IN: usize
//> {
//    decoder: ChobitDecoder<OUT, MIDDLE>,
//    encoder: ChobitEncoder<MIDDLE, IN>
//}
//
//impl<
//    const OUT: usize,
//    const MIDDLE: usize,
//    const IN: usize
//> ChobitSeqAI<OUT, MIDDLE, IN> {
//    #[inline]
//    pub fn new(
//        decoder: ChobitDecoder<OUT, MIDDLE>,
//        encoder: ChobitEncoder<MIDDLE, IN>
//    ) -> Self {
//        Self {
//            decoder: decoder,
//            encoder: encoder
//        }
//    }
//
//    #[inline]
//    pub fn decoder(&self) -> &ChobitDecoder<OUT, MIDDLE> {&self.decoder}
//
//    #[inline]
//    pub fn encoder(&self) -> &ChobitEncoder<MIDDLE, IN> {&self.encoder}
//
//    #[inline]
//    pub fn start_calc<'a>(
//        &self,
//        inputs: &mut impl Iterator<Item = &'a [f32; IN]>,
//        initial_state: &[f32; MIDDLE]
//    ) -> ChobitDecoderIter<OUT, MIDDLE> {
//        self.decoder.start_calc(&self.encoder.calc(inputs, initial_state))
//    }
//
//    pub fn study<'a>(
//        &mut self,
//        train_out: &mut impl Iterator<Item = &'a [f32; OUT]>,
//        train_in: &mut (impl Iterator<Item = &'a [f32; IN]> + Clone),
//        initial_state: &[f32; MIDDLE]
//    ) {
//        let mut train_in_clone = train_in.clone();
//
//        let middle_value = self.encoder.calc(train_in, initial_state);
//
//        match self.decoder.study(train_out, &middle_value) {
//            Some(feedback) => {
//                let _ = self.encoder.study_from_decorder(
//                    &feedback,
//                    &mut train_in_clone,
//                    initial_state
//                );
//            },
//
//            None => {}
//        }
//    }
//
//    pub fn update(&mut self, rate: f32) {
//        self.decoder.update(rate);
//        self.encoder.update(rate);
//    }
//}
