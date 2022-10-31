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
//!```
//! use chobit::chobit_ai::{
//!     Weights,
//!     Activation,
//!     Neuron,
//!     Layer,
//!     ChobitAI
//! };
//!
//! use chobit::chobit_rand::ChobitRand;
//!
//! //--------------------//
//! // Data set generator //
//! //--------------------//
//! // Output data generator for data set.
//! // This is like multiply of 2 complex numbers.
//! fn gen_output(input: &[f64; 4]) -> [f64; 2] {
//!     [
//!         (input[0] * input[2]) - (input[1] * input[3]),
//!         (input[0] * input[3]) + (input[1] * input[2])
//!     ]
//! }
//! 
//! // Input data generator for data set.
//! fn gen_input(rand: &mut ChobitRand) -> [f64; 4] {
//!     // converter from [0.0, 1.0] to [-1.0, 1.0].
//!     fn convert(x: f64) -> f64 {(x * 2.0) - 1.0}
//! 
//!     [
//!         convert(rand.next_f64()),
//!         convert(rand.next_f64()),
//!         convert(rand.next_f64()),
//!         convert(rand.next_f64())
//!     ]
//! }
//! 
//! // Data set generator.
//! fn gen_data_set(length: usize) -> Vec<([f64; 4], [f64; 2])> {
//!     let mut ret = Vec::<([f64; 4], [f64; 2])>::with_capacity(length);
//!     let mut rand = ChobitRand::new("This is a pen!".as_bytes());
//! 
//!     for _ in 0..length {
//!         let input = gen_input(&mut rand);
//!         let output = gen_output(&input);
//! 
//!         ret.push((input, output))
//!     }
//! 
//!     ret
//! }
//! 
//! //----------------//
//! // Ready data set //
//! //----------------//
//! // Generates data set.
//! let length: usize = 128;
//! let data_set = gen_data_set(length);
//! 
//! // Separates data_set into train_data and test_data.
//! let length = length / 2;
//! let train_data = data_set[..length].to_vec();
//! let test_data = data_set[length..].to_vec();
//! 
//! // Separates train_data into 4 batches.
//! let length = length / 4;
//! let mut batches = [
//!     train_data[..length].to_vec(),
//!     train_data[length..(length * 2)].to_vec(),
//!     train_data[(length * 2)..(length * 3)].to_vec(),
//!     train_data[(length * 3)..].to_vec()
//! ];
//! 
//! // Generates random number generator with seed bytes.
//! let mut rand = ChobitRand::new("Hello! I love to play game!".as_bytes());
//! 
//! // this is converter from [0.0, 1.0] into [-1.0, 1.0].
//! fn convert(x: f64) -> f64 {(x * 2.0) - 1.0}
//! 
//! //----------//
//! // Ready AI //
//! //----------//
//! // Decides numbers of input nodes, middle layer nodes, output nodes.
//! const IN: usize = 4;
//! const MIDDLE: usize = 32;
//! const OUT: usize = 2;
//! 
//! // Gererates weights of output nodes with random numbers.
//! let out_weights = [0u8; OUT].map(|_| {
//!     Weights::<MIDDLE>(
//!         [0u8; MIDDLE].map(|_| {
//!             convert(rand.next_f64())
//!         }),
//!         convert(rand.next_f64())  // bias
//!     )
//! });
//! 
//! // Gererates weights of middle nodes with random numbers.
//! let middle_weights = [0u8; MIDDLE].map(|_| {
//!     Weights::<IN>(
//!         [0u8; IN].map(|_| {
//!             convert(rand.next_f64())
//!         }),
//!         convert(rand.next_f64())  // bias
//!     )
//! });
//! 
//! // Generates output neurons with activate function.
//! let out_neurons = out_weights.map(|weights| {
//!     Neuron::<MIDDLE>::new(weights, Activation::Linear)
//! });
//! 
//! // Generates middle neurons with activate function.
//! let middle_neurons = middle_weights.map(|weights| {
//!     Neuron::<IN>::new(weights, Activation::ReLU)
//! });
//! 
//! // Generates output layer.
//! let output_layer = Layer::<OUT, MIDDLE>::new(out_neurons);
//! 
//! // Generates middle layer.
//! let middle_layer = Layer::<MIDDLE, IN>::new(middle_neurons);
//! 
//! // Generates AI.
//! let mut ai = ChobitAI::<OUT, MIDDLE, IN>::new(output_layer, middle_layer);
//! 
//! //-----------------------//
//! // Test without learning //
//! //-----------------------//
//! // Calculates test inputs.
//! let mut ai_output = Vec::<[f64; OUT]>::new();
//! 
//! test_data.iter().for_each(
//!     |(input, _)| ai_output.push(ai.calc(input))
//! );
//! 
//! // Calculates loss.
//! let mut before_loss: f64 = 0.0;
//! for i in 0..test_data.len() {
//!     for j in 0..OUT {
//!         before_loss += (ai_output[i][j] - test_data[i].1[j]).abs();
//!     }
//! }
//! std::println!("Before learning: {}", before_loss);
//! 
//! //----------//
//! // Learning //
//! //----------//
//! // Decides epoch and learning rate.
//! const EPOCH: usize = 100;
//! const RATE: f64 = 0.001;
//! 
//! // Learns.
//! for _ in 0..EPOCH {
//!     // Gets one batch.
//!     for batch in &mut batches {
//!         // Shuffle the batch.
//!         rand.shuffle(batch);
//! 
//!         // Learns each data.
//!         for data in batch {
//!             // Studies gradients. (not update weights yet.)
//!             ai.study(&data.1, &data.0);
//!         }
//! 
//!         // Updates weights with gradients.
//!         ai.update(RATE);
//!     }
//! }
//! 
//! //---------------------//
//! // Test after learning //
//! //---------------------//
//! // Calculates test inputs.
//! let mut ai_output = Vec::<[f64; OUT]>::new();
//! 
//! test_data.iter().for_each(
//!     |(input, _)| ai_output.push(ai.calc(input))
//! );
//! 
//! // Calculates loss.
//! let mut after_loss: f64 = 0.0;
//! for i in 0..test_data.len() {
//!     for j in 0..OUT {
//!         after_loss += (ai_output[i][j] - test_data[i].1[j]).abs();
//!     }
//! }
//! std::println!("After learning: {}", after_loss);
//! 
//! // Wishes to pass the following assertion...
//! assert!(after_loss < before_loss);
//!```

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
        RemAssign
    }
};

#[inline]
fn sqrt(x: f64) -> f64 {
    const MAGIC_64: u64 = 0x5fe6eb50c7b537aa;

    let a = x * 0.5;
    let y = f64::from_bits(MAGIC_64 - (x.to_bits() >> 1));

    y * (1.5 - (a * y * y))
}

/// Weights of a linear function.
///
/// ```
/// use chobit::chobit_ai::Weights;
/// use chobit::chobit_rand::ChobitRand;
///
/// let mut rand = ChobitRand::new("Initialize weights!".as_bytes());
///
/// const N: usize = 10;
///
/// let weights_1 = Weights::<N>(
///     [0u8; N].map(|_| rand.next_f64() * 10.0),
///     rand.next_f64() * 10.0
/// );
///
/// let weights_2 = Weights::<N>(
///     [0u8; N].map(|_| rand.next_f64() * 10.0),
///     rand.next_f64() * 10.0
/// );
///
/// let weights_3 = Weights::<N>::default();  // All elements is 0.0.
///
/// let weights_3 = weights_1 + weights_2;
/// let weights_3 = weights_1 - weights_2;
/// let weights_3 = weights_1 * 2.0;
/// let weights_3 = weights_1 / 2.0;
/// let mut weights_3 = weights_1 % 2.0;
///
/// weights_3 += weights_2;
/// weights_3 -= weights_2;
/// weights_3 *= 2.0;
/// weights_3 /= 2.0;
/// weights_3 %= 2.0;
///
/// // Inner product.
/// let val: f64 = weights_1 * weights_2;
///
/// // Linear function.
/// let vector: [f64; N] = [0u8; N].map(|_| rand.next_f64());
/// let val: f64 = weights_1 * vector;
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weights<const N: usize>(pub [f64; N], pub f64);

impl<const N: usize> Default for Weights<N> {
    #[inline]
    fn default() -> Self {
        Self([f64::default(); N], f64::default())
    }
}

impl<const N: usize> Add<Self> for Weights<N> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        let mut ret = Self::default();

        for i in 0..N {
            ret.0[i] = self.0[i] + other.0[i];
        }

        ret.1 = self.1 + other.1;

        ret
    }
}

impl<const N: usize> AddAssign<Self> for Weights<N> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        for i in 0..N {
            self.0[i] += other.0[i];
        }

        self.1 += other.1;
    }
}

impl<const N: usize> Sub<Self> for Weights<N> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        let mut ret = Self::default();

        for i in 0..N {
            ret.0[i] = self.0[i] - other.0[i];
        }

        ret.1 = self.1 - other.1;

        ret
    }
}

impl<const N: usize> SubAssign<Self> for Weights<N> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        for i in 0..N {
            self.0[i] -= other.0[i];
        }

        self.1 -= other.1;
    }
}

impl<const N: usize> Mul<f64> for Weights<N> {
    type Output = Self;

    #[inline]
    fn mul(self, other: f64) -> Self {
        let mut ret = Self::default();

        for i in 0..N {
            ret.0[i] = self.0[i] * other;
        }

        ret.1 = self.1 * other;

        ret
    }
}

impl<const N: usize> MulAssign<f64> for Weights<N> {
    #[inline]
    fn mul_assign(&mut self, other: f64) {
        for i in 0..N {
            self.0[i] *= other;
        }

        self.1 *= other;
    }
}

impl<const N: usize> Div<f64> for Weights<N> {
    type Output = Self;

    #[inline]
    fn div(self, other: f64) -> Self {
        let mut ret = Self::default();

        for i in 0..N {
            ret.0[i] = self.0[i] / other;
        }

        ret.1 = self.1 / other;

        ret
    }
}

impl<const N: usize> DivAssign<f64> for Weights<N> {
    #[inline]
    fn div_assign(&mut self, other: f64) {
        for i in 0..N {
            self.0[i] /= other;
        }

        self.1 /= other;
    }
}

impl<const N: usize> Rem<f64> for Weights<N> {
    type Output = Self;

    #[inline]
    fn rem(self, other: f64) -> Self {
        let mut ret = Self::default();

        for i in 0..N {
            ret.0[i] = self.0[i] % other;
        }

        ret.1 = self.1 % other;

        ret
    }
}

impl<const N: usize> RemAssign<f64> for Weights<N> {
    #[inline]
    fn rem_assign(&mut self, other: f64) {
        for i in 0..N {
            self.0[i] %= other;
        }

        self.1 %= other;
    }
}

impl<const N: usize> Mul<Self> for Weights<N> {
    type Output = f64;

    #[inline]
    fn mul(self, other: Self) -> f64 {
        let mut ret = self.1 * other.1;

        for i in 0..N {
            ret += self.0[i] * other.0[i]
        }

        ret
    }
}

impl<const N: usize> Mul<[f64; N]> for Weights<N> {
    type Output = f64;

    #[inline]
    fn mul(self, other: [f64; N]) -> f64 {
        let mut ret = self.1;

        for i in 0..N {
            ret += self.0[i] * other[i]
        }

        ret
    }
}

/// Activation function for Neuron.
///
/// See [Neuron].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Activation {
    /// No activation function.
    Linear,

    /// ReLU : `(-inf, +inf) -> [0.0, +inf)`
    ReLU,

    /// SoftSign : `(-inf, +inf) -> (-1.0, +1.0)`
    SoftSign
}

impl Activation {
    /// Calculates activation function.
    ///
    /// * `x` : Input number.
    /// * _Return_ : Output number.
    #[inline]
    pub fn activate(&self, x: f64) -> f64 {
        match self {
            Self::Linear => x,
            Self::ReLU => x.max(0.0),
            Self::SoftSign => Self::softsign(x)
        }
    }

    /// Calculates its derivative function.
    ///
    /// * `x` : Input number.
    /// * _Return_ : Differential coefficient.
    #[inline]
    pub fn d_activate(&self, x: f64) -> f64 {
        match self {
            Self::Linear => 1.0,

            Self::ReLU => if x <= 0.0 {0.0} else {1.0},

            Self::SoftSign => Self::d_softsign(x)
        }
    }

    #[inline]
    fn softsign_deno(x: f64) -> f64 {
        1.0 + x.max(-x)
    }

    #[inline]
    fn softsign(x: f64) -> f64 {
        x / Self::softsign_deno(x)
    }

    #[inline]
    fn d_softsign(x: f64) -> f64 {
        let deno = Self::softsign_deno(x);
        (deno * deno).recip()
    }
}

/// Neuron that is a part of AI.
#[derive(Debug, Clone, PartialEq)]
pub struct Neuron<const N: usize> {
    weights: Weights<N>,
    activation: Activation,

    total_grad: Weights<N>,
    study_count: f64,
    mome_1: Weights<N>,
    mome_2: f64
}

impl<const N: usize> Neuron<N> {
    /// Creates Neuron.
    ///
    /// * `weights` : Initial weights.
    /// * `activation` : Activation function.
    /// * _Return_ : Instance.
    #[inline]
    pub fn new(weights: Weights<N>, activation: Activation) -> Self {
        Self {
            weights: weights,
            activation: activation,

            total_grad: Weights::<N>::default(),
            study_count: 0.0,
            mome_1: Weights::<N>::default(),
            mome_2: 0.0
        }
    }

    /// Creates Neuron with study data.
    ///
    /// If you have been unfinished machine learning,
    /// you can continue it with this constructor.
    ///
    /// * `weights` : Initial weights.
    /// * `activation` : Activation function.
    /// * _Return_ : Instance.
    #[inline]
    pub fn with_study_data(
        weights: Weights<N>,
        activation: Activation,
        total_grad: Weights<N>,
        study_count: f64,
        mome_1: Weights<N>,
        mome_2: f64
    ) -> Self {
        Self {
            weights: weights,
            activation: activation,

            total_grad: total_grad,
            study_count: study_count,
            mome_1: mome_1,
            mome_2: mome_2
        }
    }

    /// Gets Weights.
    ///
    /// * _Return_ : Weights.
    #[inline]
    pub fn weights(&self) -> &Weights<N> {&self.weights}

    /// Gets activation function.
    ///
    /// * _Return_ : Activation function.
    #[inline]
    pub fn activation(&self) -> Activation {self.activation}


    /// Gets total gradients.
    ///
    /// * _Return_ : Total gradients.
    #[inline]
    pub fn total_grad(&self) -> &Weights<N> {&self.total_grad}

    /// Gets a count how many times it have been studying.
    ///
    /// * _Return_ : A count.
    #[inline]
    pub fn study_count(&self) -> f64 {self.study_count}

    /// Gets 1st momentum for Adam.
    ///
    /// * _Return_ : 1st momentum.
    #[inline]
    pub fn mome_1(&self) -> &Weights<N> {&self.mome_1}


    /// Gets 2nd momentum for Adam.
    ///
    /// * _Return_ : 2nd momentum.
    #[inline]
    pub fn mome_2(&self) -> f64 {self.mome_2}

    /// Calculates input by linear function and activation function.
    ///
    /// * `input` : Input vector.
    /// * _Return_ : Output number.
    #[inline]
    pub fn calc(&self, input: &[f64; N]) -> f64 {
        self.activation.activate(self.weights * *input)
    }

    /// Forgets data of gradients and momenta.
    #[inline]
    pub fn clear_study_data(&mut self) {
        self.total_grad = Weights::<N>::default();
        self.study_count = 0.0;
        self.mome_1 = Weights::<N>::default();
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
    pub fn study(&mut self, feedback: f64, input: &[f64; N]) -> [f64; N] {
        let (grad_w, grad_i) = self.grad(feedback, input);

        self.total_grad += grad_w;
        self.study_count += 1.0;

        grad_i
    }

    #[inline]
    fn grad(&self, feedback: f64, input: &[f64; N]) -> (Weights<N>, [f64; N]) {
        let factor =
            feedback * self.activation.d_activate(self.weights * *input);

        let mut grad_w = Weights::<N>(input.clone(), 1.0);
        grad_w *= factor;

        let grad_i = (self.weights * factor).0;

        (grad_w, grad_i)
    }

    /// Updates Weights to use studied gradients.
    ///
    /// * `rate` : Learning rate.
    pub fn update(&mut self, rate: f64) {
        let grad = self.total_grad / self.study_count;

        self.mome_1 = self.next_mome_1(&grad);
        self.mome_2 = self.next_mome_2(&grad);

        let mome_1 = Self::mome_1_hat(&self.mome_1);
        let mome_2 = Self::mome_2_hat(self.mome_2);

        self.weights -= Self::d_weights(&mome_1, mome_2, rate);

        self.total_grad = Weights::<N>::default();
        self.study_count = 0.0;
    }

    #[inline]
    fn next_mome_1(&self, grad: &Weights<N>) -> Weights<N> {
        const BETA: f64 = 0.9;
        const BETA_INV: f64 = 1.0 - BETA;

        (self.mome_1 * BETA) + (*grad * BETA_INV)
    }

    #[inline]
    fn next_mome_2(&self, grad: &Weights<N>) -> f64 {
        const BETA: f64 = 0.999;
        const BETA_INV: f64 = 1.0 - BETA;

        (self.mome_2 * BETA) + ((*grad * *grad) * BETA_INV)
    }

    #[inline]
    fn mome_1_hat(mome_1: &Weights<N>) -> Weights<N> {
        const BETA: f64 = 0.9;
        const BETA_INV: f64 = 1.0 - BETA;

        *mome_1 / BETA_INV
    }

    #[inline]
    fn mome_2_hat(mome_2: f64) -> f64 {
        const BETA: f64 = 0.999;
        const BETA_INV: f64 = 1.0 - BETA;

        mome_2 / BETA_INV
    }

    #[inline]
    fn d_weights(mome_1: &Weights<N>, mome_2: f64, rate: f64) -> Weights<N> {
        const EPSILION: f64 = 1.0e-8;

        (*mome_1 / (sqrt(mome_2) + EPSILION)) * rate
    }
}

/// Layer of AI.
///
/// * `OUT` : Dimension of output. It equals a number of Neurons.
/// * `IN` : Dimension of input. It equals a number of weights per one Neuron.
#[derive(Debug, Clone, PartialEq)]
pub struct Layer<const OUT: usize, const IN: usize> {
    neurons: [Neuron<IN>; OUT]
}

impl<const OUT: usize, const IN: usize> Layer<OUT, IN> {
    /// Creates Layer.
    ///
    /// `neurons` : Array of Neurons.
    /// _Return_ : Instance.
    #[inline]
    pub fn new(neurons: [Neuron<IN>; OUT]) -> Self {
        Self {neurons: neurons}
    }

    /// Gets neurons.
    ///
    /// _Return_ : neurons.
    #[inline]
    pub fn neurons(&self) -> &[Neuron<IN>; OUT] {&self.neurons}

    /// Calculates input.
    ///
    /// * `input` : Input vector.
    /// * _Return_ : Output vector.
    #[inline]
    pub fn calc(&self, input: &[f64; IN]) -> [f64; OUT] {
        let mut ret = [0.0f64; OUT];

        for i in 0..OUT {
            ret[i] = self.neurons[i].calc(input);
        }

        ret
    }

    /// Studies gradients.
    ///
    /// See [Neuron::study]
    ///
    /// * `feedback` : Feedback from next layer. See [Neuron::study]
    /// * `input` : Input vector
    /// * _Return_ : Feedback to previous layer. See [Neuron::study]
    pub fn study(
        &mut self,
        feedback: &[f64; OUT],
        input: &[f64; IN]
    ) -> [f64; IN] {
        let mut ret = [0.0f64; IN];

        for i in 0..OUT {
            let feedback_next = self.neurons[i].study(feedback[i], input);

            for j in 0..IN {
                ret[j] += feedback_next[j];
            }
        }

        ret
    }

    /// Updates Weights to use studied gradients.
    ///
    /// * `rate` : Learning rate.
    #[inline]
    pub fn update( &mut self, rate: f64) {
        self.neurons.iter_mut().for_each(|neuron| neuron.update(rate));
    }
}

/// Simple perceptron.
///
/// * `OUT` : Dimension of output.
/// * `MIDDLE` : Dimension of middle layer.
/// * `IN` : Dimension of input.
#[derive(Debug, Clone, PartialEq)]
pub struct ChobitAI<const OUT: usize, const MIDDLE: usize, const IN: usize> {
    output_layer: Layer<OUT, MIDDLE>,
    middle_layer: Layer<MIDDLE, IN>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> ChobitAI<OUT, MIDDLE, IN> {
    /// Creates ChobitAI.
    ///
    /// * `output_layer` : Output layer.
    /// * `middle_layer` : Middle Layer.
    #[inline]
    pub fn new(
        output_layer: Layer<OUT, MIDDLE>,
        middle_layer: Layer<MIDDLE, IN>
    ) -> Self {
        Self {
            output_layer: output_layer,
            middle_layer: middle_layer,
        }
    }

    /// Gets Output Layer
    ///
    /// * _Return_ : Output layer.
    #[inline]
    pub fn output_layer(&self) -> &Layer<OUT, MIDDLE> {&self.output_layer}


    /// Gets Middle Layer
    ///
    /// * _Return_ : Middle layer.
    #[inline]
    pub fn middle_layer(&self) -> &Layer<MIDDLE, IN> {&self.middle_layer}

    /// Calcurates input.
    ///
    /// * `input` : Input vector;
    /// * _Return_ : Output vector;
    #[inline]
    pub fn calc(&self, input: &[f64; IN]) -> [f64; OUT] {
        self.output_layer.calc(&self.middle_layer.calc(input))
    }

    /// Studies Gradients with training data.
    ///
    /// It only studies gradients. It doesn't update weights yet.
    ///
    /// * `train_out` : Output of training data.
    /// * `train_in` : Input of training data.
    pub fn study(&mut self, train_out: &[f64; OUT], train_in: &[f64; IN]) {
        let middle_value = self.middle_layer.calc(train_in);
        let out_value = self.output_layer.calc(&middle_value);

        let mut loss = [0.0f64; OUT];
        for i in 0..OUT {
            loss[i] = out_value[i] - train_out[i];
        }

        let _ = self.middle_layer.study(
            &self.output_layer.study(
                &loss,
                &middle_value
            ),
            train_in
        );
    }

    /// Updates Weights to use studied gradients.
    ///
    /// * `rate` : Learning rate.
    #[inline]
    pub fn update(&mut self, rate: f64) {
        self.output_layer.update(rate);
        self.middle_layer.update(rate);
    }
}