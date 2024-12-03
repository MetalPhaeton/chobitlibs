// Copyright (C) 2022 Hironori Ishibashi
//
// This work is free. You can redistribute it and/or modify it under the
// terms of the Do What The Fuck You Want To Public License, Version 2,
// as published by Sam Hocevar. See http://www.wtfpl.net/ for more details.

#![allow(dead_code)]

//! Neural network library.
//!
//! This library needs `alloc` crate.  
//! This AI works on single thread, but is able to work on `no_std`.  
//! (But you can devise to do machine learning in multithread. See the following list.)
//!
//! - [Example of Single Thread Machine Learning](#example-of-single-thread-machine-learning)
//! - [Example of Multithread Machine Learning](#example-of-multithread-machine-learning)
//!
//!
//! # Example of Single Thread Machine Learning
//!
//! Letter classifier.
//! - If inputs Japanese letter, outputs '日'.
//! - If inputs English letter, outputs 'E'.
//!
//! (1) Defines letter generator and ID.
//!
//! ```ignore
//! extern crate chobitlibs;
//! 
//! use chobitlibs::chobit_ai::{
//!     MathVec,
//!     Activation,
//!     ChobitAi,
//!     ChobitMlAi,
//!     MlAiCache
//! };
//! 
//! use chobitlibs::chobit_rand::ChobitRand;
//! 
//! fn japanese_letter(rng: &mut ChobitRand) -> char {
//!     let letters = [
//!         'あ', 'い', 'う', 'え', 'お',
//!         'か', 'き', 'く', 'け', 'こ',
//!         'さ', 'し', 'す', 'せ', 'そ'
//!     ];
//! 
//!     letters[(rng.next_u64() as usize) % letters.len()]
//! }
//! 
//! fn english_letter(rng: &mut ChobitRand) -> char {
//!     let letters = [
//!         'a', 'b', 'c', 'd', 'e',
//!         'f', 'g', 'h', 'i', 'j',
//!         'k', 'l', 'm', 'n', 'o'
//!     ];
//! 
//!     letters[(rng.next_u64() as usize) % letters.len()]
//! }
//! 
//! const JAPANESE_ID: char = '日';
//! const ENGLISH_ID: char = 'E';
//! ```
//!
//! (2) Creates [`ChobitAi`] and randomises weights.
//!
//! ```ignore
//! const OUT: usize = 32;
//! const MIDDLE: usize = 64;
//! const IN: usize = 32;
//!
//! let mut rng = ChobitRand::new(b"ChobitAi Example");
//!
//! let mut ai = ChobitAi::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//!
//! // Randomises weights.
//! ai.for_each_weight_mut(|weight| {
//!     *weight = ((rng.next_f64() as f32) * 2.0) - 1.0;
//! });
//!
//! let mut input = MathVec::<IN>::new();
//! let mut output = MathVec::<OUT>::new();
//! ```
//!
//! (3) Wraps AI with [`ChobitMlAi`] for machine learning.
//!
//! ```ignore
//! let mut ai = ChobitMlAi::<OUT, MIDDLE, IN>::new(ai);
//! let mut cache = MlAiCache::<OUT, MIDDLE, IN>::new();
//!
//! let mut input_error = MathVec::<IN>::new();
//! let mut output_error = MathVec::<OUT>::new();
//! ```
//!
//! (4) Machine learning.
//!
//! ```ignore
//! const EPOCH: usize = 1000;
//! const BATCH_SIZE: usize = 100;
//! const RATE: f32 = 0.01;
//!
//! for _ in 0..EPOCH {
//!     for _ in 0..BATCH_SIZE {
//!         //--- Learns Japanese ---//
//!         input.load_u32_label(japanese_letter(&mut rng) as u32);
//!         output.load_u32_label(JAPANESE_ID as u32);
//!
//!         // Writes cache.
//!         ai.ready(&input, &mut cache);
//!
//!         // Calculates error.
//!         cache.calc_output_error(&output, &mut output_error);
//!
//!         // Studies.
//!         ai.study(&output_error, &cache, &mut input_error);
//!
//!         //--- Learns English ---//
//!         input.load_u32_label(english_letter(&mut rng) as u32);
//!         output.load_u32_label(ENGLISH_ID as u32);
//!
//!         // Writes cache.
//!         ai.ready(&input, &mut cache);
//!
//!         // Calculates error.
//!         cache.calc_output_error(&output, &mut output_error);
//!
//!         // Studies.
//!         ai.study(&output_error, &cache, &mut input_error);
//!     }
//!
//!     // Updates weights.
//!     ai.update(RATE);
//! }
//! ```
//!
//! (5) Tests AI.
//!
//! ```ignore
//! // Unwrap Ai.
//! let ai = ai.drop();
//!
//! let mut tmpbuf = MathVec::<MIDDLE>::new();
//!
//! // Tests Japanese.
//! for _ in 0..10 {
//!     input.load_u32_label(japanese_letter(&mut rng) as u32);
//!
//!     ai.calc(&input, &mut output, &mut tmpbuf);
//!
//!     assert_eq!(output.to_u32_label(), JAPANESE_ID as u32);
//! }
//!
//! // Tests English.
//! for _ in 0..10 {
//!     input.load_u32_label(english_letter(&mut rng) as u32);
//!
//!     ai.calc(&input, &mut output, &mut tmpbuf);
//!
//!     assert_eq!(output.to_u32_label(), ENGLISH_ID as u32);
//! }
//! ```
//!
//! # Example of Multithread Machine Learning
//!
//! Letter classifier.
//! 
//! - If inputs Japanese letter, outputs '日'.
//! - If inputs English letter, outputs 'E'.
//! 
//! (1) Imports libraries.
//! 
//! ```ignore
//! extern crate chobitlibs;
//! 
//! use std::{
//!     thread,
//!     sync::{
//!         mpsc,
//!         Arc,
//!         Mutex
//!     }
//! };
//! 
//! use chobitlibs::chobit_ai::{
//!     MathVec,
//!     Activation,
//!     ChobitAi,
//!     ChobitMlAi,
//!     MlAiCache
//! };
//! 
//! use chobitlibs::chobit_rand::ChobitRand;
//! ```
//! 
//! (2) Defines constants.
//! 
//! ```ignore
//! const OUT: usize = 32;
//! const MIDDLE: usize = 64;
//! const IN: usize = 32;
//! 
//! const EPOCH: usize = 10000;
//! const BATCH_SIZE: usize = 30;
//! const RATE: f32 = 0.01;
//! ```
//! 
//! (3) Commands between threads.
//! 
//! ```ignore
//! // Command from parent to child.
//! enum P2C {
//!     StartEpoch,
//!     Break
//! }
//! 
//! // Command from child to parent.
//! enum C2P {
//!     EndedOneEpoch,
//! }
//! ```
//! 
//! (4) Defines letter generator and letter ID.
//! 
//! ```ignore
//! fn japanese_letter(rng: &mut ChobitRand) -> char {
//!     let letters = [
//!         'あ', 'い', 'う', 'え', 'お',
//!         'か', 'き', 'く', 'け', 'こ',
//!         'さ', 'し', 'す', 'せ', 'そ'
//!     ];
//! 
//!     letters[(rng.next_u64() as usize) % letters.len()]
//! }
//! 
//! fn english_letter(rng: &mut ChobitRand) -> char {
//!     let letters = [
//!         'a', 'b', 'c', 'd', 'e',
//!         'f', 'g', 'h', 'i', 'j',
//!         'k', 'l', 'm', 'n', 'o'
//!     ];
//! 
//!     letters[(rng.next_u64() as usize) % letters.len()]
//! }
//! 
//! const JAPANESE_ID: char = '日';
//! const ENGLISH_ID: char = 'E';
//! ```
//! 
//! (4) Defines functions to operate shared gradient.
//! 
//! ```ignore
//! // Initializes shared gradient.
//! fn init_grad(
//!     ai: &ChobitMlAi<OUT, MIDDLE, IN>,
//!     grads: &Arc<Mutex<(Vec<f32>, Vec<f32>)>>
//! ) {
//!     let mut lock = grads.lock().unwrap();
//! 
//!     ai.for_each_total_grad(|_| {
//!         lock.0.push(0.0);
//!         lock.1.push(0.0);
//!     });
//! }
//! 
//! // Loads from shared gradient to AI's gradient.
//! fn load_grad(
//!     ai: &mut ChobitMlAi<OUT, MIDDLE, IN>,
//!     grads: &Arc<Mutex<(Vec<f32>, Vec<f32>)>>
//! ) {
//!     let lock = grads.lock().unwrap();
//! 
//!     let mut iter = lock.0.iter();
//! 
//!     ai.for_each_total_grad_mut(|grad| {
//!         if let Some(shared_grad) = iter.next() {
//!             *grad = *shared_grad;
//!         }
//!     });
//! }
//! 
//! // Adds AI's gradient to shared gradient.
//! fn save_grad(
//!     ai: &ChobitMlAi<OUT, MIDDLE, IN>,
//!     grads: &Arc<Mutex<(Vec<f32>, Vec<f32>)>>
//! ) {
//!     let mut lock = grads.lock().unwrap();
//! 
//!     let mut iter = lock.1.iter_mut();
//! 
//!     ai.for_each_total_grad(|grad| {
//!         if let Some(shared_grad) = iter.next() {
//!             *shared_grad += *grad;
//!         }
//!     });
//! }
//! 
//! // Prepares to load_grad().
//! fn move_grad(
//!     grads: &Arc<Mutex<(Vec<f32>, Vec<f32>)>>,
//!     tmpbuf: &mut Vec<f32>
//! ) {
//!     let mut lock = grads.lock().unwrap();
//! 
//!     lock.0.clear();
//!     tmpbuf.clear();
//! 
//!     tmpbuf.extend_from_slice(&lock.1);
//! 
//!     lock.0.extend_from_slice(&tmpbuf);
//! 
//!     lock.1.fill(0.0);
//! }
//! ```
//! 
//! (5) Defines child thread.
//! 
//! ```ignore
//! fn run_thread(
//!     rng_seed: &[u8],
//!     p2c_rx: mpsc::Receiver<P2C>,
//!     c2p_tx: mpsc::Sender<C2P>,
//!     ai: ChobitAi<OUT, MIDDLE, IN>,
//!     grads: Arc<Mutex<(Vec<f32>, Vec<f32>)>>
//! ) -> thread::JoinHandle<()> {
//!     let rng_seed = rng_seed.to_vec();
//! 
//!     thread::spawn(move || {
//!         // Wraps AI for machine learning.
//!         let mut ai = ChobitMlAi::<OUT, MIDDLE, IN>::new(ai);
//! 
//!         let mut rng = ChobitRand::new(&rng_seed);
//! 
//!         let mut cache = MlAiCache::<OUT, MIDDLE, IN>::new();
//!         let mut input = MathVec::<IN>::new();
//!         let mut output = MathVec::<OUT>::new();
//! 
//!         let mut input_error = MathVec::<IN>::new();
//!         let mut output_error = MathVec::<OUT>::new();
//! 
//!         // Machine learning.
//!         for cmd in p2c_rx {  // Waits for command from parent.
//!             match cmd {
//!                 P2C::StartEpoch => {
//!                     // To load shared gradient and update itself,
//!                     // all children and parent become same state.
//!                     load_grad(&mut ai, &grads);
//!                     ai.update(RATE);
//! 
//!                     for _ in 0..BATCH_SIZE {
//!                         //--- Learns Japanese ---//
//!                         input.load_u32_label(japanese_letter(&mut rng) as u32);
//!                         output.load_u32_label(JAPANESE_ID as u32);
//! 
//!                         // Writes cache.
//!                         ai.ready(&input, &mut cache);
//! 
//!                         // Calculates error.
//!                         cache.calc_output_error(&output, &mut output_error);
//! 
//!                         // Studies.
//!                         ai.study(&output_error, &cache, &mut input_error);
//! 
//!                         //--- Learns English ---//
//!                         input.load_u32_label(english_letter(&mut rng) as u32);
//!                         output.load_u32_label(ENGLISH_ID as u32);
//! 
//!                         // Writes cache.
//!                         ai.ready(&input, &mut cache);
//! 
//!                         // Calculates error.
//!                         cache.calc_output_error(&output, &mut output_error);
//! 
//!                         // Studies.
//!                         ai.study(&output_error, &cache, &mut input_error);
//!                     }
//! 
//!                     // Adds its own gradient to shared gradient.
//!                     save_grad(&ai, &grads);
//! 
//!                     // Notifies ended one epoch to parent.
//!                     c2p_tx.send(C2P::EndedOneEpoch).unwrap();
//!                 },
//! 
//!                 P2C::Break => {
//!                     break;
//!                 }
//!             }
//!         }
//!     })
//! }
//! ```
//! 
//! (6) Defines AI test.
//! 
//! ```ignore
//! fn test_ai(ai: &ChobitAi<OUT, MIDDLE, IN>, rng: &mut ChobitRand) {
//!     let mut input = MathVec::<IN>::new();
//!     let mut output = MathVec::<OUT>::new();
//!     let mut tmpbuf = MathVec::<MIDDLE>::new();
//! 
//!     // Tests Japanese.
//!     for _ in 0..10 {
//!         input.load_u32_label(japanese_letter(rng) as u32);
//! 
//!         ai.calc(&input, &mut output, &mut tmpbuf);
//! 
//!         assert_eq!(output.to_u32_label(), JAPANESE_ID as u32);
//!     }
//! 
//!     // Tests English.
//!     for _ in 0..10 {
//!         input.load_u32_label(english_letter(rng) as u32);
//! 
//!         ai.calc(&input, &mut output, &mut tmpbuf);
//! 
//!         assert_eq!(output.to_u32_label(), ENGLISH_ID as u32);
//!     }
//! }
//! ```
//! 
//! (7) Starts main thread and Creates AI and randomise weights.
//! 
//! ```ignore
//! fn main() {
//!     let mut rng = ChobitRand::new(b"ChobitAi Example");
//! 
//!     let mut ai = ChobitAi::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//! 
//!     // Randomises weights.
//!     ai.for_each_weight_mut(|weight| {
//!         *weight = ((rng.next_f64() as f32) * 2.0) - 1.0;
//!     });
//! # }
//! ```
//! 
//! (8) Creates shared gradient.
//! 
//! ```ignore
//!     let grads = Arc::new(Mutex::new(
//!         (Vec::<f32>::new(), Vec::<f32>::new())
//!     ));
//! ```
//! 
//! (9) Creates commands for a parent and 2 children.
//! 
//! ```ignore
//!     // Channel from parent to child.
//!     let (p2c_1_tx, p2c_1_rx) = mpsc::channel::<P2C>();
//!     let (p2c_2_tx, p2c_2_rx) = mpsc::channel::<P2C>();
//!     let p2c_tx = [p2c_1_tx, p2c_2_tx];
//! 
//!     // Channel from child to parent.
//!     let (c2p_1_tx, c2p_1_rx) = mpsc::channel::<C2P>();
//!     let (c2p_2_tx, c2p_2_rx) = mpsc::channel::<C2P>();
//!     let c2p_rx = [c2p_1_rx, c2p_2_rx];
//! ```
//! 
//! (10) Runs 2 children.
//! 
//! ```ignore
//!     let handle_1 =
//!         run_thread(b"child_1", p2c_1_rx, c2p_1_tx, ai.clone(), grads.clone());
//! 
//!     let handle_2 =
//!         run_thread(b"child_2", p2c_2_rx, c2p_2_tx, ai.clone(), grads.clone());
//! ```
//! 
//! (11) Process on parent thread.
//! 
//! ```ignore
//!     // Wraps AI for machine learning.
//!     let mut ai = ChobitMlAi::<OUT, MIDDLE, IN>::new(ai);
//!     let mut tmpbuf = Vec::<f32>::new();
//! 
//!     // Initialize shared gradient.
//!     init_grad(&ai, &grads);
//! 
//!     // Loop each epoch.
//!     for _ in 0..EPOCH {
//!         load_grad(&mut ai, &grads);
//!         ai.update(RATE);
//! 
//!         p2c_tx.iter().for_each(|tx| {tx.send(P2C::StartEpoch).unwrap();});
//! 
//!         c2p_rx.iter().for_each(|rx| {
//!             let _ = rx.recv().unwrap();
//!         });
//! 
//!         move_grad(&grads, &mut tmpbuf);
//!     }
//! 
//!     p2c_tx.iter().for_each(|tx| {tx.send(P2C::Break).unwrap();});
//! 
//!     // Updates by gradient of last epoch.
//!     move_grad(&grads, &mut tmpbuf);
//!     load_grad(&mut ai, &grads);
//!     ai.update(RATE);
//! 
//!     handle_1.join().unwrap();
//!     handle_2.join().unwrap();
//! ```
//! 
//! (12) Tests and ends main thread.
//! 
//! ```ignore
//! # {
//!     let ai = ai.drop();
//! 
//!     test_ai(&ai, &mut rng);
//! }
//! ```
//!
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
    iter::Iterator
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
        $self.body.iter_mut().zip($other.body.iter()).for_each(
            |(self_val, other_val)| {
                *self_val $ops *other_val;
            }
        );
    }};
}

macro_rules! scalar_op {
    ($self:expr, $other:expr, $ops:tt) => {{
        $self.body.iter_mut().for_each(
            |self_val| {*self_val $ops $other;}
        );
    }};
}

/// Vector for mathematics.
///
/// - `N` : Dimension.
///
/// # Example
///
/// ```ignore
/// const DIM: usize = 3;
///
/// let mut v_1 = MathVec::<DIM>::new();
///
/// v_1[0] = 1.0;
/// v_1[1] = 2.0;
/// v_1[2] = 3.0;
///
/// let mut v_2 = MathVec::<DIM>::new();
///
/// v_2[0] = 4.0;
/// v_2[1] = 5.0;
/// v_2[2] = 6.0;
///
/// let x: f32 = 10.0;
/// ```
///
/// ## Addition
///
/// ```ignore
/// // Add
/// let mut v_3 = &v_1 + &v_2;
/// assert_eq!(v_3[0], 1.0 + 4.0);
/// assert_eq!(v_3[1], 2.0 + 5.0);
/// assert_eq!(v_3[2], 3.0 + 6.0);
///
/// // Add assign
/// v_3 += &v_1;
/// assert_eq!(v_3[0], 1.0 + 4.0 + 1.0);
/// assert_eq!(v_3[1], 2.0 + 5.0 + 2.0);
/// assert_eq!(v_3[2], 3.0 + 6.0 + 3.0);
/// ```
///
/// ## Subtract
///
/// ```ignore
/// // Sub
/// let mut v_3 = &v_1 - &v_2;
/// assert_eq!(v_3[0], 1.0 - 4.0);
/// assert_eq!(v_3[1], 2.0 - 5.0);
/// assert_eq!(v_3[2], 3.0 - 6.0);
///
/// // Sub assign
/// v_3 -= &v_1;
/// assert_eq!(v_3[0], 1.0 - 4.0 - 1.0);
/// assert_eq!(v_3[1], 2.0 - 5.0 - 2.0);
/// assert_eq!(v_3[2], 3.0 - 6.0 - 3.0);
/// ```
///
/// ## Multiply
///
/// ```ignore
/// // Multiply.
/// let mut v_3 = &v_1 * x;
/// assert_eq!(v_3[0], 1.0 * 10.0);
/// assert_eq!(v_3[1], 2.0 * 10.0);
/// assert_eq!(v_3[2], 3.0 * 10.0);
///
/// // Multiply assign.
/// v_3 *= x;
/// assert_eq!(v_3[0], 1.0 * 10.0 * 10.0);
/// assert_eq!(v_3[1], 2.0 * 10.0 * 10.0);
/// assert_eq!(v_3[2], 3.0 * 10.0 * 10.0);
/// ```
///
/// ## Division
///
/// ```ignore
/// // Division.
/// let mut v_3 = &v_1 / x;
/// assert_eq!(v_3[0], 1.0 / 10.0);
/// assert_eq!(v_3[1], 2.0 / 10.0);
/// assert_eq!(v_3[2], 3.0 / 10.0);
///
/// // Division assign.
/// v_3 /= x;
/// assert_eq!(v_3[0], 1.0 / 10.0 / 10.0);
/// assert_eq!(v_3[1], 2.0 / 10.0 / 10.0);
/// assert_eq!(v_3[2], 3.0 / 10.0 / 10.0);
/// ```
///
/// ## Division remainder
///
/// ```ignore
/// // Division remainder.
/// let mut v_3 = &v_1 % x;
/// assert_eq!(v_3[0], 1.0 % 10.0);
/// assert_eq!(v_3[1], 2.0 % 10.0);
/// assert_eq!(v_3[2], 3.0 % 10.0);
///
/// // Division remainder assign.
/// v_3 %= x;
/// assert_eq!(v_3[0], 1.0 % 10.0 % 10.0);
/// assert_eq!(v_3[1], 2.0 % 10.0 % 10.0);
/// assert_eq!(v_3[2], 3.0 % 10.0 % 10.0);
/// ```
///
/// ## Inner product
///
/// ```ignore
/// // Inner product.
/// let y = &v_1 * &v_2;
/// assert_eq!(y, (1.0 * 4.0) + (2.0 * 5.0) + (3.0 * 6.0));
/// ```
#[derive(Debug)]
pub struct MathVec<const N: usize> {
    body: Box<[f32]>,

    ptr: *const [f32; N],
    mut_ptr: *mut [f32; N]
}

impl<const N: usize> MathVec<N> {
    /// Creates MathVec.
    ///
    /// - _Return_ : MathVec.
    #[inline]
    pub fn new() -> Self {
        let mut body = vec![f32::default(); N].into_boxed_slice();

        let ptr = body.as_ptr() as *const [f32; N];
        let mut_ptr = body.as_mut_ptr() as *mut [f32; N];

        if cfg!(debug_assertions) {
            unsafe {
                assert_eq!(ptr as usize, mut_ptr as usize);

                assert_eq!(
                    ptr.add(1) as usize,
                    body.as_ptr().add(N) as usize
                );

                assert_eq!(
                    mut_ptr.add(1) as usize,
                    body.as_mut_ptr().add(N) as usize
                );
            }
        }

        Self {
            body: body,

            ptr: ptr,
            mut_ptr: mut_ptr
        }
    }

    /// Gets self as array.
    ///
    /// - _Return_ : Self as array.
    #[inline]
    pub fn as_array(&self) -> &[f32; N] {
        unsafe {&*self.ptr}
    }

    /// Gets self as mutable array.
    ///
    /// - _Return_ : Self as mutable array.
    #[inline]
    pub fn as_mut_array(&mut self) -> &mut [f32; N] {
        unsafe {&mut *self.mut_ptr}
    }

    /// Resets all values into 0.
    #[inline]
    pub fn clear(&mut self) {self.body.fill(f32::default());}

    /// Pointwise multiplication.
    ///
    /// - `other` : Other factor.
    /// - _Return_ : Result.
    #[inline]
    pub fn pointwise_mul(&self, other: &Self) -> Self {
        let mut ret = self.clone();

        pointwise_op!(ret, other, *=);

        ret
    }

    /// Pointwise multiplication and Assign.
    ///
    /// - `other` : Other factor.
    #[inline]
    pub fn pointwise_mul_assign(&mut self, other: &Self) {
        pointwise_op!(self, other, *=);
    }

    /// Pointwise division.
    ///
    /// - `other` : Divisor.
    /// - _Return_ : Result.
    #[inline]
    pub fn pointwise_div(&self, other: &Self) -> Self {
        let mut ret = self.clone();

        pointwise_op!(ret, other, /=);

        ret
    }

    /// Pointwise division and Assign.
    ///
    /// - `other` : Divisor.
    #[inline]
    pub fn pointwise_div_assign(&mut self, other: &Self) {
        pointwise_op!(self, other, /=);
    }

    /// Pointwise division remainder.
    ///
    /// - `other` : Divisor.
    /// - _Return_ : Result.
    #[inline]
    pub fn pointwise_rem(&self, other: &Self) -> Self {
        let mut ret = self.clone();

        pointwise_op!(ret, other, %=);

        ret
    }

    /// Pointwise division remainder and Assign.
    ///
    /// - `other` : Divisor.
    #[inline]
    pub fn pointwise_rem_assign(&mut self, other: &Self) {
        pointwise_op!(self, other, %=);
    }

    /// Copies from other vector.
    ///
    /// - `other` : Other vector.
    #[inline]
    pub fn copy_from(&mut self, other: &Self) {
        self.body.copy_from_slice(&*other.body);
    }

    /// Copies to other vector.
    ///
    /// - `other` : Other vector.
    #[inline]
    pub fn copy_to(&self, other: &mut Self) {
        other.body.copy_from_slice(&*self.body);
    }
}

unsafe impl<const N: usize> Send for MathVec<N> {}
unsafe impl<const N: usize> Sync for MathVec<N> {}

impl<const N: usize> Default for MathVec<N> {
    #[inline]
    fn default() -> Self {Self::new()}
}

impl<const N: usize> Clone for MathVec<N> {
    #[inline]
    fn clone(&self) -> Self {
        let mut ret = Self::new();

        ret.body.copy_from_slice(&*self.body);

        ret
    }
}

impl<const N: usize> PartialEq for MathVec<N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (&*self.body) == (&*other.body)
    }
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

impl MathVec<1> {
    /// Converts to `bool`.
    ///
    /// - _Return_ : Boolean value.
    #[inline]
    pub fn to_bool(&self) -> bool {
        unsafe {*self.body.get_unchecked(0) >= 0.0}
    }

    /// Loads values from `bool`.
    ///
    /// - `boolean_value` : Boolean value.
    #[inline]
    pub fn load_bool(&mut self, boolean_value: bool) {
        unsafe {
            *self.body.get_unchecked_mut(0) = if boolean_value {
                1.0
            } else {
                -1.0
            };
        }
    }
}

impl MathVec<8> {
    /// Converts to `u8` label.
    ///
    /// - _Return_ : Label.
    #[inline]
    pub fn to_u8_label(&self) -> u8 {
        to_label_body!(self, u8)
    }

    /// Loads values from `u8` label.
    ///
    /// - `label` : Label.
    #[inline]
    pub fn load_u8_label(&mut self, mut label: u8) {
        load_label_body!(self, u8, label)
    }
}

impl MathVec<16> {
    /// Converts to `u16` label.
    ///
    /// - _Return_ : Label.
    #[inline]
    pub fn to_u16_label(&self) -> u16 {
        to_label_body!(self, u16)
    }

    /// Loads values from `u16` label.
    ///
    /// - `label` : Label.
    #[inline]
    pub fn load_u16_label(&mut self, mut label: u16) {
        load_label_body!(self, u16, label)
    }
}

impl MathVec<32> {
    /// Converts to `u32` label.
    ///
    /// - _Return_ : Label.
    #[inline]
    pub fn to_u32_label(&self) -> u32 {
        to_label_body!(self, u32)
    }

    /// Loads values from `u32` label.
    ///
    /// - `label` : Label.
    #[inline]
    pub fn load_u32_label(&mut self, mut label: u32) {
        load_label_body!(self, u32, label)
    }
}

impl MathVec<64> {
    /// Converts to `u64` label.
    ///
    /// - _Return_ : Label.
    #[inline]
    pub fn to_u64_label(&self) -> u64 {
        to_label_body!(self, u64)
    }

    /// Loads values from `u64` label.
    ///
    /// - `label` : Label.
    #[inline]
    pub fn load_u64_label(&mut self, mut label: u64) {
        load_label_body!(self, u64, label)
    }
}

impl MathVec<128> {
    /// Converts to `u64` label.
    ///
    /// - _Return_ : Label.
    #[inline]
    pub fn to_u128_label(&self) -> u128 {
        to_label_body!(self, u128)
    }

    /// Loads values from `u64` label.
    ///
    /// - `label` : Label.
    #[inline]
    pub fn load_u128_label(&mut self, mut label: u128) {
        load_label_body!(self, u128, label)
    }
}

/// Weights of a linear function.
///
/// | Formula |
/// |:-:|
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mrow> <msub> <mi>y</mi> <mi>i</mi> </msub> <mo stretchy="false">=</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>j</mi> </munder> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> </mrow> </mrow> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">+</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>k</mi> </munder> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> </mrow> </mrow> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">+</mo> <msub> <mi>b</mi> <mi>i</mi> </msub> </mrow> </mrow> </semantics> </math> |
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mi>i</mi> <mo stretchy="false">≝</mo> <mtext>Index of output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>j</mi> <mo stretchy="false">≝</mo> <mtext>Index of input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mrow> <mi>k</mi> <mo stretchy="false">≝</mo> <mtext>Index of state.</mtext> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mtext>dim</mtext> <mspace width="0.5em"/> <mrow> <mi>k</mi> <mo stretchy="false">=</mo> <mtext>dim</mtext> </mrow> <mspace width="0.5em"/> <mi>i</mi> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>y</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Weights for input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Weights for state.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">≝</mo> <mtext>State.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>b</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Bias.</mtext> </mrow> </mtd> </mtr> </mtable> </semantics> </math> |
///
/// - `OUT` : Dimension of output.
/// - `IN` : Dimension of input.
#[derive(Debug)]
pub struct Weights<const OUT: usize, const IN: usize> {
    body: Box<[f32]>,

    ptr_b: *const [f32; OUT],
    ptr_i: *const [[f32; IN]; OUT],
    ptr_s: Option<*const [[f32; OUT]; OUT]>,

    mut_ptr_b: *mut [f32; OUT],
    mut_ptr_i: *mut [[f32; IN]; OUT],
    mut_ptr_s: Option<*mut [[f32; OUT]; OUT]>
}

impl<const OUT: usize, const IN: usize> Weights<OUT, IN> {
    /// Creates Weights.
    ///
    /// - `has_state_weights` : If `true`, this weights accepts state.
    /// - _Return_ : Weights.
    #[inline]
    pub fn new(has_state_weights: bool) -> Self {
        let bias_len = OUT;
        let input_weights_len = OUT * IN;
        let state_weights_len = OUT * OUT;

        if has_state_weights {
            let mut body = vec![
                f32::default();
                bias_len + input_weights_len + state_weights_len
            ].into_boxed_slice();

            let ptr_b = body.as_ptr() as *const [f32; OUT];
            let ptr_i = unsafe {ptr_b.add(1) as *const [[f32; IN]; OUT]};
            let ptr_s = unsafe {ptr_i.add(1) as *const [[f32; OUT]; OUT]};

            let mut_ptr_b = body.as_mut_ptr() as *mut [f32; OUT];
            let mut_ptr_i = unsafe {mut_ptr_b.add(1) as *mut [[f32; IN]; OUT]};
            let mut_ptr_s = unsafe {mut_ptr_i.add(1) as *mut [[f32; OUT]; OUT]};

            if cfg!(debug_assertions) {
                unsafe {
                    assert_eq!(ptr_b as usize, mut_ptr_b as usize);
                    assert_eq!(ptr_i as usize, mut_ptr_i as usize);
                    assert_eq!(ptr_s as usize, mut_ptr_s as usize);

                    let body_ptr = body.as_ptr();
                    assert_eq!(ptr_b as usize, body_ptr as usize);

                    let body_ptr = body_ptr.add(bias_len);
                    assert_eq!(ptr_i as usize, body_ptr as usize);
                    assert_eq!(ptr_i as usize, ptr_b.add(1) as usize);

                    let body_ptr = body_ptr.add(input_weights_len);
                    assert_eq!(ptr_s as usize, body_ptr as usize);
                    assert_eq!(ptr_s as usize, ptr_i.add(1) as usize);

                    let body_ptr = body_ptr.add(state_weights_len);
                    assert_eq!(
                        body.as_ptr().add(body.len()) as usize,
                        body_ptr as usize
                    );
                    assert_eq!(
                        body.as_ptr().add(body.len()) as usize,
                        ptr_s.add(1) as usize
                    );
                }
            }

            Self {
                body: body,

                ptr_b: ptr_b,
                ptr_i: ptr_i,
                ptr_s: Some(ptr_s),

                mut_ptr_b: mut_ptr_b,
                mut_ptr_i: mut_ptr_i,
                mut_ptr_s: Some(mut_ptr_s)
            }
        } else {
            let mut body = vec![
                f32::default();
                bias_len + input_weights_len
            ].into_boxed_slice();

            let ptr_b = body.as_ptr() as *const [f32; OUT];
            let ptr_i = unsafe {ptr_b.add(1) as *const [[f32; IN]; OUT]};

            let mut_ptr_b = body.as_mut_ptr() as *mut [f32; OUT];
            let mut_ptr_i = unsafe {mut_ptr_b.add(1) as *mut [[f32; IN]; OUT]};

            if cfg!(debug_assertions) {
                unsafe {
                    assert_eq!(ptr_b as usize, mut_ptr_b as usize);
                    assert_eq!(ptr_i as usize, mut_ptr_i as usize);

                    let body_ptr = body.as_ptr();
                    assert_eq!(ptr_b as usize, body_ptr as usize);

                    let body_ptr = body_ptr.add(bias_len);
                    assert_eq!(ptr_i as usize, body_ptr as usize);
                    assert_eq!(ptr_i as usize, ptr_b.add(1) as usize);

                    let body_ptr = body_ptr.add(input_weights_len);
                    assert_eq!(
                        body.as_ptr().add(body.len()) as usize,
                        body_ptr as usize
                    );
                    assert_eq!(
                        body.as_ptr().add(body.len()) as usize,
                        ptr_i.add(1) as usize
                    );
                }
            }

            Self {
                body: body,

                ptr_b: ptr_b,
                ptr_i: ptr_i,
                ptr_s: None,

                mut_ptr_b: mut_ptr_b,
                mut_ptr_i: mut_ptr_i,
                mut_ptr_s: None
            }
        }
    }

    /// Gets self as slice.
    ///
    /// - _Return_ : Self as slice.
    #[inline]
    pub fn as_slice(&self) -> &[f32] {&*self.body}

    /// Gets self as mutable slice.
    ///
    /// - _Return_ : Self as mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [f32] {&mut *self.body}

    /// Clear all weights into zero.
    #[inline]
    pub fn clear(&mut self) {self.body.fill(f32::default());}

    /// Gets immutable bias.
    ///
    /// - _Return_ : bias.
    #[inline]
    pub fn bias(&self) -> &[f32; OUT] {
        unsafe {&*self.ptr_b}
    }

    /// Gets mutable bias.
    ///
    /// - _Return_ : Bias.
    #[inline]
    pub fn bias_mut(&mut self) -> &mut [f32; OUT] {
        unsafe {&mut *self.mut_ptr_b}
    }

    /// Gets immutable weights for input.
    ///
    /// - _Return_ : Weights for input.
    #[inline]
    pub fn input_weights(&self) -> &[[f32; IN]; OUT] {
        unsafe {&*self.ptr_i}
    }

    /// Gets mutable weights for input.
    ///
    /// - _Return_ : Weights for input.
    #[inline]
    pub fn input_weights_mut(&mut self) -> &mut [[f32; IN]; OUT] {
        unsafe {&mut *self.mut_ptr_i}
    }

    /// Gets immutable weights for state.
    ///
    /// - _Return_ : Weights for state.
    #[inline]
    pub fn state_weights(&self) -> Option<&[[f32; OUT]; OUT]> {
        unsafe {self.ptr_s.map(|p| &*p)}
    }

    /// Gets mutable weights for state.
    ///
    /// - _Return_ : Weights for state.
    #[inline]
    pub fn state_weights_mut(&mut self) -> Option<&mut [[f32; OUT]; OUT]> {
        unsafe {self.mut_ptr_s.map(|p| &mut *p)}
    }

    /// Calculates linear function.
    ///
    /// - `input` : Input.
    /// - `state` : State for RNN.
    /// - `output` : Buffer for output.
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
        *output.as_mut_array() = *self.bias();
    }

    #[inline]
    fn calc_input(&self, input: &MathVec<IN>, output: &mut MathVec<OUT>) {
        self.input_weights().iter().zip(
            output.as_mut_array().iter_mut()
        ).for_each(|(weights, output_one)| {
            weights.iter().zip(input.as_array().iter()).for_each(
                |(w, i)| {*output_one += *w * *i;}
            );
        });
    }

    #[inline]
    fn calc_state(&self, state: &MathVec<OUT>, output: &mut MathVec<OUT>) {
        if let Some(state_weights) = self.state_weights() {
            state_weights.iter().zip(
                output.as_mut_array().iter_mut()
            ).for_each(|(weights, output_one)| {
                weights.iter().zip(state.as_array().iter()).for_each(
                    |(w, s)| {*output_one += *w * *s;}
                );
            });
        }
    }

    /// Calculates gradient with respect to input.
    ///
    /// | Formula |
    /// |:-:|
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>i</mi> </munder> <msub> <mi>C</mi> <mi>i</mi> </msub> </mrow> <mrow> <mrow> <mfrac> <mo stretchy="false">∂</mo> <mrow> <mo stretchy="false">∂</mo> <msub> <mi>x</mi> <mi>j</mi> </msub> </mrow> </mfrac> <msub> <mrow> <mo fence="true" form="prefix" stretchy="true">(</mo> <mrow> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>j</mi> </munder> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> </mrow> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">+</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>k</mi> </munder> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> </mrow> </mrow> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">+</mo> <msub> <mi>b</mi> <mi>i</mi> </msub> </mrow> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="true">)</mo> </mrow> <mi>i</mi> </msub> </mrow> <mo stretchy="false">=</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>i</mi> </munder> <msub> <mi>C</mi> <mi>i</mi> </msub> </mrow> </mrow> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> </mrow> </semantics> </math> |
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mi>i</mi> <mo stretchy="false">≝</mo> <mtext>Index of output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>j</mi> <mo stretchy="false">≝</mo> <mtext>Index of input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mrow> <mi>k</mi> <mo stretchy="false">≝</mo> <mtext>Index of state.</mtext> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mtext>dim</mtext> <mspace width="0.5em"/> <mrow> <mi>k</mi> <mo stretchy="false">=</mo> <mtext>dim</mtext> </mrow> <mspace width="0.5em"/> <mi>i</mi> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>C</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Coeffcients</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Weights for input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Weights for state.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">≝</mo> <mtext>State.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>b</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Bias.</mtext> </mrow> </mtd> </mtr> </mtable> </semantics> </math> |
    ///
    /// - `coefficient` : Coefficient.
    /// - `grad` : Buffer for output.
    pub fn grad_with_input(
        &self,
        coefficient: &MathVec<OUT>,
        grad: &mut MathVec<IN>
    ) {
        grad.clear();

        self.input_weights().iter().zip(
            coefficient.as_array().iter()
        ).for_each(
            |(weights, c)| {
                weights.iter().zip(grad.as_mut_array().iter_mut()).for_each(
                    |(w, g)| {*g += *c * *w;}
                );
            }
        );
    }

    /// Calculates gradient with respect to state.
    ///
    /// | Formula |
    /// |:-:|
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>i</mi> </munder> <msub> <mi>C</mi> <mi>i</mi> </msub> </mrow> <mrow> <mrow> <mfrac> <mo stretchy="false">∂</mo> <mrow> <mo stretchy="false">∂</mo> <msub> <mi>s</mi> <mi>k</mi> </msub> </mrow> </mfrac> <msub> <mrow> <mo fence="true" form="prefix" stretchy="true">(</mo> <mrow> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>j</mi> </munder> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> </mrow> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">+</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>k</mi> </munder> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> </mrow> </mrow> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">+</mo> <msub> <mi>b</mi> <mi>i</mi> </msub> </mrow> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="true">)</mo> </mrow> <mi>i</mi> </msub> </mrow> <mo stretchy="false">=</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>i</mi> </munder> <msub> <mi>C</mi> <mi>i</mi> </msub> </mrow> </mrow> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> </mrow> </semantics> </math> |
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mi>i</mi> <mo stretchy="false">≝</mo> <mtext>Index of output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>j</mi> <mo stretchy="false">≝</mo> <mtext>Index of input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mrow> <mi>k</mi> <mo stretchy="false">≝</mo> <mtext>Index of state.</mtext> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mtext>dim</mtext> <mspace width="0.5em"/> <mrow> <mi>k</mi> <mo stretchy="false">=</mo> <mtext>dim</mtext> </mrow> <mspace width="0.5em"/> <mi>i</mi> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>C</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Coeffcients</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Weights for input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Weights for state.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">≝</mo> <mtext>State.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>b</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Bias.</mtext> </mrow> </mtd> </mtr> </mtable> </semantics> </math> |
    ///
    /// - `coefficient` : Coefficient.
    /// - `grad` : Buffer for output.
    pub fn grad_with_state(
        &self,
        coefficient: &MathVec<OUT>,
        grad: &mut MathVec<OUT>
    ) {
        grad.clear();

        if let Some(state_weights) = self.state_weights() {
            state_weights.iter().zip(
                coefficient.as_array().iter()
            ).for_each(
                |(weights, c)| {
                    weights.iter().zip(grad.as_mut_array().iter_mut()).for_each(
                        |(w, g)| {*g += *c * *w;}
                    );
                }
            );
        }
    }

    /// Calculates gradient with respect to weights.
    ///
    /// | Formula |
    /// |:-:|
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <msub> <mi>C</mi> <mi>i</mi> </msub> <mrow> <mrow> <mfrac> <mo stretchy="false">∂</mo> <mrow> <mo stretchy="false">∂</mo> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> </mrow> </mfrac> <msub> <mrow> <mo fence="true" form="prefix" stretchy="true">(</mo> <mrow> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>j</mi> </munder> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> </mrow> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">+</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>k</mi> </munder> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> </mrow> </mrow> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">+</mo> <msub> <mi>b</mi> <mi>i</mi> </msub> </mrow> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="true">)</mo> </mrow> <mi>i</mi> </msub> </mrow> <mo stretchy="false">=</mo> <msub> <mi>C</mi> <mi>i</mi> </msub> </mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> </mrow> </semantics> </math> |
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <msub> <mi>C</mi> <mi>i</mi> </msub> <mrow> <mrow> <mfrac> <mo stretchy="false">∂</mo> <mrow> <mo stretchy="false">∂</mo> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> </mrow> </mfrac> <msub> <mrow> <mo fence="true" form="prefix" stretchy="true">(</mo> <mrow> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>j</mi> </munder> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> </mrow> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">+</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>k</mi> </munder> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> </mrow> </mrow> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">+</mo> <msub> <mi>b</mi> <mi>i</mi> </msub> </mrow> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="true">)</mo> </mrow> <mi>i</mi> </msub> </mrow> <mo stretchy="false">=</mo> <msub> <mi>C</mi> <mi>i</mi> </msub> </mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> </mrow> </semantics> </math> |
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <msub> <mi>C</mi> <mi>i</mi> </msub> <mrow> <mrow> <mfrac> <mo stretchy="false">∂</mo> <mrow> <mo stretchy="false">∂</mo> <msub> <mi>b</mi> <mi>i</mi> </msub> </mrow> </mfrac> <msub> <mrow> <mo fence="true" form="prefix" stretchy="true">(</mo> <mrow> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>j</mi> </munder> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> </mrow> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">+</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>k</mi> </munder> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> </mrow> </mrow> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">+</mo> <msub> <mi>b</mi> <mi>i</mi> </msub> </mrow> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="true">)</mo> </mrow> <mi>i</mi> </msub> </mrow> <mo stretchy="false">=</mo> <msub> <mi>C</mi> <mi>i</mi> </msub> </mrow> </mrow> </semantics> </math> |
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mi>i</mi> <mo stretchy="false">≝</mo> <mtext>Index of output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>j</mi> <mo stretchy="false">≝</mo> <mtext>Index of input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mrow> <mi>k</mi> <mo stretchy="false">≝</mo> <mtext>Index of state.</mtext> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mtext>dim</mtext> <mspace width="0.5em"/> <mrow> <mi>k</mi> <mo stretchy="false">=</mo> <mtext>dim</mtext> </mrow> <mspace width="0.5em"/> <mi>i</mi> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>C</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Coeffcients</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Weights for input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Weights for state.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">≝</mo> <mtext>State.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>b</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Bias.</mtext> </mrow> </mtd> </mtr> </mtable> </semantics> </math> |
    ///
    /// - `coefficient` : Coefficient.
    /// - `grad` : Buffer for output.
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
        *grad.bias_mut() = *coefficient.as_array();
    }

    #[inline]
    fn grad_with_weights_i(
        coefficient: &MathVec<OUT>,
        input: &MathVec<IN>,
        grad: &mut Self
    ) {
        grad.input_weights_mut().iter_mut().zip(
            coefficient.as_array().iter()
        ).for_each(
            |(grad_i, c)| {
                grad_i.iter_mut().zip(input.as_array().iter()).for_each(
                    |(g, i)| {*g += *c * *i;}
                );
            }
        );
    }

    #[inline]
    fn grad_with_weights_s(
        coefficient: &MathVec<OUT>,
        state: &MathVec<OUT>,
        grad: &mut Self
    ) {
        if let Some(state_weights) = grad.state_weights_mut() {
            state_weights.iter_mut().zip(
                coefficient.as_array().iter()
            ).for_each(
                |(grad_s, c)| {
                    grad_s.iter_mut().zip(state.as_array().iter()).for_each(
                        |(g, i)| {*g += *c * *i;}
                    );
                }
            );
        }
    }

    /// Copies from other weights.
    ///
    /// - `other` : Other weights.
    #[inline]
    pub fn copy_from(&mut self, other: &Self) {
        let len = self.body.len().min(other.body.len());

        self.body[..len].copy_from_slice(&other[..len])
    }

    /// Copies to other weights.
    ///
    /// - `other` : Other weights.
    #[inline]
    pub fn copy_to(&self, other: &mut Self) {
        other.copy_from(self);
    }
}

unsafe impl<const OUT: usize, const IN: usize> Sync for Weights<OUT, IN> {}
unsafe impl<const OUT: usize, const IN: usize> Send for Weights<OUT, IN> {}

impl<const OUT: usize, const IN: usize> Clone for Weights<OUT, IN> {
    #[inline]
    fn clone(&self) -> Self {
        let mut ret = Self::new(self.ptr_s.is_some());

        ret.body.copy_from_slice(&*self.body);

        ret
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
    /// - `x` : Input number.
    /// - _Return_ : Output number.
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
    /// - `x` : Input number.
    /// - _Return_ : Differential coefficient.
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

/// Layer for neural network only for calculating.
///
/// | Formula |
/// |:-:|
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mrow> <msub> <mi>y</mi> <mi>i</mi> </msub> <mo stretchy="false">=</mo> <mi>φ</mi> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="true">(</mo> <mrow> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>j</mi> </munder> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> </mrow> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">+</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>k</mi> </munder> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> </mrow> </mrow> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">+</mo> <msub> <mi>b</mi> <mi>i</mi> </msub> </mrow> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="true">)</mo> </mrow> </mrow> </semantics> </math> |
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mi>i</mi> <mo stretchy="false">≝</mo> <mtext>Index of output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>j</mi> <mo stretchy="false">≝</mo> <mtext>Index of input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mrow> <mi>k</mi> <mo stretchy="false">≝</mo> <mtext>Index of state.</mtext> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mtext>dim</mtext> <mspace width="0.5em"/> <mrow> <mi>k</mi> <mo stretchy="false">=</mo> <mtext>dim</mtext> </mrow> <mspace width="0.5em"/> <mi>i</mi> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>y</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>φ</mi> <mo stretchy="false">≝</mo> <mtext>Activation functin.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Weights for input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Weights for state.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">≝</mo> <mtext>State.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>b</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Bias.</mtext> </mrow> </mtd> </mtr> </mtable> </semantics> </math> ||
///
/// - `OUT` : Dimension of output.
/// - `IN` : Dimension of input.
#[derive(Debug, Clone, PartialEq)]
pub struct Layer<const OUT: usize, const IN: usize> {
    weights: Weights<OUT, IN>,
    activation: Activation
}

impl<const OUT: usize, const IN: usize> Layer<OUT, IN> {
    /// Creates Layer.
    ///
    /// - `activation` : Activation function.
    /// - `accept_state` : If `true`, this accepts state.
    /// - _Return_ : Neuron.
    #[inline]
    pub fn new(activation: Activation, accept_state: bool) -> Self {
        Self {
            weights: Weights::<OUT, IN>::new(accept_state),
            activation: activation
        }
    }

    /// Gets immutable weights.
    ///
    /// - _Return_ : Weights.
    #[inline]
    pub fn weights(&self) -> &Weights<OUT, IN> {&self.weights}

    /// Gets mutable weights.
    ///
    /// - _Return_ : Weights.
    #[inline]
    pub fn mut_weights(&mut self) -> &mut Weights<OUT, IN> {&mut self.weights}

    /// Gets immutable activation function.
    ///
    /// - _Return_ : Weights.
    #[inline]
    pub fn activation(&self) -> &Activation {&self.activation}

    /// Gets mutable activation function.
    ///
    /// - _Return_ : Weights.
    #[inline]
    pub fn mut_activation(&mut self) -> &mut Activation {&mut self.activation}

    /// Calculates neural network layer.
    ///
    /// - `input` : Input.
    /// - `state` : State if it exists.
    /// - `output` : Buffer for output.
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

/// Cache for [`MlLayer`].
///
/// - `OUT` : `OUT` of [`MlLayer`].
/// - `IN` : `IN` of [`MlLayer`].
#[derive(Debug, Clone, PartialEq)]
pub struct MlCache<const OUT: usize, const IN: usize> {
    input: MathVec<IN>,
    state: MathVec<OUT>,
    has_state: bool,

    middle_value: MathVec<OUT>,

    d_activate: MathVec<OUT>,
    output: MathVec<OUT>
}

impl<const OUT: usize, const IN: usize> MlCache<OUT, IN> {
    /// Creates MlCache.
    ///
    /// - _Return_ : MlCache.
    #[inline]
    pub fn new() -> Self {
        Self {
            input: MathVec::<IN>::new(),
            state: MathVec::<OUT>::new(),
            has_state: false,

            middle_value: MathVec::<OUT>::new(),

            d_activate: MathVec::<OUT>::new(),
            output: MathVec::<OUT>::new()
        }
    }

    /// Calculates output error.
    ///
    /// | Formula |
    /// |:-:|
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mi>e</mi> <mo stretchy="false">=</mo> <mrow> <mi>o</mi> <mo stretchy="false">−</mo> <mi>t</mi> </mrow> </mrow> </semantics> </math> |
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mi>e</mi> <mo stretchy="false">≝</mo> <mtext>Error.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>o</mi> <mo stretchy="false">≝</mo> <mtext>Actual output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>t</mi> <mo stretchy="false">≝</mo> <mtext>Correct output.</mtext> </mrow> </mtd> </mtr> </mtable> </semantics> </math> |
    ///
    /// - `train_out` : Correct output.
    /// - `output_error` : Buffer for output error.
    #[inline]
    pub fn calc_output_error(
        &self,
        train_out: &MathVec<OUT>,
        output_error: &mut MathVec<OUT>
    ) {
        output_error.copy_from(&self.output);
        *output_error -= train_out;
    }

    /// Gets input.
    ///
    /// - _Return_ : Input.
    #[inline]
    pub fn input(&self) -> &MathVec<IN> {&self.input}

    /// Gets state.
    ///
    /// - _Return_ : State.
    #[inline]
    pub fn state(&self) -> Option<&MathVec<OUT>> {
        self.has_state.then(|| &self.state)
    }

    /// Gets middle value. (Output before activate.)
    ///
    /// - _Return_ : Middle value.
    #[inline]
    pub fn middle_value(&self) -> &MathVec<OUT> {&self.middle_value}

    /// Gets derivative of activation function.
    ///
    /// - _Return_ : Derivative of activation function.
    #[inline]
    pub fn d_activate(&self) -> &MathVec<OUT> {&self.d_activate}

    /// Gets output. (Output after activate.)
    ///
    /// - _Return_ : Output.
    #[inline]
    pub fn output(&self) -> &MathVec<OUT> {&self.output}
}

/// Layer for neural network only for machine learning.
///
/// | Formula |
/// |:-:|
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mrow> <msub> <mi>y</mi> <mi>i</mi> </msub> <mo stretchy="false">=</mo> <mi>φ</mi> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="true">(</mo> <mrow> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>j</mi> </munder> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> </mrow> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">+</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>k</mi> </munder> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> </mrow> </mrow> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">+</mo> <msub> <mi>b</mi> <mi>i</mi> </msub> </mrow> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="true">)</mo> </mrow> </mrow> </semantics> </math> |
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mi>i</mi> <mo stretchy="false">≝</mo> <mtext>Index of output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>j</mi> <mo stretchy="false">≝</mo> <mtext>Index of input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mrow> <mi>k</mi> <mo stretchy="false">≝</mo> <mtext>Index of state.</mtext> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mtext>dim</mtext> <mspace width="0.5em"/> <mrow> <mi>k</mi> <mo stretchy="false">=</mo> <mtext>dim</mtext> </mrow> <mspace width="0.5em"/> <mi>i</mi> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>y</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>φ</mi> <mo stretchy="false">≝</mo> <mtext>Activation functin.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>W</mi> <mi mathvariant="italic">ij</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Weights for input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>U</mi> <mi mathvariant="italic">ik</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Weights for state.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo stretchy="false">≝</mo> <mtext>State.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>b</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Bias.</mtext> </mrow> </mtd> </mtr> </mtable> </semantics> </math> ||
///
/// - `OUT` : Dimension of output.
/// - `IN` : Dimension of input.
#[derive(Debug, Clone, PartialEq)]
pub struct MlLayer<const OUT: usize, const IN: usize> {
    layer: Layer<OUT, IN>,

    total_grad: Weights<OUT, IN>,
    momentum_1: Weights<OUT, IN>,
    momentum_2: MathVec<OUT>,

    tmp_error: MathVec<OUT>,
    tmp_grad: Weights<OUT, IN>
}

const BETA_1: f32 = 0.9;
const BETA_INV_1: f32 = 1.0 - BETA_1;

const BETA_2: f32 = 0.999;
const BETA_INV_2: f32 = 1.0 - BETA_2;
impl<const OUT: usize, const IN: usize> MlLayer<OUT, IN> {
    /// Creates MlLayer.
    ///
    /// - `layer` : Base [`Layer`].
    /// - _Return_ : MlLayer.
    #[inline]
    pub fn new(layer: Layer<OUT, IN>) -> Self {
        let has_state_weights = layer.weights.ptr_s.is_some();

        Self {
            layer: layer,

            total_grad: Weights::<OUT, IN>::new(has_state_weights),
            momentum_1: Weights::<OUT, IN>::new(has_state_weights),
            momentum_2: MathVec::<OUT>::default(),

            tmp_error: MathVec::<OUT>::default(),
            tmp_grad: Weights::<OUT, IN>::new(has_state_weights)
        }
    }

    /// Drops Base [`Layer`].
    ///
    /// - _Return_ : [`Layer`].
    #[inline]
    pub fn drop(self) -> Layer<OUT, IN> {self.layer}

    /// Clears internal data for study.
    #[inline]
    pub fn clear_study_data(&mut self) {
        self.total_grad.clear();
        self.momentum_1.clear();
        self.momentum_2.clear();
    }

    /// Gets immutable total gradient.
    ///
    /// Total gradient is increased every time to call [`study()`](Self::study()).
    ///
    /// - _Return_ : Total gradient.
    #[inline]
    pub fn total_grad(&self) -> &Weights<OUT, IN> {&self.total_grad}

    /// Gets mutable total gradient.
    ///
    /// Total gradient is increased every time to call [`study()`](Self::study()).
    ///
    /// - _Return_ : Total gradient.
    #[inline]
    pub fn total_grad_mut(&mut self) -> &mut Weights<OUT, IN> {
        &mut self.total_grad
    }

    /// Writes infomation on [`MlCache`] for [`study()`](Self::study()).
    ///
    /// - `input` : Input.
    /// - `state` : State if it exists.
    /// - `cache` : Cache.
    pub fn ready(
        &self,
        input: &MathVec<IN>,
        state: Option<&MathVec<OUT>>,
        cache: &mut MlCache<OUT, IN>
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

        cache.d_activate.as_mut_array().iter_mut().zip(
            cache.middle_value.as_array().iter()
        ).for_each(|(d_f, m_value)| {
            *d_f = self.layer.activation.d_activate(*m_value);
        });

        cache.output.as_mut_array().iter_mut().zip(
            cache.middle_value.as_array().iter()
        ).for_each(|(output_one, m_value)| {
            *output_one = self.layer.activation.activate(*m_value);
        });
    }

    /// Accumulates gradient.
    ///
    /// - `output_error` : Backpropagated output error.
    /// - `next_state_error` : Backpropagated state error if it exists.
    /// - `cache` : Cache generated by [MlLayer::ready].
    /// - `input_error` : Error for previous output.
    /// - `prev_state_error` : Error for previous state.
    pub fn study(
        &mut self,
        output_error: &MathVec<OUT>,
        next_state_error: Option<&MathVec<OUT>>,
        cache: &MlCache<OUT, IN>,
        input_error: &mut MathVec<IN>,
        prev_state_error: Option<&mut MathVec<OUT>>
    ) {
        self.calc_tmp_error(output_error, next_state_error, cache);

        // add self.total_grad ----------
        Weights::grad_with_weights(
            &self.tmp_error,
            &cache.input,
            cache.has_state.then(|| &cache.state),
            &mut self.tmp_grad
        );

        self.total_grad.iter_mut().zip(self.tmp_grad.iter()).for_each(
            |(total_g, tmp_g)| {
                *total_g += *tmp_g;
            }
        );

        // calc errors ----------
        self.layer.weights.grad_with_input(&self.tmp_error, input_error);

        if let Some(prev_state_error) = prev_state_error {
            if cache.has_state {
                self.layer.weights.grad_with_state(
                    &self.tmp_error,
                    prev_state_error
                );
            }
        }
    }

    #[inline]
    fn calc_tmp_error(
        &mut self,
        output_error: &MathVec<OUT>,
        next_state_error: Option<&MathVec<OUT>>,
        cache: &MlCache<OUT, IN>
    ) {
        match next_state_error {
            Some(next_state_error) => {
                self.tmp_error.copy_from(next_state_error);
            },

            None => {
                self.tmp_error.clear();
            }
        }

        self.tmp_error.as_mut_array().iter_mut().zip(
            output_error.as_array().iter()
        ).zip(
            cache.d_activate.as_array().iter()
        ).for_each(
            |((tmp_e, output_e), d_f)| {
                *tmp_e += *output_e * *d_f;
            }
        );
    }

    /// Updates weights with Adam.
    ///
    /// | Formula |
    /// |:-:|
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mstyle mathvariant="bold"> <mover accent="true"> <mi>v</mi> <mo stretchy="false">^</mo> </mover> </mstyle> <mo stretchy="false">=</mo> <mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <msub> <mi>β</mi> <mn>1</mn> </msub> <mstyle mathvariant="bold"> <mi>v</mi> </mstyle> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> <mo stretchy="false">+</mo> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mn>1</mn> <mo stretchy="false">−</mo> <msub> <mi>β</mi> <mn>1</mn> </msub> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> <mstyle mathvariant="bold"> <mi>G</mi> </mstyle> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </mrow> </semantics> </math> |
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mover accent="true"> <mi>s</mi> <mo stretchy="false">^</mo> </mover> <mo stretchy="false">=</mo> <mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <msub> <mi>β</mi> <mn>2</mn> </msub> <mi>s</mi> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> <mo stretchy="false">+</mo> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mn>1</mn> <mo stretchy="false">−</mo> <msub> <mi>β</mi> <mn>2</mn> </msub> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> <mstyle mathvariant="bold"> <msup> <mi>G</mi> <mn>2</mn> </msup> </mstyle> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </mrow> </semantics> </math> |
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mrow> <mstyle mathvariant="bold"> <mover accent="true"> <mi>W</mi> <mo stretchy="false">^</mo> </mover> </mstyle> <mo stretchy="false">=</mo> <mrow> <mstyle mathvariant="bold"> <mi>W</mi> </mstyle> <mo stretchy="false">−</mo> <mfrac> <mi>η</mi> <mrow> <msqrt> <mover accent="true"> <mi>s</mi> <mo stretchy="false">^</mo> </mover> </msqrt> <mo stretchy="false">+</mo> <mi>ε</mi> </mrow> </mfrac> </mrow> </mrow> <mstyle mathvariant="bold"> <mover accent="true"> <mi>v</mi> <mo stretchy="false">^</mo> </mover> </mstyle> </mrow> </semantics> </math> |
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mstyle mathvariant="bold"> <mi>G</mi> </mstyle> <mo stretchy="false">≝</mo> <mtext>Total gradient.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mstyle mathvariant="bold"> <mi>v</mi> </mstyle> <mo stretchy="false">≝</mo> <mtext>Previous momentum 1.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mstyle mathvariant="bold"> <mover accent="true"> <mi>v</mi> <mo stretchy="false">^</mo> </mover> </mstyle> <mo stretchy="false">≝</mo> <mtext>Next momentum 1.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>s</mi> <mo stretchy="false">≝</mo> <mtext>Previous momentum 2.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mover accent="true"> <mi>s</mi> <mo stretchy="false">^</mo> </mover> <mo stretchy="false">≝</mo> <mtext>Next momentum 2.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mrow> <msub> <mi>β</mi> <mn>1</mn> </msub> <mo stretchy="false">≝</mo> <mtext>Rate of momentum 1.</mtext> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <msub> <mi>β</mi> <mn>1</mn> </msub> <mo stretchy="false">=</mo> <mn>0.9</mn> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mrow> <msub> <mi>β</mi> <mn>2</mn> </msub> <mo stretchy="false">≝</mo> <mtext>Rate of momentum 2.</mtext> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <msub> <mi>β</mi> <mn>2</mn> </msub> <mo stretchy="false">=</mo> <mn>0.999</mn> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mstyle mathvariant="bold"> <mi>W</mi> </mstyle> <mo stretchy="false">≝</mo> <mtext>Previous weights.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mstyle mathvariant="bold"> <mover accent="true"> <mi>W</mi> <mo stretchy="false">^</mo> </mover> </mstyle> <mo stretchy="false">≝</mo> <mtext>Next weights.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>η</mi> <mo stretchy="false">≝</mo> <mtext>Learning rate.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>ε</mi> <mo stretchy="false">≝</mo> <mtext>Machine epsilon.</mtext> </mrow> </mtd> </mtr> </mtable> </semantics> </math> |
    ///
    /// - `rate` : Learning rate.
    pub fn update(&mut self, rate: f32) {
        self.next_momentum_1();
        self.next_momentum_2();

        self.total_grad.copy_from(&self.momentum_1);

        // calc delta weights.
        for i in 0..OUT {
            debug_assert!(self.momentum_2.get(i).is_some());
            debug_assert!(self.total_grad.input_weights().get(i).is_some());
            debug_assert!(self.total_grad.bias().get(i).is_some());

            unsafe {
                let rate_2 = rate
                    / (sqrt(*self.momentum_2.get_unchecked(i)) + f32::EPSILON);

                *self.total_grad.bias_mut().get_unchecked_mut(i) *= rate_2;

                self.total_grad
                    .input_weights_mut()
                    .get_unchecked_mut(i)
                    .iter_mut()
                    .for_each(|g| {*g *= rate_2;});

                if let Some(state_weights) =
                    self.total_grad.state_weights_mut()
                {
                    debug_assert!(state_weights.get(i).is_some());

                    state_weights
                        .get_unchecked_mut(i)
                        .iter_mut()
                        .for_each(|g| {*g *= rate_2;});
                }
            }
        }

        // updates weights.
        self.layer.weights.iter_mut().zip(self.total_grad.iter()).for_each(
            |(w, g)| {*w -= *g;}
        );

        self.total_grad.clear();
    }

    #[inline]
    fn next_momentum_1(&mut self) {
        self.momentum_1.iter_mut().zip(self.total_grad.iter()).for_each(
            |(mom, grad)| {
                *mom = (BETA_1 * *mom) + (BETA_INV_1 * *grad);
            }
        );
    }

    #[inline]
    fn next_momentum_2(&mut self) {
        for i in 0..OUT {
            debug_assert!(self.momentum_2.get(i).is_some());
            debug_assert!(self.total_grad.input_weights().get(i).is_some());
            debug_assert!(self.total_grad.bias().get(i).is_some());

            let mut dot_product: f32 = 0.0;

            unsafe {
                let bias = *self.total_grad.bias().get_unchecked(i);
                dot_product += bias * bias;

                self.total_grad
                    .input_weights()
                    .get_unchecked(i)
                    .iter()
                    .for_each(|val| {dot_product += *val * *val;});

                if let Some(state_weights) = self.total_grad.state_weights() {
                    debug_assert!(state_weights.get(i).is_some());
                    state_weights
                        .get_unchecked(i)
                        .iter()
                        .for_each(|val| {dot_product += *val * *val;});
                }

                *self.momentum_2.get_unchecked_mut(i) =
                    (BETA_2 * *self.momentum_2.get_unchecked(i))
                    + (BETA_INV_2 * dot_product);
            }

        }
    }
}

/// AI for fixed length data.
///
/// - `OUT` : Dimension of output.
/// - `MIDDLE` : Dimension of hidden layer.
/// - `IN` : Dimension of input.
///
/// # Example
///
/// Letter classifier.
/// - If inputs Japanese letter, outputs '日'.
/// - If inputs English letter, outputs 'E'.
///
/// (1) Defines letter generator and ID.
///
/// ```ignore
/// extern crate chobitlibs;
/// 
/// use chobitlibs::chobit_ai::{
///     MathVec,
///     Activation,
///     ChobitAi,
///     ChobitMlAi,
///     MlAiCache
/// };
/// 
/// use chobitlibs::chobit_rand::ChobitRand;
/// 
/// fn japanese_letter(rng: &mut ChobitRand) -> char {
///     let letters = [
///         'あ', 'い', 'う', 'え', 'お',
///         'か', 'き', 'く', 'け', 'こ',
///         'さ', 'し', 'す', 'せ', 'そ'
///     ];
/// 
///     letters[(rng.next_u64() as usize) % letters.len()]
/// }
/// 
/// fn english_letter(rng: &mut ChobitRand) -> char {
///     let letters = [
///         'a', 'b', 'c', 'd', 'e',
///         'f', 'g', 'h', 'i', 'j',
///         'k', 'l', 'm', 'n', 'o'
///     ];
/// 
///     letters[(rng.next_u64() as usize) % letters.len()]
/// }
/// 
/// const JAPANESE_ID: char = '日';
/// const ENGLISH_ID: char = 'E';
/// ```
///
/// (2) Creates [`ChobitAi`] and randomises weights.
///
/// ```ignore
/// const OUT: usize = 32;
/// const MIDDLE: usize = 64;
/// const IN: usize = 32;
///
/// let mut rng = ChobitRand::new(b"ChobitAi Example");
///
/// let mut ai = ChobitAi::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
///
/// // Randomises weights.
/// ai.for_each_weight_mut(|weight| {
///     *weight = ((rng.next_f64() as f32) * 2.0) - 1.0;
/// });
///
/// let mut input = MathVec::<IN>::new();
/// let mut output = MathVec::<OUT>::new();
/// ```
///
/// (3) Wraps AI with [`ChobitMlAi`] for machine learning.
///
/// ```ignore
/// let mut ai = ChobitMlAi::<OUT, MIDDLE, IN>::new(ai);
/// let mut cache = MlAiCache::<OUT, MIDDLE, IN>::new();
///
/// let mut input_error = MathVec::<IN>::new();
/// let mut output_error = MathVec::<OUT>::new();
/// ```
///
/// (4) Machine learning.
///
/// ```ignore
/// const EPOCH: usize = 1000;
/// const BATCH_SIZE: usize = 100;
/// const RATE: f32 = 0.01;
///
/// for _ in 0..EPOCH {
///     for _ in 0..BATCH_SIZE {
///         //--- Learns Japanese ---//
///         input.load_u32_label(japanese_letter(&mut rng) as u32);
///         output.load_u32_label(JAPANESE_ID as u32);
///
///         // Writes cache.
///         ai.ready(&input, &mut cache);
///
///         // Calculates error.
///         cache.calc_output_error(&output, &mut output_error);
///
///         // Studies.
///         ai.study(&output_error, &cache, &mut input_error);
///
///         //--- Learns English ---//
///         input.load_u32_label(english_letter(&mut rng) as u32);
///         output.load_u32_label(ENGLISH_ID as u32);
///
///         // Writes cache.
///         ai.ready(&input, &mut cache);
///
///         // Calculates error.
///         cache.calc_output_error(&output, &mut output_error);
///
///         // Studies.
///         ai.study(&output_error, &cache, &mut input_error);
///     }
///
///     // Updates weights.
///     ai.update(RATE);
/// }
/// ```
///
/// (5) Tests AI.
///
/// ```ignore
/// // Unwrap AI.
/// let ai = ai.drop();
///
/// let mut tmpbuf = MathVec::<MIDDLE>::new();
///
/// // Tests Japanese.
/// for _ in 0..10 {
///     input.load_u32_label(japanese_letter(&mut rng) as u32);
///
///     ai.calc(&input, &mut output, &mut tmpbuf);
///
///     assert_eq!(output.to_u32_label(), JAPANESE_ID as u32);
/// }
///
/// // Tests English.
/// for _ in 0..10 {
///     input.load_u32_label(english_letter(&mut rng) as u32);
///
///     ai.calc(&input, &mut output, &mut tmpbuf);
///
///     assert_eq!(output.to_u32_label(), ENGLISH_ID as u32);
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ChobitAi<const OUT: usize, const MIDDLE: usize, const IN: usize> {
    middle_layer: Layer<MIDDLE, IN>,
    output_layer: Layer<OUT, MIDDLE>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> ChobitAi<OUT, MIDDLE, IN> {
    /// Creates ChobitAi.
    ///
    /// - `activation` : Activation function for output layer.
    /// - _Return_ : ChobitAi.
    #[inline]
    pub fn new(activation: Activation) -> Self {
        Self {
            middle_layer: Layer::<MIDDLE, IN>::new(Activation::ReLU, false),
            output_layer: Layer::<OUT, MIDDLE>::new(activation, false)
        }
    }

    /// Gets immutable middle layer.
    ///
    /// - _Return_ : Middle layer.
    #[inline]
    pub fn middle_layer(&self) -> &Layer<MIDDLE, IN> {&self.middle_layer}

    /// Gets mutable middle layer.
    ///
    /// - _Return_ : Middle layer.
    #[inline]
    pub fn middle_layer_mut(&mut self) -> &mut Layer<MIDDLE, IN> {
        &mut self.middle_layer
    }

    /// Gets immutable output layer.
    ///
    /// - _Return_ : Output layer.
    #[inline]
    pub fn output_layer(&self) -> &Layer<OUT, MIDDLE> {&self.output_layer}

    /// Gets mutable output layer.
    ///
    /// - _Return_ : Output layer.
    #[inline]
    pub fn output_layer_mut(&mut self) -> &mut Layer<OUT, MIDDLE> {
        &mut self.output_layer
    }

    /// Accesses each immutable weight with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_weight<F>(&self, mut f: F) where F: FnMut(&f32) {
        self.middle_layer.weights().iter().for_each(|val| {f(val)});
        self.output_layer.weights().iter().for_each(|val| {f(val)});
    }

    /// Accesses each mutable weight with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_weight_mut<F>(
        &mut self,
        mut f: F
    ) where F: FnMut(&mut f32) {
        self.middle_layer.mut_weights().iter_mut().for_each(|val| {f(val)});
        self.output_layer.mut_weights().iter_mut().for_each(|val| {f(val)});
    }

    /// Calculates
    ///
    /// - `input` : Input.
    /// - `output` : Buffer for output.
    /// - `tmpbuf` : Temporary buffer for this function to work.
    #[inline]
    pub fn calc(
        &self,
        input: &MathVec<IN>,
        output: &mut MathVec<OUT>,
        tmpbuf: &mut MathVec<MIDDLE>
    ) {
        self.middle_layer.calc(input, None, tmpbuf);
        self.output_layer.calc(tmpbuf, None, output);
    }
}

/// Cache for [`ChobitMlAi`].
///
/// - `OUT` : `OUT` of [`ChobitMlAi`].
/// - `MIDDLE` : `MIDDLE` of [`ChobitMlAi`].
/// - `IN` : `IN` of [`ChobitMlAi`].
#[derive(Debug, Clone, PartialEq)]
pub struct MlAiCache<const OUT: usize, const MIDDLE: usize, const IN: usize> {
    middle_cache: MlCache<MIDDLE, IN>,
    output_cache: MlCache<OUT, MIDDLE>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> MlAiCache<OUT, MIDDLE, IN> {
    /// Creates MlAiCache.
    ///
    /// - _Return_ : MlAiCache.
    #[inline]
    pub fn new() -> Self {
        Self {
            middle_cache: MlCache::<MIDDLE, IN>::new(),
            output_cache: MlCache::<OUT, MIDDLE>::new()
        }
    }

    /// Calculates output error.
    ///
    /// | Formula |
    /// |:-:|
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mi>e</mi> <mo stretchy="false">=</mo> <mrow> <mi>o</mi> <mo stretchy="false">−</mo> <mi>t</mi> </mrow> </mrow> </semantics> </math> |
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mi>e</mi> <mo stretchy="false">≝</mo> <mtext>Error.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>o</mi> <mo stretchy="false">≝</mo> <mtext>Actual output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>t</mi> <mo stretchy="false">≝</mo> <mtext>Correct output.</mtext> </mrow> </mtd> </mtr> </mtable> </semantics> </math> |
    ///
    /// - `train_out` : Correct output.
    /// - `output_error` : Buffer for output error.
    #[inline]
    pub fn calc_output_error(
        &self,
        train_out: &MathVec<OUT>,
        error: &mut MathVec<OUT>
    ) {
        error.copy_from(&self.output_cache.output);
        *error -= train_out;
    }

    /// Gets cache for middle layer.
    ///
    /// - _Return_ : Cache for middle layer.
    #[inline]
    pub fn middle_cache(&self) -> &MlCache<MIDDLE, IN> {&self.middle_cache}

    /// Gets cache for output layer.
    ///
    /// - _Return_ : Cache for output layer.
    #[inline]
    pub fn output_cache(&self) -> &MlCache<OUT, MIDDLE> {&self.output_cache}

    /// Gets output.
    ///
    /// - _Return_ : Output.
    #[inline]
    pub fn output(&self) -> &MathVec<OUT> {&self.output_cache.output}
}

/// Wrapper of [`ChobitAi`] for machine learning.
///
/// See [`ChobitAi`] for details.
///
/// - `OUT` : Dimension of output.
/// - `MIDDLE` : Dimension of hidden layer.
/// - `IN` : Dimension of input.
#[derive(Debug, Clone, PartialEq)]
pub struct ChobitMlAi<const OUT: usize, const MIDDLE: usize, const IN: usize> {
    middle_layer: MlLayer<MIDDLE, IN>,
    output_layer: MlLayer<OUT, MIDDLE>,

    middle_error: MathVec<MIDDLE>,
    output_error: MathVec<OUT>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> ChobitMlAi<OUT, MIDDLE, IN> {
    /// Creates ChobitMlAi.
    ///
    /// - `ai` : Base [`ChobitAi`].
    /// - _Return_ : ChobitMlAi.
    #[inline]
    pub fn new(ai: ChobitAi<OUT, MIDDLE, IN>) -> Self {
        let ChobitAi::<OUT, MIDDLE, IN> {middle_layer, output_layer} = ai;

        Self {
            middle_layer: MlLayer::<MIDDLE, IN>::new(middle_layer),
            output_layer: MlLayer::<OUT, MIDDLE>::new(output_layer),

            middle_error: MathVec::<MIDDLE>::new(),
            output_error: MathVec::<OUT>::new(),
        }
    }

    /// Drops base [`ChobitAi`].
    ///
    /// - _Return_ : [`ChobitAi`].
    #[inline]
    pub fn drop(self) -> ChobitAi<OUT, MIDDLE, IN> {
        let Self {middle_layer, output_layer, ..} = self;

        ChobitAi::<OUT, MIDDLE, IN> {
            middle_layer: middle_layer.drop(),
            output_layer: output_layer.drop()
        }
    }

    /// Clears internal data for study.
    #[inline]
    pub fn clear_study_data(&mut self) {
        self.middle_layer.clear_study_data();
        self.output_layer.clear_study_data();
    }

    /// Writes information on [`MlAiCache`] for [`study()`](Self::study()).
    ///
    /// - `input` : Input.
    /// - `cache` : Cache.
    #[inline]
    pub fn ready(
        &self,
        input: &MathVec<IN>,
        cache: &mut MlAiCache<OUT, MIDDLE, IN>
    ) {
        self.middle_layer.ready(input, None, &mut cache.middle_cache);

        self.output_layer.ready(
            &cache.middle_cache.output,
            None,
            &mut cache.output_cache
        );
    }

    /// Accumulates gradient.
    ///
    /// - `output_error` : Backpropagated ouput error.
    /// - `cache` : Cache.
    /// - `input_error` : Error to backpropagate for previous output error.
    #[inline]
    pub fn study(
        &mut self,
        output_error: &MathVec<OUT>,
        cache: &MlAiCache<OUT, MIDDLE, IN>,
        input_error: &mut MathVec<IN>
    ) {
        self.output_layer.study(
            output_error,
            None,
            &cache.output_cache,
            &mut self.middle_error,
            None
        );

        self.middle_layer.study(
            &self.middle_error,
            None,
            &cache.middle_cache,
            input_error,
            None
        );
    }

    /// Updates weights.
    ///
    /// - `rate` : Learning rate.
    #[inline]
    pub fn update(&mut self, rate: f32) {
        self.middle_layer.update(rate);
        self.output_layer.update(rate);
    }

    /// Accesses each immutable total gradient with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_total_grad<F>(&self, mut f: F) where F: FnMut(&f32) {
        self.middle_layer.total_grad.iter().for_each(|val| {f(val)});
        self.output_layer.total_grad.iter().for_each(|val| {f(val)});
    }

    /// Accesses each mutable total gradient with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_total_grad_mut<F>(
        &mut self,
        mut f: F
    ) where F: FnMut(&mut f32) {
        self.middle_layer.total_grad.iter_mut().for_each(|val| {f(val)});
        self.output_layer.total_grad.iter_mut().for_each(|val| {f(val)});
    }
}

/// [Peephole LSTM](https://en.wikipedia.org/wiki/Long_short-term_memory#Peephole_Lstm)
///
/// | Formula |
/// |:-:|
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mrow> <msub> <mi>m</mi> <mi>i</mi> </msub> <mo stretchy="false">=</mo> <mi>tanh</mi> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="true">(</mo> <mrow> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>j</mi> </munder> <msubsup> <mi>W</mi> <mi mathvariant="italic">ij</mi> <mi>m</mi> </msubsup> </mrow> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">+</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>k</mi> </munder> <msubsup> <mi>U</mi> <mi mathvariant="italic">ik</mi> <mi>m</mi> </msubsup> </mrow> </mrow> <mrow> <mover accent="true"> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo>¯ </mo> </mover> <mo stretchy="false">+</mo> <msubsup> <mi>b</mi> <mi>i</mi> <mi>m</mi> </msubsup> </mrow> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="true">)</mo> </mrow> </mrow> </semantics> </math> |
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mrow> <msub> <mi>f</mi> <mi>i</mi> </msub> <mo stretchy="false">=</mo> <mi>&sigma;</mi> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="true">(</mo> <mrow> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>j</mi> </munder> <msubsup> <mi>W</mi> <mi mathvariant="italic">ij</mi> <mi>f</mi> </msubsup> </mrow> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">+</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>k</mi> </munder> <msubsup> <mi>U</mi> <mi mathvariant="italic">ik</mi> <mi>f</mi> </msubsup> </mrow> </mrow> <mrow> <mover accent="true"> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo>¯ </mo> </mover> <mo stretchy="false">+</mo> <msubsup> <mi>b</mi> <mi>i</mi> <mi>f</mi> </msubsup> </mrow> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="true">)</mo> </mrow> </mrow> </semantics> </math> |
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mrow> <msub> <mi>i</mi> <mi>i</mi> </msub> <mo stretchy="false">=</mo> <mi>&sigma;</mi> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="true">(</mo> <mrow> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>j</mi> </munder> <msubsup> <mi>W</mi> <mi mathvariant="italic">ij</mi> <mi>i</mi> </msubsup> </mrow> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">+</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>k</mi> </munder> <msubsup> <mi>U</mi> <mi mathvariant="italic">ik</mi> <mi>i</mi> </msubsup> </mrow> </mrow> <mrow> <mover accent="true"> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo>¯ </mo> </mover> <mo stretchy="false">+</mo> <msubsup> <mi>b</mi> <mi>i</mi> <mi>i</mi> </msubsup> </mrow> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="true">)</mo> </mrow> </mrow> </semantics> </math> |
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mrow> <msub> <mi>o</mi> <mi>i</mi> </msub> <mo stretchy="false">=</mo> <mi>&sigma;</mi> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="true">(</mo> <mrow> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>j</mi> </munder> <msubsup> <mi>W</mi> <mi mathvariant="italic">ij</mi> <mi>o</mi> </msubsup> </mrow> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">+</mo> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>k</mi> </munder> <msubsup> <mi>U</mi> <mi mathvariant="italic">ik</mi> <mi>o</mi> </msubsup> </mrow> </mrow> <mrow> <mover accent="true"> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo>¯ </mo> </mover> <mo stretchy="false">+</mo> <msubsup> <mi>b</mi> <mi>i</mi> <mi>o</mi> </msubsup> </mrow> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="true">)</mo> </mrow> </mrow> </semantics> </math> |
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mrow> <msub> <mi>s</mi> <mi>i</mi> </msub> <mo stretchy="false">=</mo> <msub> <mi>f</mi> <mi>i</mi> </msub> </mrow> <mrow> <mrow> <mo fence="true" form="prefix" stretchy="true">(</mo> <mrow> <mrow> <mrow> <munder> <mo stretchy="false">∑</mo> <mi>k</mi> </munder> <msub> <mi>δ</mi> <mi mathvariant="italic">ik</mi> </msub> </mrow> <mover accent="true"> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo>¯ </mo> </mover> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="true">)</mo> </mrow> <mo stretchy="false">+</mo> <msub> <mi>i</mi> <mi>i</mi> </msub> </mrow> <msub> <mi>m</mi> <mi>i</mi> </msub> </mrow> </semantics> </math> |
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mrow> <msub> <mi>y</mi> <mi>i</mi> </msub> <mo stretchy="false">=</mo> <msub> <mi>o</mi> <mi>i</mi> </msub> </mrow> <mi>tanh</mi> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <msub> <mi>s</mi> <mi>i</mi> </msub> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </semantics> </math> |
/// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mi>i</mi> <mo stretchy="false">≝</mo> <mtext>Index of output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>j</mi> <mo stretchy="false">≝</mo> <mtext>Index of input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mrow> <mi>k</mi> <mo stretchy="false">≝</mo> <mtext>Index of state.</mtext> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mtext>dim</mtext> <mspace width="0.5em"/> <mrow> <mi>k</mi> <mo stretchy="false">=</mo> <mtext>dim</mtext> </mrow> <mspace width="0.5em"/> <mi>i</mi> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>δ</mi> <mi mathvariant="italic">ik</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Kronecker&apos;s delta</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mrow> <mi>tanh</mi> <mo stretchy="false">≝</mo> <mtext>Hyperbolic tangent.</mtext> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mi>tanh</mi> <mo stretchy="false">→</mo> <mtext>soft sign</mtext> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mrow> <mi>&sigma;</mi> <mo stretchy="false">≝</mo> <mtext>Sigmoid function.</mtext> </mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mi>&sigma;</mi> <mo stretchy="false">→</mo> <mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mrow> <mrow> <mo fence="true" form="prefix" stretchy="false">(</mo> <mrow> <mtext>soft sign</mtext> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> <mo stretchy="false">+</mo> <mn>1</mn> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> <mo stretchy="false">÷</mo> <mn>2</mn> </mrow> </mrow> </mrow> <mo fence="true" form="postfix" stretchy="false">)</mo> </mrow> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>x</mi> <mi>j</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mover accent="true"> <msub> <mi>s</mi> <mi>k</mi> </msub> <mo>¯ </mo> </mover> <mo stretchy="false">≝</mo> <mtext>Previous state.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>m</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Output of main layer.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msubsup> <mi>W</mi> <mi mathvariant="italic">ij</mi> <mi>m</mi> </msubsup> <mo stretchy="false">≝</mo> <mtext>Weights of main layer for input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msubsup> <mi>U</mi> <mi mathvariant="italic">ik</mi> <mi>m</mi> </msubsup> <mo stretchy="false">≝</mo> <mtext>Weights of main layer for state.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msubsup> <mi>b</mi> <mi>i</mi> <mi>m</mi> </msubsup> <mo stretchy="false">≝</mo> <mtext>Bias of main layer.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>f</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Output of forget gate.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msubsup> <mi>W</mi> <mi mathvariant="italic">ij</mi> <mi>f</mi> </msubsup> <mo stretchy="false">≝</mo> <mtext>Weights of forget gate for input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msubsup> <mi>U</mi> <mi mathvariant="italic">ik</mi> <mi>f</mi> </msubsup> <mo stretchy="false">≝</mo> <mtext>Weights of forget gate for state.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msubsup> <mi>b</mi> <mi>i</mi> <mi>f</mi> </msubsup> <mo stretchy="false">≝</mo> <mtext>Bias of forget gate.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>i</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Output of input gate.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msubsup> <mi>W</mi> <mi mathvariant="italic">ij</mi> <mi>i</mi> </msubsup> <mo stretchy="false">≝</mo> <mtext>Weights of input gate for input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msubsup> <mi>U</mi> <mi mathvariant="italic">ik</mi> <mi>i</mi> </msubsup> <mo stretchy="false">≝</mo> <mtext>Weights of input gate for state.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msubsup> <mi>b</mi> <mi>i</mi> <mi>i</mi> </msubsup> <mo stretchy="false">≝</mo> <mtext>Bias of input gate.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msub> <mi>o</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Output of output gate.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msubsup> <mi>W</mi> <mi mathvariant="italic">ij</mi> <mi>o</mi> </msubsup> <mo stretchy="false">≝</mo> <mtext>Weights of output gate for input.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msubsup> <mi>U</mi> <mi mathvariant="italic">ik</mi> <mi>o</mi> </msubsup> <mo stretchy="false">≝</mo> <mtext>Weights of output gate for state.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <msubsup> <mi>b</mi> <mi>i</mi> <mi>o</mi> </msubsup> <mo stretchy="false">≝</mo> <mtext>Bias of output gate.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mrow> <msub> <mi>y</mi> <mi>i</mi> </msub> <mo stretchy="false">≝</mo> <mtext>Output</mtext> </mrow> <mi>.</mi> </mrow> </mtd> </mtr> </mtable> </semantics> </math> |
///
/// - `OUT` : Dimension of output.
/// - `IN` : Dimension of input.
#[derive(Debug, Clone, PartialEq)]
pub struct Lstm<const OUT: usize, const IN: usize> {
    main_layer: Layer<OUT, IN>,

    f_gate: Layer<OUT, IN>,
    i_gate: Layer<OUT, IN>,
    o_gate: Layer<OUT, IN>,

    tanh: Activation
}

impl<const OUT: usize, const IN: usize> Lstm<OUT, IN> {
    /// Creates Lstm.
    ///
    /// - _Return_ : Lstm.
    #[inline]
    pub fn new() -> Self {
        Self {
            main_layer: Layer::<OUT, IN>::new(Activation::SoftSign, true),

            f_gate: Layer::<OUT, IN>::new(Activation::Sigmoid, true),
            i_gate: Layer::<OUT, IN>::new(Activation::Sigmoid, true),
            o_gate: Layer::<OUT, IN>::new(Activation::Sigmoid, true),

            tanh: Activation::SoftSign
        }
    }

    /// Gets immutable main layer.
    ///
    /// - _Return_ : Main layer.
    #[inline]
    pub fn main_layer(&self) -> &Layer<OUT, IN> {&self.main_layer}

    /// Gets mutable main layer.
    ///
    /// - _Return_ : Main layer.
    #[inline]
    pub fn main_layer_mut(&mut self) -> &mut Layer<OUT, IN> {
        &mut self.main_layer
    }

    /// Gets immutable forget gate.
    ///
    /// - _Return_ : Forget gate.
    #[inline]
    pub fn f_gate(&self) -> &Layer<OUT, IN> {&self.f_gate}

    /// Gets mutable forget gate.
    ///
    /// - _Return_ : Forget gate.
    #[inline]
    pub fn f_gate_mut(&mut self) -> &mut Layer<OUT, IN> {&mut self.f_gate}

    /// Gets immutable input gate.
    ///
    /// - _Return_ : Input gate.
    #[inline]
    pub fn i_gate(&self) -> &Layer<OUT, IN> {&self.i_gate}

    /// Gets mutable input gate.
    ///
    /// - _Return_ : Input gate.
    #[inline]
    pub fn i_gate_mut(&mut self) -> &mut Layer<OUT, IN> {&mut self.i_gate}

    /// Gets immutable output gate.
    ///
    /// - _Return_ : Output gate.
    #[inline]
    pub fn o_gate(&self) -> &Layer<OUT, IN> {&self.o_gate}

    /// Gets mutable output gate.
    ///
    /// - _Return_ : Output gate.
    #[inline]
    pub fn o_gate_mut(&mut self) -> &mut Layer<OUT, IN> {&mut self.o_gate}

    /// Calculates only state.
    ///
    /// - `input` : Input.
    /// - `prev_state` : Previous state.
    /// - `next_state` : Buffer for next state.
    /// - `tmpbuf` : Temporary buffer for this function to work.
    pub fn calc_state(
        &self,
        input: &MathVec<IN>,
        prev_state: &MathVec<OUT>,
        next_state: &mut MathVec<OUT>,
        tmpbuf: &mut MathVec<OUT>
    ) {
        // state = (f_gate * prev_state) + (i_gate * main_layer);
        self.main_layer.calc(input, Some(prev_state), next_state);
        self.i_gate.calc(input, Some(prev_state), tmpbuf);
        next_state.pointwise_mul_assign(tmpbuf);

        self.f_gate.calc(input, Some(prev_state), tmpbuf);
        tmpbuf.pointwise_mul_assign(prev_state);

        *next_state += tmpbuf;
    }

    /// Calculates state and output.
    ///
    /// - `input` : Input.
    /// - `prev_state` : Previous state.
    /// - `output` : Buffer for output.
    /// - `next_state` : Buffer for next state.
    /// - `tmpbuf` : Temporary buffer for this function to work.
    pub fn calc(
        &self,
        input: &MathVec<IN>,
        prev_state: &MathVec<OUT>,
        output: &mut MathVec<OUT>,
        next_state: &mut MathVec<OUT>,
        tmpbuf: &mut MathVec<OUT>
    ) {
        self.calc_state(input, prev_state, next_state, tmpbuf);

        // output = o_gate * tanh(state)
        self.o_gate.calc(input, Some(prev_state), output);

        output.as_mut_array().iter_mut().zip(
            next_state.as_array().iter()
        ).for_each(|(output_one, next_s)| {
            *output_one *= self.tanh.activate(*next_s);
        });
    }

    /// Accesses each immutable weight with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_weight<F>(&self, mut f: F) where F: FnMut(&f32) {
        self.main_layer.weights().iter().for_each(|val| {f(val)});
        self.f_gate.weights().iter().for_each(|val| {f(val)});
        self.i_gate.weights().iter().for_each(|val| {f(val)});
        self.o_gate.weights().iter().for_each(|val| {f(val)});
    }

    /// Accesses each mutable weight with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_weight_mut<F>(
        &mut self,
        mut f: F
    ) where F: FnMut(&mut f32) {
        self.main_layer.mut_weights().iter_mut().for_each(|val| {f(val)});
        self.f_gate.mut_weights().iter_mut().for_each(|val| {f(val)});
        self.i_gate.mut_weights().iter_mut().for_each(|val| {f(val)});
        self.o_gate.mut_weights().iter_mut().for_each(|val| {f(val)});
    }
}

/// Cache for state error of [`MlLstm`].
///
/// - `OUT` : Output of [`MlLstm`].
/// - `IN` : Input of [`MlLstm`].
#[derive(Debug, Clone, PartialEq)]
pub struct MlLstmStateCache<const OUT: usize, const IN: usize> {
    input: MathVec<IN>,
    prev_state: MathVec<OUT>,

    main_layer_cache: MlCache<OUT, IN>,

    f_gate_cache: MlCache<OUT, IN>,
    i_gate_cache: MlCache<OUT, IN>,

    state: MathVec<OUT>
}

impl<const OUT: usize, const IN: usize> MlLstmStateCache<OUT, IN> {
    /// Creates MlLstmStateCache.
    ///
    /// - _Return_ : MlLstmStateCache.
    #[inline]
    pub fn new() -> Self {
        Self {
            input: MathVec::<IN>::new(),
            prev_state: MathVec::<OUT>::new(),

            main_layer_cache: MlCache::<OUT, IN>::new(),
            f_gate_cache: MlCache::<OUT, IN>::new(),
            i_gate_cache: MlCache::<OUT, IN>::new(),

            state: MathVec::<OUT>::new()
        }
    }

    /// Gets input.
    ///
    /// - _Return_ : Input.
    #[inline]
    pub fn input(&self) -> &MathVec<IN> {&self.input}

    /// Gets previous state.
    ///
    /// - _Return_ : Previous state.
    #[inline]
    pub fn prev_state(&self) -> &MathVec<OUT> {&self.prev_state}

    /// Gets cache of main layer.
    ///
    /// - _Return_ : Cache of main layer.
    #[inline]
    pub fn main_layer_cache(&self) -> &MlCache<OUT, IN> {
        &self.main_layer_cache
    }

    /// Gets cache of forget gate.
    ///
    /// - _Return_ : Cache of forget gate.
    #[inline]
    pub fn f_gate_cache(&self) -> &MlCache<OUT, IN> {&self.f_gate_cache}

    /// Gets cache of input gate.
    ///
    /// - _Return_ : Cache of input gate.
    #[inline]
    pub fn i_gate_cache(&self) -> &MlCache<OUT, IN> {&self.i_gate_cache}

    /// Gets state.
    ///
    /// - _Return_ : State.
    #[inline]
    pub fn state(&self) -> &MathVec<OUT> {&self.state}
}

/// Cache for output error of [`MlLstm`].
///
/// - `OUT` : Output of [`MlLstm`].
/// - `IN` : Input of [`MlLstm`].
#[derive(Debug, Clone, PartialEq)]
pub struct MlLstmOutputCache<const OUT: usize, const IN: usize> {
    o_gate_cache: MlCache<OUT, IN>,

    tanh_s: MathVec<OUT>,
    d_tanh_s: MathVec<OUT>,

    output: MathVec<OUT>
}

impl<const OUT: usize, const IN: usize> MlLstmOutputCache<OUT, IN> {
    /// Creates MlLstmOutputCache.
    ///
    /// - _Return_ : MlLstmOutputCache.
    #[inline]
    pub fn new() -> Self {
        Self {
            o_gate_cache: MlCache::<OUT, IN>::new(),

            tanh_s: MathVec::<OUT>::new(),
            d_tanh_s: MathVec::<OUT>::new(),

            output: MathVec::<OUT>::new()
        }
    }

    /// Calculates output error.
    ///
    /// | Formula |
    /// |:-:|
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mi>e</mi> <mo stretchy="false">=</mo> <mrow> <mi>o</mi> <mo stretchy="false">−</mo> <mi>t</mi> </mrow> </mrow> </semantics> </math> |
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mi>e</mi> <mo stretchy="false">≝</mo> <mtext>Error.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>o</mi> <mo stretchy="false">≝</mo> <mtext>Actual output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>t</mi> <mo stretchy="false">≝</mo> <mtext>Correct output.</mtext> </mrow> </mtd> </mtr> </mtable> </semantics> </math> |
    ///
    /// - `train_out` : Correct output.
    /// - `output_error` : Buffer for output error.
    #[inline]
    pub fn calc_output_error(
        &self,
        train_out: &MathVec<OUT>,
        output_error: &mut MathVec<OUT>
    ) {
        output_error.copy_from(&self.output);
        *output_error -= train_out;
    }

    /// Gets cache of output gate.
    ///
    /// - _Return_ : Cache of output gate.
    #[inline]
    pub fn o_gate_cache(&self) -> &MlCache<OUT, IN> {&self.o_gate_cache}

    /// Gets output of tanh(state).
    ///
    /// - _Return_ : Output of tanh(state).
    #[inline]
    pub fn tanh_s(&self) -> &MathVec<OUT> {&self.tanh_s}

    /// Gets derivative of tanh(state).
    ///
    /// - _Return_ : Derivative of tanh(state).
    #[inline]
    pub fn d_tanh_s(&self) -> &MathVec<OUT> {&self.d_tanh_s}

    /// Gets output.
    ///
    /// - _Return_ : Output.
    #[inline]
    pub fn output(&self) -> &MathVec<OUT> {&self.output}
}

/// LSTM for machine learning.
///
/// See [`Lstm`] for details.
///
/// - `OUT` : Dimension of output.
/// - `IN` : Dimension of input.
#[derive(Debug, Clone, PartialEq)]
pub struct MlLstm<const OUT: usize, const IN: usize> {
    main_layer: MlLayer<OUT, IN>,
    f_gate: MlLayer<OUT, IN>,
    i_gate: MlLayer<OUT, IN>,
    o_gate: MlLayer<OUT, IN>,
    tanh: Activation,

    input_error_main_by_output_error: MathVec<IN>,
    input_error_main_by_state_error: MathVec<IN>,
    input_error_f_by_output_error: MathVec<IN>,
    input_error_f_by_state_error: MathVec<IN>,
    input_error_i_by_output_error: MathVec<IN>,
    input_error_i_by_state_error: MathVec<IN>,
    input_error_o_by_output_error: MathVec<IN>,
    input_error_o_by_state_error: MathVec<IN>,

    prev_state_error_main_by_output_error: MathVec<OUT>,
    prev_state_error_main_by_state_error: MathVec<OUT>,
    prev_state_error_f_by_output_error: MathVec<OUT>,
    prev_state_error_f_by_state_error: MathVec<OUT>,
    prev_state_error_i_by_output_error: MathVec<OUT>,
    prev_state_error_i_by_state_error: MathVec<OUT>,
    prev_state_error_o_by_output_error: MathVec<OUT>,
    prev_state_error_o_by_state_error: MathVec<OUT>,

    tmp_error: MathVec<OUT>
}

impl<const OUT: usize, const IN: usize> MlLstm<OUT, IN> {
    /// Creates MlLstm.
    ///
    /// - `lstm` : Base [`Lstm`].
    /// - _Return_ : MlLstm.
    #[inline]
    pub fn new(lstm: Lstm<OUT, IN>) -> Self {
        let Lstm::<OUT, IN> {main_layer, f_gate, i_gate, o_gate, tanh} = lstm;

        Self {
            main_layer: MlLayer::<OUT, IN>::new(main_layer),
            f_gate: MlLayer::<OUT, IN>::new(f_gate),
            i_gate: MlLayer::<OUT, IN>::new(i_gate),
            o_gate: MlLayer::<OUT, IN>::new(o_gate),
            tanh: tanh,

            input_error_main_by_output_error: MathVec::<IN>::new(),
            input_error_main_by_state_error: MathVec::<IN>::new(),
            input_error_f_by_output_error: MathVec::<IN>::new(),
            input_error_f_by_state_error: MathVec::<IN>::new(),
            input_error_i_by_output_error: MathVec::<IN>::new(),
            input_error_i_by_state_error: MathVec::<IN>::new(),
            input_error_o_by_output_error: MathVec::<IN>::new(),
            input_error_o_by_state_error: MathVec::<IN>::new(),

            prev_state_error_main_by_output_error: MathVec::<OUT>::new(),
            prev_state_error_main_by_state_error: MathVec::<OUT>::new(),
            prev_state_error_f_by_output_error: MathVec::<OUT>::new(),
            prev_state_error_f_by_state_error: MathVec::<OUT>::new(),
            prev_state_error_i_by_output_error: MathVec::<OUT>::new(),
            prev_state_error_i_by_state_error: MathVec::<OUT>::new(),
            prev_state_error_o_by_output_error: MathVec::<OUT>::new(),
            prev_state_error_o_by_state_error: MathVec::<OUT>::new(),

            tmp_error: MathVec::<OUT>::new()
        }
    }

    /// Drops base [`Lstm`].
    ///
    /// - _Return_ : [`Lstm`].
    #[inline]
    pub fn drop(self) -> Lstm<OUT, IN> {
        let Self {main_layer, f_gate, i_gate, o_gate, tanh, ..} = self;

        Lstm::<OUT, IN> {
            main_layer: main_layer.drop(),

            f_gate: f_gate.drop(),
            i_gate: i_gate.drop(),
            o_gate: o_gate.drop(),

            tanh: tanh
        }
    }

    /// Clears internal data for study.
    #[inline]
    pub fn clear_study_data(&mut self) {
        self.main_layer.clear_study_data();
        self.f_gate.clear_study_data();
        self.i_gate.clear_study_data();
        self.o_gate.clear_study_data();
    }

    /// Writes information on [`MlLstmStateCache`] for [`study_state()`](Self::study_state()) or [`MlLstm::study()`].
    ///
    /// - `input` : Input.
    /// - `prev_state` : Previous state.
    /// - `cache` : Cache.
    pub fn ready_state_cache(
        &self,
        input: &MathVec<IN>,
        prev_state: &MathVec<OUT>,
        cache: &mut MlLstmStateCache<OUT, IN>
    ) {
        cache.input.copy_from(input);
        cache.prev_state.copy_from(prev_state);

        self.main_layer.ready(
            input,
            Some(prev_state),
            &mut cache.main_layer_cache
        );
        self.f_gate.ready(input, Some(prev_state), &mut cache.f_gate_cache);
        self.i_gate.ready(input, Some(prev_state), &mut cache.i_gate_cache);

        cache.state.as_mut_array().iter_mut().zip(
            prev_state.as_array().iter()
        ).zip(
            cache.f_gate_cache.output.as_array().iter()
        ).zip(
            cache.i_gate_cache.output.as_array().iter()
        ).zip(
            cache.main_layer_cache.output.as_array().iter()
        ).for_each(|((((state_one, p_state), f_out), i_out), main_out)| {
            *state_one = (*p_state * *f_out) + (*i_out * *main_out);
        });
    }

    /// Writes information on [`MlLstmOutputCache`] for [`study()`](Self::study()).
    ///
    /// - `last_state_cache` : Cache written at [`ready_state_cache()`](Self::ready_state_cache()) just before.
    /// - `cache` : Cache.
    pub fn ready_output_cache(
        &self,
        last_state_cache: &MlLstmStateCache<OUT, IN>,
        output_cache: &mut MlLstmOutputCache<OUT, IN>
    ) {
        self.o_gate.ready(
            &last_state_cache.input,
            Some(&last_state_cache.prev_state),
            &mut output_cache.o_gate_cache
        );

        last_state_cache.state.as_array().iter().zip(
            output_cache.tanh_s.as_mut_array().iter_mut()
        ).zip(
            output_cache.d_tanh_s.as_mut_array().iter_mut()
        ).zip(
            output_cache.output.as_mut_array().iter_mut()
        ).zip(
            output_cache.o_gate_cache.output.as_array().iter()
        ).for_each(
            |((((s, tanh_s_one), d_tanh_s_one), output_one), o_out)| {
                *tanh_s_one = self.tanh.activate(*s);
                *d_tanh_s_one = self.tanh.d_activate(*s);
                *output_one = *o_out * *tanh_s_one;
            }
        );
    }

    /// Accumulates gradient.
    ///
    /// - `state_error` : Backpropagated state error.
    /// - `cache` : Cache written at [`ready_state_cache()`](Self::ready_state_cache()).
    /// - `input_error` : Error to backpropagate for previous output error.
    /// - `prev_state_error` : Error to backpropagate previous state error.
    pub fn study_state(
        &mut self,
        state_error: &MathVec<OUT>,
        cache: &MlLstmStateCache<OUT, IN>,
        input_error: &mut MathVec<IN>,
        prev_state_error: &mut MathVec<OUT>
    ) {
        self.study_main_layer_with_state_error(state_error, cache);
        self.study_f_gate_with_state_error(state_error, cache);
        self.study_i_gate_with_state_error(state_error, cache);

        input_error.copy_from(&self.input_error_main_by_state_error);
        *input_error += &self.input_error_f_by_state_error;
        *input_error += &self.input_error_i_by_state_error;

        prev_state_error.copy_from(&self.prev_state_error_main_by_state_error);
        *prev_state_error += &self.prev_state_error_f_by_state_error;
        *prev_state_error += &self.prev_state_error_i_by_state_error;

        prev_state_error.as_mut_array().iter_mut().zip(
            state_error.as_array().iter()
        ).zip(
            cache.f_gate_cache.output.as_array().iter()
        ).for_each(|((p_state_e, state_e), f_out)| {
            *p_state_e += *state_e * *f_out;
        });
    }

    fn study_main_layer_with_state_error(
        &mut self,
        state_error: &MathVec<OUT>,
        cache: &MlLstmStateCache<OUT, IN>
    ) {
        self.tmp_error.as_mut_array().iter_mut().zip(
            state_error.as_array().iter()
        ).zip(
            cache.i_gate_cache.output.as_array().iter()
        ).for_each(|((tmp_e, state_e), i_out)| {
            *tmp_e = *state_e * *i_out;
        });

        self.main_layer.study(
            &self.tmp_error,
            None,
            &cache.main_layer_cache,
            &mut self.input_error_main_by_state_error,
            Some(&mut self.prev_state_error_main_by_state_error)
        );
    }

    fn study_f_gate_with_state_error(
        &mut self,
        state_error: &MathVec<OUT>,
        cache: &MlLstmStateCache<OUT, IN>
    ) {
        self.tmp_error.as_mut_array().iter_mut().zip(
            state_error.as_array().iter()
        ).zip(
            cache.prev_state.as_array().iter()
        ).for_each(|((tmp_e, state_e), p_state)| {
            *tmp_e = *state_e * *p_state;
        });

        self.f_gate.study(
            &self.tmp_error,
            None,
            &cache.f_gate_cache,
            &mut self.input_error_f_by_state_error,
            Some(&mut self.prev_state_error_f_by_state_error)
        );
    }

    fn study_i_gate_with_state_error(
        &mut self,
        state_error: &MathVec<OUT>,
        cache: &MlLstmStateCache<OUT, IN>
    ) {
        self.tmp_error.as_mut_array().iter_mut().zip(
            state_error.as_array().iter()
        ).zip(
            cache.main_layer_cache.output.as_array().iter()
        ).for_each(|((tmp_e, state_e), main_out)| {
            *tmp_e = *state_e * *main_out;
        });

        self.i_gate.study(
            &self.tmp_error,
            None,
            &cache.i_gate_cache,
            &mut self.input_error_i_by_state_error,
            Some(&mut self.prev_state_error_i_by_state_error)
        );
    }

    /// Studies weights with output error and state_error.
    ///
    /// - `output_error` : Backpropagated output error.
    /// - `state_error` : Backpropagated state error.
    /// - `state_cache` : Cache written at [`ready_state_cache()`](Self::ready_state_cache()).
    /// - `output_cache` : Cache written at [`ready_output_cache()`](Self::ready_output_cache()).
    /// - `input_error` : Error to backpropagate for previous output error.
    /// - `prev_state_error` : Error to backpropagate for previous state error.
    pub fn study(
        &mut self,
        output_error: &MathVec<OUT>,
        state_error: &MathVec<OUT>,
        state_cache: &MlLstmStateCache<OUT, IN>,
        output_cache: &MlLstmOutputCache<OUT, IN>,
        input_error: &mut MathVec<IN>,
        prev_state_error: &mut MathVec<OUT>
    ) {
        self.study_main_layer(
            output_error,
            state_error,
            state_cache,
            output_cache,
        );
        self.study_f_gate(
            output_error,
            state_error,
            state_cache,
            output_cache,
        );
        self.study_i_gate(
            output_error,
            state_error,
            state_cache,
            output_cache,
        );
        self.study_o_gate(
            output_error,
            output_cache,
        );

        input_error.copy_from(&self.input_error_main_by_output_error);
        *input_error += &self.input_error_f_by_output_error;
        *input_error += &self.input_error_i_by_output_error;
        *input_error += &self.input_error_o_by_output_error;

        prev_state_error.copy_from(
            &self.prev_state_error_main_by_output_error
        );
        *prev_state_error += &self.prev_state_error_f_by_output_error;
        *prev_state_error += &self.prev_state_error_i_by_output_error;
        *prev_state_error += &self.prev_state_error_o_by_output_error;

        prev_state_error.as_mut_array().iter_mut().zip(
            output_error.as_array().iter()
        ).zip(
            output_cache.o_gate_cache.output.as_array().iter()
        ).zip(
            output_cache.d_tanh_s.as_array().iter()
        ).zip(
            state_cache.f_gate_cache.output.as_array().iter()
        ).for_each(|((((p_state_e, out_e), o_out), d_tanh_s_one), f_out)| {
            *p_state_e += *out_e * *o_out * *d_tanh_s_one * *f_out;
        });
    }

    fn study_main_layer(
        &mut self,
        output_error: &MathVec<OUT>,
        state_error: &MathVec<OUT>,
        state_cache: &MlLstmStateCache<OUT, IN>,
        output_cache: &MlLstmOutputCache<OUT, IN>
    ) {
        self.tmp_error.as_mut_array().iter_mut().zip(
            output_error.as_array().iter()
        ).zip(
            output_cache.o_gate_cache.output.as_array().iter()
        ).zip(
            output_cache.d_tanh_s.as_array().iter()
        ).zip(
            state_cache.i_gate_cache.output.as_array().iter()
        ).zip(
            state_error.as_array().iter()
        ).for_each(
            |(((((tmp_e, out_e), o_out), d_tanh_s_one), i_out), state_e)| {
                *tmp_e = *out_e * *o_out * *d_tanh_s_one * *i_out;
                *tmp_e += *state_e * *i_out;
            }
        );

        self.main_layer.study(
            &self.tmp_error,
            None,
            &state_cache.main_layer_cache,
            &mut self.input_error_main_by_output_error,
            Some(&mut self.prev_state_error_main_by_output_error)
        );
    }

    fn study_f_gate(
        &mut self,
        output_error: &MathVec<OUT>,
        state_error: &MathVec<OUT>,
        state_cache: &MlLstmStateCache<OUT, IN>,
        output_cache: &MlLstmOutputCache<OUT, IN>
    ) {
        self.tmp_error.as_mut_array().iter_mut().zip(
            output_error.as_array().iter()
        ).zip(
            output_cache.o_gate_cache.output.as_array().iter()
        ).zip(
            output_cache.d_tanh_s.as_array().iter()
        ).zip(
            state_cache.prev_state.as_array().iter()
        ).zip(
            state_error.as_array().iter()
        ).for_each(
            |(((((tmp_e, out_e), o_out), d_tanh_s_one), p_state), state_e)| {
                *tmp_e = *out_e * *o_out * *d_tanh_s_one * *p_state;
                *tmp_e += *state_e * *p_state;
            }
        );

        self.f_gate.study(
            &self.tmp_error,
            None,
            &state_cache.f_gate_cache,
            &mut self.input_error_f_by_output_error,
            Some(&mut self.prev_state_error_f_by_output_error)
        );
    }

    fn study_i_gate(
        &mut self,
        output_error: &MathVec<OUT>,
        state_error: &MathVec<OUT>,
        state_cache: &MlLstmStateCache<OUT, IN>,
        output_cache: &MlLstmOutputCache<OUT, IN>
    ) {
        self.tmp_error.as_mut_array().iter_mut().zip(
            output_error.as_array().iter()
        ).zip(
            output_cache.o_gate_cache.output.as_array().iter()
        ).zip(
            output_cache.d_tanh_s.as_array().iter()
        ).zip(
            state_cache.main_layer_cache.output.as_array().iter()
        ).zip(
            state_error.as_array().iter()
        ).for_each(
            |(((((tmp_e, out_e), o_out), d_tanh_s_one), main_out), state_e)| {
                *tmp_e = *out_e * *o_out * *d_tanh_s_one * *main_out;
                *tmp_e += *state_e * *main_out;
            }
        );

        self.i_gate.study(
            &self.tmp_error,
            None,
            &state_cache.i_gate_cache,
            &mut self.input_error_i_by_output_error,
            Some(&mut self.prev_state_error_i_by_output_error)
        );
    }

    fn study_o_gate(
        &mut self,
        output_error: &MathVec<OUT>,
        cache: &MlLstmOutputCache<OUT, IN>
    ) {
        self.tmp_error.as_mut_array().iter_mut().zip(
            output_error.as_array().iter()
        ).zip(
            cache.tanh_s.as_array().iter()
        ).for_each(|((tmp_e, out_e),  tanh_s_one)| {
            *tmp_e = *out_e * *tanh_s_one;
        });

        self.o_gate.study(
            &self.tmp_error,
            None,
            &cache.o_gate_cache,
            &mut self.input_error_o_by_output_error,
            Some(&mut self.prev_state_error_o_by_output_error)
        );
    }

    /// Update weights.
    ///
    /// - `rate` : Learning rate.
    #[inline]
    pub fn update(&mut self, rate: f32) {
        self.main_layer.update(rate);
        self.f_gate.update(rate);
        self.i_gate.update(rate);
        self.o_gate.update(rate);
    }

    /// Accesses each immutable total gradient with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_total_grad<F>(
        &self,
        mut f: F
    ) where F: FnMut(&f32) {
        self.main_layer.total_grad.iter().for_each(|val| {f(val)});
        self.f_gate.total_grad.iter().for_each(|val| {f(val)});
        self.i_gate.total_grad.iter().for_each(|val| {f(val)});
        self.o_gate.total_grad.iter().for_each(|val| {f(val)});
    }

    /// Accesses each mutable total gradient with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_total_grad_mut<F>(
        &mut self,
        mut f: F
    ) where F: FnMut(&mut f32) {
        self.main_layer.total_grad.iter_mut().for_each(|val| {f(val)});
        self.f_gate.total_grad.iter_mut().for_each(|val| {f(val)});
        self.i_gate.total_grad.iter_mut().for_each(|val| {f(val)});
        self.o_gate.total_grad.iter_mut().for_each(|val| {f(val)});
    }
}

/// Encoder from sequence data to fixed length data.
///
/// - `OUT` : Dimension of output.
/// - `MIDDLE` : Dimension of hidden layer.
/// - `IN` : Dimension of input.
///
/// # Example
///
/// Variable length word classifier.
/// - If inputs Japanese word, outputs '日'.
/// - If inputs English word, outputs 'E'.
///
/// (1) Defines word generator and ID.
///
/// ```ignore
/// extern crate chobitlibs;
/// 
/// use chobitlibs::chobit_ai::{
///     MathVec,
///     Activation,
///     ChobitEncoder,
///     ChobitMlEncoder,
///     MlEncoderCache
/// };
/// 
/// use chobitlibs::chobit_rand::ChobitRand;
/// 
/// fn japanese_letter(rng: &mut ChobitRand) -> char {
///     let letters = [
///         'あ', 'い', 'う', 'え', 'お',
///         'か', 'き', 'く', 'け', 'こ',
///         'さ', 'し', 'す', 'せ', 'そ'
///     ];
/// 
///     letters[(rng.next_u64() as usize) % letters.len()]
/// }
/// 
/// fn english_letter(rng: &mut ChobitRand) -> char {
///     let letters = [
///         'a', 'b', 'c', 'd', 'e',
///         'f', 'g', 'h', 'i', 'j',
///         'k', 'l', 'm', 'n', 'o'
///     ];
/// 
///     letters[(rng.next_u64() as usize) % letters.len()]
/// }
/// 
/// const JAPANESE_ID: char = '日';
/// const ENGLISH_ID: char = 'E';
/// 
/// fn gen_word(
///     f: fn(&mut ChobitRand) -> char,
///     rng: &mut ChobitRand,
///     max_len: usize
/// ) -> String {
///     let word_len = ((rng.next_u64() as usize) % max_len) + 1;
/// 
///     let mut ret = String::with_capacity(word_len);
/// 
///     for _ in 0..word_len {
///         ret.push(f(rng));
///     }
/// 
///     ret
/// }
/// 
/// fn write_string_to_slice(string: &str, slice: &mut [MathVec<32>]) {
///     string.chars().zip(slice.iter_mut()).for_each(|(c, s)| {
///         s.load_u32_label(c as u32);
///     });
/// }
/// ```
///
/// (2) Creates [`ChobitEncoder`] and randomises weights.
///
/// ```ignore
/// const OUT: usize = 32;
/// const MIDDLE: usize = 64;
/// const IN: usize = 32;
///
/// const MAX_WORD_LEN: usize = 10;
///
/// let mut rng = ChobitRand::new(b"ChobitEncoder Example");
///
/// let mut encoder =
///     ChobitEncoder::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
///
/// // Randomises weights.
/// encoder.for_each_weight_mut(|weight| {
///     *weight = ((rng.next_f64() as f32) * 2.0) - 1.0;
/// });
///
/// let mut input = vec![MathVec::<IN>::new(); MAX_WORD_LEN];
/// let mut output = MathVec::<OUT>::new();
/// let initial_state = MathVec::<MIDDLE>::new();
/// ```
///
/// (3) Wraps AI with [`ChobitMlEncoder`] for machine learning.
///
/// ```ignore
/// let mut encoder = ChobitMlEncoder::<OUT, MIDDLE, IN>::new(encoder);
/// let mut cache = MlEncoderCache::<OUT, MIDDLE, IN>::new(MAX_WORD_LEN);
///
/// let mut input_error = vec![MathVec::<IN>::new(); MAX_WORD_LEN];
/// let mut output_error = MathVec::<OUT>::new();
/// let mut prev_state_error = MathVec::<MIDDLE>::new();
/// ```
///
/// (4) Machine learning.
///
/// ```ignore
/// const EPOCH: usize = 1000;
/// const BATCH_SIZE: usize = 100;
/// const RATE: f32 = 0.01;
///
/// for _ in 0..EPOCH {
///     for _ in 0..BATCH_SIZE {
///         //--- Learns Japanese ---//
///         let string = gen_word(japanese_letter, &mut rng, MAX_WORD_LEN);
///
///         write_string_to_slice(&string, &mut input);
///
///         output.load_u32_label(JAPANESE_ID as u32);
///
///         // Writes cache.
///         encoder.ready(
///             &input[..string.chars().count()],
///             &initial_state,
///             &mut cache
///         );
///
///         // Calculates error.
///         cache.calc_output_error(&output, &mut output_error);
///
///         // Studies.
///         encoder.study(
///             &output_error,
///             &cache,
///             &mut input_error,
///             &mut prev_state_error
///         );
///
///         //--- Learns English ---//
///         let string = gen_word(english_letter, &mut rng, MAX_WORD_LEN);
///
///         write_string_to_slice(&string, &mut input);
///
///         output.load_u32_label(ENGLISH_ID as u32);
///
///         // Writes cache.
///         encoder.ready(
///             &input[..string.chars().count()],
///             &initial_state,
///             &mut cache
///         );
///
///         // Calculates error.
///         cache.calc_output_error(&output, &mut output_error);
///
///         // Studies.
///         encoder.study(
///             &output_error,
///             &cache,
///             &mut input_error,
///             &mut prev_state_error
///         );
///     }
///
///     // Updates weights.
///     encoder.update(RATE);
/// }
/// ```
///
/// (5) Tests encoder.
///
/// ```ignore
/// // Unwrap Encoder.
/// let mut encoder = encoder.drop();
///
/// // Tests Japanese.
/// for _ in 0..10 {
///     let string = gen_word(japanese_letter, &mut rng, MAX_WORD_LEN);
///
///     write_string_to_slice(&string, &mut input);
///
///     // Initializes state.
///     encoder.state_mut().copy_from(&initial_state);
///
///     // Inputs for each one.
///     input[..string.chars().count()].iter().for_each(|input_one| {
///         encoder.input_next(input_one)
///     });
///
///     // Outputs.
///     encoder.output(&mut output);
///
///     assert_eq!(output.to_u32_label(), JAPANESE_ID as u32);
/// }
///
/// // Tests English.
/// for _ in 0..10 {
///     let string = gen_word(english_letter, &mut rng, MAX_WORD_LEN);
///
///     write_string_to_slice(&string, &mut input);
///
///     // Initializes state.
///     encoder.state_mut().copy_from(&initial_state);
///
///     // Inputs for each one.
///     input[..string.chars().count()].iter().for_each(|input_one| {
///         encoder.input_next(input_one)
///     });
///
///     // Outputs.
///     encoder.output(&mut output);
///
///     assert_eq!(output.to_u32_label(), ENGLISH_ID as u32);
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ChobitEncoder<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> {
    lstm: Lstm<MIDDLE, IN>,
    output_layer: Layer<OUT, MIDDLE>,

    prev_state: MathVec<MIDDLE>,
    state: MathVec<MIDDLE>,
    last_input: MathVec<IN>,

    middle_output: MathVec<MIDDLE>,
    tmpbuf: MathVec<MIDDLE>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> ChobitEncoder<OUT, MIDDLE, IN> {
    /// Creates ChobitEncoder.
    ///
    /// - `activation` : Activation function for output layer.
    /// - _Return_ : ChobitEncoder.
    #[inline]
    pub fn new(activation: Activation) -> Self {
        Self {
            lstm: Lstm::<MIDDLE, IN>::new(),
            output_layer: Layer::<OUT, MIDDLE>::new(activation, false),

            prev_state: MathVec::<MIDDLE>::new(),
            state: MathVec::<MIDDLE>::new(),
            last_input: MathVec::<IN>::new(),

            middle_output: MathVec::<MIDDLE>::new(),
            tmpbuf: MathVec::<MIDDLE>::new()
        }
    }

    /// Gets immutable Lstm.
    ///
    /// - _Return_ : Lstm.
    #[inline]
    pub fn lstm(&self) -> &Lstm<MIDDLE, IN> {&self.lstm}

    /// Gets mutable Lstm.
    ///
    /// - _Return_ : Lstm.
    #[inline]
    pub fn lstm_mut(&mut self) -> &mut Lstm<MIDDLE, IN> {&mut self.lstm}

    /// Gets immutable output layer.
    ///
    /// - _Return_ : Output layer.
    #[inline]
    pub fn output_layer(&self) -> &Layer<OUT, MIDDLE> {&self.output_layer}

    /// Gets mutable output layer.
    ///
    /// - _Return_ : Output layer.
    #[inline]
    pub fn output_layer_mut(&mut self) -> &mut Layer<OUT, MIDDLE> {
        &mut self.output_layer
    }

    /// Gets immutable state.
    ///
    /// This should be initialized before the first [`input_next()`](Self::input_next).
    ///
    /// - _Return_ : State.
    #[inline]
    pub fn state(&self) -> &MathVec<MIDDLE> {&self.state}

    /// Gets mutable state.
    ///
    /// This should be initialized before the first [`input_next()`](Self::input_next).
    ///
    /// - _Return_ : State.
    #[inline]
    pub fn state_mut(&mut self) -> &mut MathVec<MIDDLE> {&mut self.state}

    /// Gets last input.
    ///
    /// - _Return_ : Last input.
    #[inline]
    pub fn last_input(&self) -> &MathVec<IN> {&self.last_input}

    /// Accesses each immutable weight with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_weight<F>(&self, mut f: F) where F: FnMut(&f32) {
        self.lstm.for_each_weight(|val| {f(val)});
        self.output_layer.weights().iter().for_each(|val| {f(val)});
    }

    /// Accesses each mutable weight with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_weight_mut<F>(
        &mut self,
        mut f: F
    ) where F: FnMut(&mut f32) {
        self.lstm.for_each_weight_mut(|val| {f(val)});
        self.output_layer.mut_weights().iter_mut().for_each(|val| {f(val)});
    }

    /// Input next data.
    ///
    /// - `input` : Next data.
    #[inline]
    pub fn input_next(&mut self, input: &MathVec<IN>) {
        self.prev_state.copy_from(&self.state);
        self.last_input.copy_from(input);

        self.lstm.calc_state(
            input,
            &self.prev_state,
            &mut self.state,
            &mut self.tmpbuf
        );
    }

    /// Output data calculated by current state and last input.
    ///
    /// - `output` : Buffer for output.
    #[inline]
    pub fn output(&mut self, output: &mut MathVec<OUT>) {
        self.prev_state.copy_from(&self.state);

        self.lstm.calc(
            &self.last_input,
            &self.prev_state,
            &mut self.middle_output,
            &mut self.state,
            &mut self.tmpbuf
        );

        self.output_layer.calc(&self.middle_output, None, output);
    }
}

/// Cache for [`ChobitMlEncoder`].
///
/// - `OUT` : `OUT` of [`ChobitMlEncoder`].
/// - `MIDDLE` : `MIDDLE` of [`ChobitMlEncoder`].
/// - `IN` : `IN` of [`ChobitMlEncoder`].
#[derive(Debug, Clone, PartialEq)]
pub struct MlEncoderCache<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> {
    lstm_state_caches: Vec<MlLstmStateCache<MIDDLE, IN>>,
    lstm_state_caches_len: usize,

    lstm_output_cache: MlLstmOutputCache<MIDDLE, IN>,

    output_layer_cache: MlCache<OUT, MIDDLE>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> MlEncoderCache<OUT, MIDDLE, IN> {
    /// Creates MlEncoderCache.
    ///
    /// - _Return_ : MlEncoderCache.
    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            lstm_state_caches: vec![
                MlLstmStateCache::<MIDDLE, IN>::new(); capacity
            ],
            lstm_state_caches_len: 0,

            lstm_output_cache: MlLstmOutputCache::<MIDDLE, IN>::new(),

            output_layer_cache: MlCache::<OUT, MIDDLE>::new()
        }
    }

    /// Calculates output error.
    ///
    /// | Formula |
    /// |:-:|
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mi>e</mi> <mo stretchy="false">=</mo> <mrow> <mi>o</mi> <mo stretchy="false">−</mo> <mi>t</mi> </mrow> </mrow> </semantics> </math> |
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mi>e</mi> <mo stretchy="false">≝</mo> <mtext>Error.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>o</mi> <mo stretchy="false">≝</mo> <mtext>Actual output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>t</mi> <mo stretchy="false">≝</mo> <mtext>Correct output.</mtext> </mrow> </mtd> </mtr> </mtable> </semantics> </math> |
    ///
    /// - `train_out` : Correct output.
    /// - `output_error` : Buffer for output error.
    #[inline]
    pub fn calc_output_error(
        &self,
        train_out: &MathVec<OUT>,
        output_error: &mut MathVec<OUT>
    ) {
        output_error.copy_from(&self.output_layer_cache.output);
        *output_error -= train_out;
    }

    /// Gets slice of ['MlLstmStateCache'] of lstm layer.
    ///
    /// - _Return_ : Slice of ['MlLstmStateCache'].
    #[inline]
    pub fn lstm_state_caches(&self) -> &[MlLstmStateCache<MIDDLE, IN>] {
        &self.lstm_state_caches[..self.lstm_state_caches_len]
    }

    /// Gets ['MlLstmOutputCache'] of lstm layer.
    ///
    /// - _Return_ : ['MlLstmOutputCache'].
    #[inline]
    pub fn lstm_output_cache(&self) -> &MlLstmOutputCache<MIDDLE, IN> {
        &self.lstm_output_cache
    }

    /// Gets ['MlCache'] of output layer.
    ///
    /// - _Return_ : [`MlCache`].
    #[inline]
    pub fn output_layer_cache(&self) -> &MlCache<OUT, MIDDLE> {
        &self.output_layer_cache
    }

    /// Gets output.
    ///
    /// - _Return_ : output.
    #[inline]
    pub fn output(&self) -> &MathVec<OUT> {
        &self.output_layer_cache.output
    }
}

/// Wrapper of [`ChobitEncoder`] for machine learning.
///
/// See [`ChobitEncoder`] for details.
///
/// - `OUT` : Dimension of output.
/// - `MIDDLE` : Dimension of hidden layer.
/// - `IN` : Dimension of input.
#[derive(Debug, Clone, PartialEq)]
pub struct ChobitMlEncoder<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> {
    lstm: MlLstm<MIDDLE, IN>,
    output_layer: MlLayer<OUT, MIDDLE>,

    prev_state: MathVec<MIDDLE>,

    tmp_middle_output_error: MathVec<MIDDLE>,
    tmp_state_error: MathVec<MIDDLE>,

    original_prev_state: MathVec<MIDDLE>,
    original_state: MathVec<MIDDLE>,
    original_last_input: MathVec<IN>,

    original_middle_output: MathVec<MIDDLE>,
    original_tmpbuf: MathVec<MIDDLE>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> ChobitMlEncoder<OUT, MIDDLE, IN> {
    /// Creates ChobitMlEncoder.
    ///
    /// - `encoder` : Base [`ChobitEncoder`].
    /// - _Return_ : ChobitMlEncoder.
    #[inline]
    pub fn new(encoder: ChobitEncoder<OUT, MIDDLE, IN>) -> Self {
        let ChobitEncoder::<OUT, MIDDLE, IN> {
            lstm,
            output_layer,
            prev_state,
            state,
            last_input,
            middle_output,
            tmpbuf
        } = encoder;

        Self {
            lstm: MlLstm::<MIDDLE, IN>::new(lstm),
            output_layer: MlLayer::<OUT, MIDDLE>::new(output_layer),

            prev_state: MathVec::<MIDDLE>::new(),

            tmp_middle_output_error: MathVec::<MIDDLE>::new(),
            tmp_state_error: MathVec::<MIDDLE>::new(),

            original_prev_state: prev_state,
            original_state: state,
            original_last_input: last_input,
            original_middle_output: middle_output,
            original_tmpbuf: tmpbuf
        }
    }

    /// Drops base [`ChobitEncoder`].
    ///
    /// - _Return_ : [`ChobitEncoder`].
    #[inline]
    pub fn drop(self) -> ChobitEncoder<OUT, MIDDLE, IN> {
        let Self {
            lstm,
            output_layer,
            original_prev_state,
            original_state,
            original_last_input,
            original_middle_output,
            original_tmpbuf,
            ..
        } = self;

        ChobitEncoder::<OUT, MIDDLE, IN> {
            lstm: lstm.drop(),
            output_layer: output_layer.drop(),
            prev_state: original_prev_state,
            state: original_state,
            last_input: original_last_input,
            middle_output: original_middle_output,
            tmpbuf: original_tmpbuf
        }
    }

    /// Clears internal data for study.
    #[inline]
    pub fn clear_study_data(&mut self) {
        self.lstm.clear_study_data();
        self.output_layer.clear_study_data();
    }

    /// Writes information on [`MlEncoderCache`] for [`study()`](Self::study()).
    ///
    /// - `train_in` : Input data sequence.
    /// - `prev_state` : Previous state.
    /// - `cache` : Cache.
    #[inline]
    pub fn ready(
        &mut self,
        train_in: &[MathVec<IN>],
        prev_state: &MathVec<MIDDLE>,
        cache: &mut MlEncoderCache<OUT, MIDDLE, IN>
    ) {
        self.ready_state_cache(train_in, prev_state, cache);
        self.ready_output_cache(cache);
    }

    fn ready_state_cache(
        &mut self,
        train_in: &[MathVec<IN>],
        prev_state: &MathVec<MIDDLE>,
        cache: &mut MlEncoderCache<OUT, MIDDLE, IN>
    ) {
        cache.lstm_state_caches_len = train_in.len();
        if cache.lstm_state_caches.len() < train_in.len() {
            cache.lstm_state_caches.resize(
                train_in.len(),
                MlLstmStateCache::<MIDDLE, IN>::new()
            );
        }

        self.prev_state.copy_from(prev_state);

        train_in.iter().zip(
            cache.lstm_state_caches.iter_mut()
        ).for_each(|(train_in_one, cache)| {
            self.lstm.ready_state_cache(
                train_in_one,
                &self.prev_state,
                cache
            );

            self.prev_state.copy_from(&cache.state);
        })
    }

    fn ready_output_cache(
        &self,
        cache: &mut MlEncoderCache<OUT, MIDDLE, IN>
    ) {
        if let Some(last_state_cache) = cache.lstm_state_caches.get(
            cache.lstm_state_caches_len.wrapping_sub(1)
        ) {
            self.lstm.ready_output_cache(
                last_state_cache,
                &mut cache.lstm_output_cache
            );

            self.output_layer.ready(
                &cache.lstm_output_cache.output,
                None,
                &mut cache.output_layer_cache
            )
        }
    }

    /// Accumulates gradient.
    ///
    /// - `output_error` : Backpropagated ouput error.
    /// - `cache` : Cache.
    /// - `input_error` : Error to backpropagate for previous output error.
    /// - `prev_state_error` : Error to backpropagate for previous state error.
    pub fn study(
        &mut self,
        output_error: &MathVec<OUT>,
        cache: &MlEncoderCache<OUT, MIDDLE, IN>,
        input_error: &mut [MathVec<IN>],
        prev_state_error: &mut MathVec<MIDDLE>
    ) {
        let mut iter = cache.lstm_state_caches().iter().zip(
            input_error.iter_mut()
        ).rev();

        self.tmp_state_error.clear();

        if let Some((lstm_state_cache, input_error)) = iter.next() {
            self.output_layer.study(
                &output_error,
                None,
                &cache.output_layer_cache,
                &mut self.tmp_middle_output_error,
                None
            );

            self.lstm.study(
                &self.tmp_middle_output_error,
                &self.tmp_state_error,
                &lstm_state_cache,
                &cache.lstm_output_cache,
                input_error,
                prev_state_error
            );
        }

        self.tmp_state_error.copy_from(prev_state_error);

        iter.for_each(|(lstm_state_cache, input_error)| {
            self.lstm.study_state(
                &self.tmp_state_error,
                lstm_state_cache,
                input_error,
                prev_state_error
            );

            self.tmp_state_error.copy_from(prev_state_error);
        });
    }

    /// Updates weights.
    ///
    /// - `rate` : Learning rate.
    #[inline]
    pub fn update(&mut self, rate: f32) {
        self.lstm.update(rate);
        self.output_layer.update(rate);
    }

    /// Accesses each immutable total gradient with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_total_grad<F>(
        &self,
        mut f: F
    ) where F: FnMut(&f32) {
        self.lstm.for_each_total_grad(|val| {f(val)});
        self.output_layer.total_grad.iter().for_each(|val| {f(val)});
    }

    /// Accesses each mutable total gradient with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_total_grad_mut<F>(
        &mut self,
        mut f: F
    ) where F: FnMut(&mut f32) {
        self.lstm.for_each_total_grad_mut(|val| {f(val)});
        self.output_layer.total_grad.iter_mut().for_each(|val| {f(val)});
    }
}

/// Decoder from fixed length data to sequence data.
///
/// - `OUT` : Dimension of output.
/// - `MIDDLE` : Dimension of hidden layer.
/// - `IN` : Dimension of input.
///
/// # Example
///
/// Letter commentator.
/// - If inputs '日', outputs "これは日本語です。"
/// - If inputs 'E', outputs "This is English."
///
/// (1) Defines ID and messages.
///
/// ```ignore
/// extern crate chobitlibs;
/// 
/// use chobitlibs::chobit_ai::{
///     MathVec,
///     Activation,
///     ChobitDecoder,
///     ChobitMlDecoder,
///     MlDecoderCache
/// };
/// 
/// use chobitlibs::chobit_rand::ChobitRand;
/// 
/// const JAPANESE_ID: char = '日';
/// const ENGLISH_ID: char = 'E';
/// 
/// const JAPANESE_MESSAGE: &str = "これは日本語です。";
/// const ENGLISH_MESSAGE: &str = "This is English.";
/// 
/// fn write_string_to_slice(string: &str, slice: &mut [MathVec<32>]) {
///     string.chars().zip(slice.iter_mut()).for_each(|(c, s)| {
///         s.load_u32_label(c as u32);
///     });
/// }
/// ```
///
/// (2) Creates [`ChobitDecoder`] and randomises weights.
///
/// ```ignore
/// const OUT: usize = 32;
/// const MIDDLE: usize = 64;
/// const IN: usize = 32;
///
/// let max_message_len = JAPANESE_MESSAGE.len().max(ENGLISH_MESSAGE.len());
///
/// let mut rng = ChobitRand::new(b"ChobitDecoder Example");
///
/// let mut decoder =
///     ChobitDecoder::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
///
/// // Randomises weights.
/// decoder.for_each_weight_mut(|weight| {
///     *weight = ((rng.next_f64() as f32) * 2.0) - 1.0;
/// });
///
/// let mut input = MathVec::<IN>::new();
/// let mut output = vec![MathVec::<OUT>::new(); max_message_len];
/// let initial_state = MathVec::<MIDDLE>::new();
/// ```
///
/// (3) Wraps AI with [`ChobitMlDecoder`] for machine learning.
///
/// ```ignore
/// let mut decoder = ChobitMlDecoder::<OUT, MIDDLE, IN>::new(decoder);
/// let mut cache = MlDecoderCache::<OUT, MIDDLE, IN>::new(max_message_len);
///
/// let mut input_error = MathVec::<IN>::new();
/// let mut output_error = vec![MathVec::<OUT>::new(); max_message_len];
/// let mut prev_state_error = MathVec::<MIDDLE>::new();
/// ```
///
/// (4) Machine learning.
///
/// ```ignore
/// const EPOCH: usize = 10000;
/// const BATCH_SIZE: usize = 1;
/// const RATE: f32 = 0.01;
///
/// let japanese_message_len = JAPANESE_MESSAGE.chars().count();
/// let english_message_len = ENGLISH_MESSAGE.chars().count();
///
/// for _ in 0..EPOCH {
///     for _ in 0..BATCH_SIZE {
///         //--- Learns Japanese ---//
///         input.load_u32_label(JAPANESE_ID as u32);
///
///         write_string_to_slice(&JAPANESE_MESSAGE, &mut output);
///
///         // Writes cache.
///         decoder.ready(
///             &input,
///             &initial_state,
///             japanese_message_len,
///             &mut cache
///         );
///
///         // Calculates error.
///         cache.calc_output_error(
///             &output[..japanese_message_len],
///             &mut output_error[..japanese_message_len]
///         );
///
///         // Studies.
///         decoder.study(
///             &output_error[..japanese_message_len],
///             &cache,
///             &mut input_error,
///             &mut prev_state_error
///         );
///
///         //--- Learns English ---//
///         input.load_u32_label(ENGLISH_ID as u32);
///
///         write_string_to_slice(&ENGLISH_MESSAGE, &mut output);
///
///         // Writes cache.
///         decoder.ready(
///             &input,
///             &initial_state,
///             english_message_len,
///             &mut cache
///         );
///
///         // Calculates error.
///         cache.calc_output_error(
///             &output[..english_message_len],
///             &mut output_error[..english_message_len]
///         );
///
///         // Studies.
///         decoder.study(
///             &output_error[..english_message_len],
///             &cache,
///             &mut input_error,
///             &mut prev_state_error
///         );
///     }
///
///     // Updates weights.
///     decoder.update(RATE);
/// }
/// ```
///
/// (5) Tests AI.
///
/// ```ignore
/// // Unwrap Decoder.
/// let mut decoder = decoder.drop();
///
/// let mut output = MathVec::<OUT>::new();
///
/// // Tests Japanese.
/// // Sets input.
/// decoder.input_mut().load_u32_label(JAPANESE_ID as u32);
///
/// // Initializes state.
/// decoder.state_mut().copy_from(&initial_state);
///
/// // Outputs for each one.
/// JAPANESE_MESSAGE.chars().for_each(|c| {
///     decoder.output_next(&mut output);
///
///     assert_eq!(output.to_u32_label(), c as u32);
/// });
///
/// // Tests English.
/// // Sets input.
/// decoder.input_mut().load_u32_label(ENGLISH_ID as u32);
///
/// // Initializes state.
/// decoder.state_mut().copy_from(&initial_state);
///
/// // Outputs for each one.
/// ENGLISH_MESSAGE.chars().for_each(|c| {
///     decoder.output_next(&mut output);
///
///     assert_eq!(output.to_u32_label(), c as u32);
/// });
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ChobitDecoder<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> {
    lstm: Lstm<MIDDLE, IN>,
    output_layer: Layer<OUT, MIDDLE>,

    input: MathVec<IN>,
    prev_state: MathVec<MIDDLE>,
    state: MathVec<MIDDLE>,

    middle_output: MathVec<MIDDLE>,
    tmpbuf: MathVec<MIDDLE>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> ChobitDecoder<OUT, MIDDLE, IN> {
    /// Creates ChobitDecoder.
    ///
    /// - `activation` : Activation function for output layer.
    /// - _Return_ : ChobitDecoder.
    pub fn new(activation: Activation) -> Self {
        Self {
            lstm: Lstm::<MIDDLE, IN>::new(),
            output_layer: Layer::<OUT, MIDDLE>::new(activation, false),

            input: MathVec::<IN>::new(),
            prev_state: MathVec::<MIDDLE>::new(),
            state: MathVec::<MIDDLE>::new(),

            middle_output: MathVec::<MIDDLE>::new(),
            tmpbuf: MathVec::<MIDDLE>::new()
        }
    }

    /// Gets immutable Lstm.
    ///
    /// - _Return_ : Lstm.
    #[inline]
    pub fn lstm(&self) -> &Lstm<MIDDLE, IN> {&self.lstm}

    /// Gets mutable Lstm.
    ///
    /// - _Return_ : Lstm.
    #[inline]
    pub fn lstm_mut(&mut self) -> &mut Lstm<MIDDLE, IN> {&mut self.lstm}

    /// Gets immutable output layer.
    ///
    /// - _Return_ : Output layer.
    #[inline]
    pub fn output_layer(&self) -> &Layer<OUT, MIDDLE> {&self.output_layer}

    /// Gets mutable output layer.
    ///
    /// - _Return_ : Output layer.
    #[inline]
    pub fn output_layer_mut(&mut self) -> &mut Layer<OUT, MIDDLE> {
        &mut self.output_layer
    }

    /// Gets immutable input.
    ///
    /// This should be set before to call [`output_next()`](Self::output_next()).
    ///
    /// - _Return_ : Input.
    #[inline]
    pub fn input(&self) -> &MathVec<IN> {&self.input}

    /// Gets mutable input.
    ///
    /// This should be set before to call [`output_next()`](Self::output_next()).
    ///
    /// - _Return_ : Input.
    #[inline]
    pub fn input_mut(&mut self) -> &mut MathVec<IN> {&mut self.input}

    /// Gets immutable state.
    ///
    /// This should be initialized before to call [`output_next()`](Self::output_next()).
    ///
    /// - _Return_ : State.
    #[inline]
    pub fn state(&self) -> &MathVec<MIDDLE> {&self.state}

    /// Gets mutable state.
    ///
    /// This should be initialized before to call [`output_next()`](Self::output_next()).
    ///
    /// - _Return_ : State.
    #[inline]
    pub fn state_mut(&mut self) -> &mut MathVec<MIDDLE> {&mut self.state}

    /// Output next data calculated by current state and input.
    ///
    /// - `output` : Buffer for next data.
    pub fn output_next(&mut self, output: &mut MathVec<OUT>) {
        self.prev_state.copy_from(&self.state);

        self.lstm.calc(
            &self.input,
            &self.prev_state,
            &mut self.middle_output,
            &mut self.state,
            &mut self.tmpbuf,
        );

        self.output_layer.calc(&self.middle_output, None, output);
    }

    /// Accesses each immutable weight with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_weight<F>(&self, mut f: F) where F: FnMut(&f32) {
        self.lstm.for_each_weight(|val| {f(val)});
        self.output_layer.weights().iter().for_each(|val| {f(val)});
    }

    /// Accesses each mutable weight with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_weight_mut<F>(
        &mut self,
        mut f: F
    ) where F: FnMut(&mut f32) {
        self.lstm.for_each_weight_mut(|val| {f(val)});
        self.output_layer.mut_weights().iter_mut().for_each(|val| {f(val)});
    }
}

/// Cache for [`ChobitMlDecoder`].
///
/// - `OUT` : `OUT` of [`ChobitMlDecoder`].
/// - `MIDDLE` : `MIDDLE` of [`ChobitMlDecoder`].
/// - `IN` : `IN` of [`ChobitMlDecoder`].
#[derive(Debug, Clone, PartialEq)]
pub struct MlDecoderCache<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> {
    caches: Vec<(
        MlLstmStateCache<MIDDLE, IN>,
        MlLstmOutputCache<MIDDLE, IN>,
        MlCache<OUT, MIDDLE>
    )>,

    caches_len: usize
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> MlDecoderCache<OUT, MIDDLE, IN> {
    /// Creates MlDecoderCache.
    ///
    /// - _Return_ : MlDecoderCache.
    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            caches: vec![
                (
                    MlLstmStateCache::<MIDDLE, IN>::new(),
                    MlLstmOutputCache::<MIDDLE, IN>::new(),
                    MlCache::<OUT, MIDDLE>::new(),
                ); capacity
            ],
            caches_len: 0
        }
    }

    /// Calculates output error.
    ///
    /// | Formula |
    /// |:-:|
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mi>e</mi> <mo stretchy="false">=</mo> <mrow> <mi>o</mi> <mo stretchy="false">−</mo> <mi>t</mi> </mrow> </mrow> </semantics> </math> |
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mi>e</mi> <mo stretchy="false">≝</mo> <mtext>Error.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>o</mi> <mo stretchy="false">≝</mo> <mtext>Actual output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>t</mi> <mo stretchy="false">≝</mo> <mtext>Correct output.</mtext> </mrow> </mtd> </mtr> </mtable> </semantics> </math> |
    ///
    /// - `train_out` : Correct output.
    /// - `output_error` : Buffer for output error.
    #[inline]
    pub fn calc_output_error(
        &self,
        train_out: &[MathVec<OUT>],
        output_error: &mut [MathVec<OUT>]
    ) {
        train_out.iter().zip(
            &self.caches
        ).zip(
            output_error
        ).for_each(|(
            (
                train_out_one,
                (_, _, output_layer_cache)
            ),
            output_error_one
        )| {
            output_error_one.copy_from(&output_layer_cache.output);
            *output_error_one -= train_out_one;
        });
    }

    /// Gets immutable slice of caches.
    ///
    /// - _Return_ : Slice of caches.
    #[inline]
    pub fn caches(&self) -> &[(
        MlLstmStateCache<MIDDLE, IN>,
        MlLstmOutputCache<MIDDLE, IN>,
        MlCache<OUT, MIDDLE>
    )] {
        &self.caches[..self.caches_len]
    }

    /// Gets mutable slice of caches.
    ///
    /// - _Return_ : Slice of caches.
    #[inline]
    pub(self) fn caches_mut(&mut self) -> &mut [(
        MlLstmStateCache<MIDDLE, IN>,
        MlLstmOutputCache<MIDDLE, IN>,
        MlCache<OUT, MIDDLE>
    )] {
        &mut self.caches[..self.caches_len]
    }

    /// Accesses each output with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_output<F>(&self, mut f: F) where F: FnMut(&MathVec<OUT>) {
        self.caches[..self.caches_len].iter().for_each(
            |(_, _, cache)| {f(&cache.output);}
        );
    }
}

/// Wrapper of [`ChobitDecoder`] for machine learning.
///
/// See [`ChobitDecoder`] for details.
///
/// - `OUT` : Dimension of output.
/// - `MIDDLE` : Dimension of hidden layer.
/// - `IN` : Dimension of input.
#[derive(Debug, Clone, PartialEq)]
pub struct ChobitMlDecoder<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> {
    lstm: MlLstm<MIDDLE, IN>,
    output_layer: MlLayer<OUT, MIDDLE>,

    prev_state: MathVec<MIDDLE>,

    tmp_middle_output_error: MathVec<MIDDLE>,
    tmp_state_error: MathVec<MIDDLE>,
    tmp_input_error: MathVec<IN>,

    original_input: MathVec<IN>,
    original_prev_state: MathVec<MIDDLE>,
    original_state: MathVec<MIDDLE>,

    original_middle_output: MathVec<MIDDLE>,
    original_tmpbuf: MathVec<MIDDLE>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> ChobitMlDecoder<OUT, MIDDLE, IN> {
    /// Creates ChobitMlDecoder.
    ///
    /// - `decoder` : Base [`ChobitDecoder`].
    /// - _Return_ : ChobitMlDecoder.
    #[inline]
    pub fn new(decoder: ChobitDecoder<OUT, MIDDLE, IN>) -> Self {
        let ChobitDecoder::<OUT, MIDDLE, IN> {
            lstm,
            output_layer,
            input,
            prev_state,
            state,
            middle_output,
            tmpbuf
        } = decoder;

        Self {
            lstm: MlLstm::<MIDDLE, IN>::new(lstm),
            output_layer: MlLayer::<OUT, MIDDLE>::new(output_layer),

            prev_state: MathVec::<MIDDLE>::new(),

            tmp_middle_output_error: MathVec::<MIDDLE>::new(),
            tmp_state_error: MathVec::<MIDDLE>::new(),
            tmp_input_error: MathVec::<IN>::new(),

            original_input: input,
            original_prev_state: prev_state,
            original_state: state,
            original_middle_output: middle_output,
            original_tmpbuf: tmpbuf
        }
    }

    /// Drops base [`ChobitDecoder`].
    ///
    /// - _Return_ : [`ChobitDecoder`].
    #[inline]
    pub fn drop(self) -> ChobitDecoder<OUT, MIDDLE, IN> {
        let Self {
            lstm,
            output_layer,
            original_input,
            original_prev_state,
            original_state,
            original_middle_output,
            original_tmpbuf,
            ..
        } = self;

        ChobitDecoder::<OUT, MIDDLE, IN> {
            lstm: lstm.drop(),
            output_layer: output_layer.drop(),
            input: original_input,
            prev_state: original_prev_state,
            state: original_state,
            middle_output: original_middle_output,
            tmpbuf: original_tmpbuf
        }
    }

    /// Clears internal data for study.
    #[inline]
    pub fn clear_study_data(&mut self) {
        self.lstm.clear_study_data();
        self.output_layer.clear_study_data();
    }

    /// Writes information on [`MlDecoderCache`] for [`study()`](Self::study()).
    ///
    /// - `input` : Input.
    /// - `prev_state` : Previous state.
    /// - `output_len` : Length of output sequence.
    /// - `cache` : Cache.
    pub fn ready(
        &mut self,
        input: &MathVec<IN>,
        prev_state: &MathVec<MIDDLE>,
        output_len: usize,
        cache: &mut MlDecoderCache<OUT, MIDDLE, IN>
    ) {
        cache.caches_len = output_len;
        if cache.caches.len() < output_len {
            cache.caches.resize(
                output_len,
                (
                    MlLstmStateCache::<MIDDLE, IN>::new(),
                    MlLstmOutputCache::<MIDDLE, IN>::new(),
                    MlCache::<OUT, MIDDLE>::new()
                )
            );
        }

        self.prev_state.copy_from(prev_state);

        cache.caches_mut().iter_mut().for_each(|(
            lstm_state_cache,
            lstm_output_cache,
            output_layer_cache
        )| {
            self.lstm.ready_state_cache(
                input,
                &self.prev_state,
                lstm_state_cache
            );
            self.prev_state.copy_from(&lstm_state_cache.state);

            self.lstm.ready_output_cache(
                lstm_state_cache,
                lstm_output_cache
            );

            self.output_layer.ready(
                &lstm_output_cache.output,
                None,
                output_layer_cache
            );
        });
    }

    /// Accumulates gradient.
    ///
    /// - `output_error` : Backpropagated ouput error.
    /// - `cache` : Cache.
    /// - `input_error` : Error to backpropagate for previous output error.
    /// - `prev_state_error` : Error to backpropagate for previous state error.
    pub fn study(
        &mut self,
        output_error: &[MathVec<OUT>],
        cache: &MlDecoderCache<OUT, MIDDLE, IN>,
        input_error: &mut MathVec<IN>,
        prev_state_error: &mut MathVec<MIDDLE>
    ) {
        self.tmp_state_error.clear();
        input_error.clear();

        cache.caches().iter().zip(
            output_error.iter()
        ).rev().for_each(|(
            (
                lstm_state_cache,
                lstm_output_cache,
                output_layer_cache
            ),
            output_error_one
        )| {
            self.output_layer.study(
                output_error_one,
                None,
                &output_layer_cache,
                &mut self.tmp_middle_output_error,
                None
            );

            self.lstm.study(
                &self.tmp_middle_output_error,
                &self.tmp_state_error,
                lstm_state_cache,
                lstm_output_cache,
                &mut self.tmp_input_error,
                prev_state_error
            );
            self.tmp_state_error.copy_from(prev_state_error);
            *input_error += &self.tmp_input_error;
        });
    }

    /// Updates weights.
    ///
    /// - `rate` : Learning rate.
    #[inline]
    pub fn update(&mut self, rate: f32) {
        self.lstm.update(rate);
        self.output_layer.update(rate);
    }

    /// Accesses each immutable total gradient with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_total_grad<F>(
        &self,
        mut f: F
    ) where F: FnMut(&f32) {
        self.lstm.for_each_total_grad(|val| {f(val)});
        self.output_layer.total_grad.iter().for_each(|val| {f(val)});
    }

    /// Accesses each mutable total gradient with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_total_grad_mut<F>(
        &mut self,
        mut f: F
    ) where F: FnMut(&mut f32) {
        self.lstm.for_each_total_grad_mut(|val| {f(val)});
        self.output_layer.total_grad.iter_mut().for_each(|val| {f(val)});
    }
}

/// Seq2Seq AI.
///
/// - `OUT` : Dimension of output.
/// - `MIDDLE` : Dimension of hidden layer.
/// - `IN` : Dimension of input.
///
/// # Example
///
/// ChatCHOBIT
/// - If inputs Japanese word, outputs "これは日本語です。"
/// - If inputs English word, outputs "This is English."
///
/// (1) Defines word generator and messages.
///
/// ```ignore
/// extern crate chobitlibs;
/// 
/// use chobitlibs::chobit_ai::{
///     MathVec,
///     Activation,
///     ChobitSeqAi,
///     ChobitMlSeqAi,
///     MlSeqAiCache
/// };
/// 
/// use chobitlibs::chobit_rand::ChobitRand;
/// 
/// fn japanese_letter(rng: &mut ChobitRand) -> char {
///     let letters = [
///         'あ', 'い', 'う', 'え', 'お',
///         'か', 'き', 'く', 'け', 'こ',
///         'さ', 'し', 'す', 'せ', 'そ'
///     ];
/// 
///     letters[(rng.next_u64() as usize) % letters.len()]
/// }
/// 
/// fn english_letter(rng: &mut ChobitRand) -> char {
///     let letters = [
///         'a', 'b', 'c', 'd', 'e',
///         'f', 'g', 'h', 'i', 'j',
///         'k', 'l', 'm', 'n', 'o'
///     ];
/// 
///     letters[(rng.next_u64() as usize) % letters.len()]
/// }
/// 
/// fn gen_word(
///     f: fn(&mut ChobitRand) -> char,
///     rng: &mut ChobitRand,
///     max_len: usize
/// ) -> String {
///     let word_len = ((rng.next_u64() as usize) % max_len) + 1;
/// 
///     let mut ret = String::with_capacity(word_len);
/// 
///     for _ in 0..word_len {
///         ret.push(f(rng));
///     }
/// 
///     ret
/// }
/// 
/// fn write_string_to_slice(string: &str, slice: &mut [MathVec<32>]) {
///     string.chars().zip(slice.iter_mut()).for_each(|(c, s)| {
///         s.load_u32_label(c as u32);
///     });
/// }
/// 
/// const JAPANESE_MESSAGE: &str = "これは日本語です。";
/// const ENGLISH_MESSAGE: &str = "This is English.";
/// ```
///
/// (2) Creates [`ChobitSeqAi`] and randomises weights.
///
/// ```ignore
/// const OUT: usize = 32;
/// const MIDDLE: usize = 64;
/// const IN: usize = 32;
///
/// const MAX_WORD_LEN: usize = 10;
/// let max_message_len = JAPANESE_MESSAGE.len().max(ENGLISH_MESSAGE.len());
///
/// let mut rng = ChobitRand::new(b"ChobitSeqAi Example");
///
/// let mut ai = ChobitSeqAi::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
///
/// // Randomises weights.
/// ai.for_each_weight_mut(|weight| {
///     *weight = ((rng.next_f64() as f32) * 2.0) - 1.0;
/// });
///
/// let mut input = vec![MathVec::<IN>::new(); MAX_WORD_LEN];
/// let mut output = vec![MathVec::<OUT>::new(); max_message_len];
/// let initial_state = MathVec::<MIDDLE>::new();
/// ```
///
/// (3) Wraps AI with [`ChobitMlSeqAi`] for machine learning.
///
/// ```ignore
/// let mut ai = ChobitMlSeqAi::<OUT, MIDDLE, IN>::new(ai);
/// let mut cache = MlSeqAiCache::<OUT, MIDDLE, IN>::new(
///     MAX_WORD_LEN,
///     max_message_len
/// );
///
/// let mut input_error = vec![MathVec::<IN>::new(); MAX_WORD_LEN];
/// let mut output_error = vec![MathVec::<OUT>::new(); max_message_len];
/// let mut prev_state_error = MathVec::<MIDDLE>::new();
/// ```
///
/// (4) Machine learning.
///
/// ```ignore
/// const EPOCH: usize = 10000;
/// const BATCH_SIZE: usize = 10;
/// const RATE: f32 = 0.01;
///
/// let japanese_message_len = JAPANESE_MESSAGE.chars().count();
/// let english_message_len = ENGLISH_MESSAGE.chars().count();
///
/// for _ in 0..EPOCH {
///     for _ in 0..BATCH_SIZE {
///         //--- Learns Japanese ---//
///         let string = gen_word(japanese_letter, &mut rng, MAX_WORD_LEN);
///
///         write_string_to_slice(&string, &mut input);
///         write_string_to_slice(&JAPANESE_MESSAGE, &mut output);
///
///         // Writes cache.
///         ai.ready(
///             &input[..string.chars().count()],
///             &initial_state,
///             japanese_message_len,
///             &mut cache
///         );
///
///         // Calculates error.
///         cache.calc_output_error(
///             &output[..japanese_message_len],
///             &mut output_error[..japanese_message_len]
///         );
///
///         // Studies.
///         ai.study(
///             &output_error[..japanese_message_len],
///             &cache,
///             &mut input_error,
///             &mut prev_state_error
///         );
///
///         //--- Learns English ---//
///         let string = gen_word(english_letter, &mut rng, MAX_WORD_LEN);
///
///         write_string_to_slice(&string, &mut input);
///         write_string_to_slice(&ENGLISH_MESSAGE, &mut output);
///
///         // Writes cache.
///         ai.ready(
///             &input[..string.chars().count()],
///             &initial_state,
///             english_message_len,
///             &mut cache
///         );
///
///         // Calculates error.
///         cache.calc_output_error(
///             &output[..english_message_len],
///             &mut output_error[..english_message_len]
///         );
///
///         // Studies.
///         ai.study(
///             &output_error[..english_message_len],
///             &cache,
///             &mut input_error,
///             &mut prev_state_error
///         );
///     }
///
///     // Updates weights.
///     ai.update(RATE);
/// }
/// ```
///
/// (5) Tests AI.
///
/// ```ignore
/// // Unwrap AI.
/// let mut ai = ai.drop();
///
/// let mut output = MathVec::<OUT>::new();
///
/// // Tests Japanese.
/// for _ in 0..10 {
///     let string = gen_word(japanese_letter, &mut rng, MAX_WORD_LEN);
///
///     write_string_to_slice(&string, &mut input);
///
///     // Initializes state.
///     ai.state_mut().copy_from(&initial_state);
///     
///     // Inputs for each one.
///     input.iter().for_each(|input_one| {
///         ai.input_next(input_one);
///     });
///
///     // Outputs for each one.
///     JAPANESE_MESSAGE.chars().for_each(|c| {
///         ai.output_next(&mut output);
///
///         assert_eq!(output.to_u32_label(), c as u32);
///     });
/// }
///
/// // Tests English.
/// for _ in 0..10 {
///     let string = gen_word(english_letter, &mut rng, MAX_WORD_LEN);
///
///     write_string_to_slice(&string, &mut input);
///
///     // Initializes state.
///     ai.state_mut().copy_from(&initial_state);
///     
///     // Inputs for each one.
///     input.iter().for_each(|input_one| {
///         ai.input_next(input_one);
///     });
///
///     // Outputs for each one.
///     ENGLISH_MESSAGE.chars().for_each(|c| {
///         ai.output_next(&mut output);
///
///         assert_eq!(output.to_u32_label(), c as u32);
///     });
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ChobitSeqAi<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> {
    enc_layer: Lstm<MIDDLE, IN>,
    dec_layer: Lstm<MIDDLE, MIDDLE>,
    output_layer: Layer<OUT, MIDDLE>,

    prev_state: MathVec<MIDDLE>,
    state: MathVec<MIDDLE>,
    enc_output: MathVec<MIDDLE>,
    dec_output: MathVec<MIDDLE>,

    tmpbuf: MathVec<MIDDLE>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> ChobitSeqAi<OUT, MIDDLE, IN> {
    /// Creates ChobitSeqAi.
    ///
    /// - `activation` : Activation function for output layer.
    /// - _Return_ : ChobitSeqAi.
    #[inline]
    pub fn new(activation: Activation) -> Self {
        Self {
            enc_layer: Lstm::<MIDDLE, IN>::new(),
            dec_layer: Lstm::<MIDDLE, MIDDLE>::new(),
            output_layer: Layer::<OUT, MIDDLE>::new(activation, false),

            prev_state: MathVec::<MIDDLE>::new(),
            state: MathVec::<MIDDLE>::new(),
            enc_output: MathVec::<MIDDLE>::new(),
            dec_output: MathVec::<MIDDLE>::new(),

            tmpbuf: MathVec::<MIDDLE>::new()
        }
    }

    /// Gets immutable encoding layer of Lstm.
    ///
    /// - _Return_ : Lstm.
    #[inline]
    pub fn enc_layer(&self) -> &Lstm<MIDDLE, IN> {&self.enc_layer}

    /// Gets mutable encoding layer of Lstm.
    ///
    /// - _Return_ : Lstm.
    #[inline]
    pub fn enc_layer_mut(&mut self) -> &mut Lstm<MIDDLE, IN> {
        &mut self.enc_layer
    }

    /// Gets immutable decoding layer of Lstm.
    ///
    /// - _Return_ : Lstm.
    #[inline]
    pub fn dec_layer(&self) -> &Lstm<MIDDLE, MIDDLE> {&self.dec_layer}

    /// Gets mutable decoding layer of Lstm.
    ///
    /// - _Return_ : Lstm.
    #[inline]
    pub fn dec_layer_mut(&mut self) -> &mut Lstm<MIDDLE, MIDDLE> {
        &mut self.dec_layer
    }

    /// Gets immutable output layer.
    ///
    /// - _Return_ : Output layer.
    #[inline]
    pub fn output_layer(&self) -> &Layer<OUT, MIDDLE> {&self.output_layer}

    /// Gets mutable output layer.
    ///
    /// - _Return_ : Output layer.
    #[inline]
    pub fn output_layer_mut(&mut self) -> &mut Layer<OUT, MIDDLE> {
        &mut self.output_layer
    }

    /// Gets immutable state.
    ///
    /// This should be initialized before the first [`input_next()`](Self::input_next).
    ///
    /// - _Return_ : State.
    #[inline]
    pub fn state(&self) -> &MathVec<MIDDLE> {&self.state}

    /// Gets mutable state.
    ///
    /// This should be initialized before the first [`input_next()`](Self::input_next).
    ///
    /// - _Return_ : State.
    #[inline]
    pub fn state_mut(&mut self) -> &mut MathVec<MIDDLE> {
        &mut self.state
    }

    /// Input next data.
    ///
    /// - `input` : Next data.
    #[inline]
    pub fn input_next(&mut self, input: &MathVec<IN>) {
        self.prev_state.copy_from(&self.state);

        self.enc_layer.calc(
            input,
            &self.prev_state,
            &mut self.enc_output,
            &mut self.state,
            &mut self.tmpbuf
        );
    }

    /// Output next data calculated by current state and input.
    ///
    /// - `output` : Buffer for next data.
    #[inline]
    pub fn output_next(&mut self, output: &mut MathVec<OUT>) {
        self.prev_state.copy_from(&self.state);

        self.dec_layer.calc(
            &self.enc_output,
            &self.prev_state,
            &mut self.dec_output,
            &mut self.state,
            &mut self.tmpbuf
        );

        self.output_layer.calc(&self.dec_output, None, output);
    }

    /// Accesses each immutable weight with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_weight<F>(&self, mut f: F) where F: FnMut(&f32) {
        self.enc_layer.for_each_weight(|val| {f(val)});
        self.dec_layer.for_each_weight(|val| {f(val)});
        self.output_layer.weights().iter().for_each(|val| {f(val)});
    }

    /// Accesses each mutable weight with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_weight_mut<F>(
        &mut self,
        mut f: F
    ) where F: FnMut(&mut f32) {
        self.enc_layer.for_each_weight_mut(|val| {f(val)});
        self.dec_layer.for_each_weight_mut(|val| {f(val)});
        self.output_layer.mut_weights().iter_mut().for_each(|val| {f(val)});
    }
}

/// Cache for [`ChobitMlSeqAi`].
///
/// - `OUT` : `OUT` of [`ChobitMlSeqAi`].
/// - `MIDDLE` : `MIDDLE` of [`ChobitMlSeqAi`].
/// - `IN` : `IN` of [`ChobitMlSeqAi`].
#[derive(Debug, Clone, PartialEq)]
pub struct MlSeqAiCache<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> {
    enc_state_caches: Vec<MlLstmStateCache<MIDDLE, IN>>,
    enc_state_caches_len: usize,
    enc_output_cache: Option<MlLstmOutputCache<MIDDLE, IN>>,

    dec_caches: Vec<(
        MlLstmStateCache<MIDDLE, MIDDLE>,
        MlLstmOutputCache<MIDDLE, MIDDLE>,
        MlCache<OUT, MIDDLE>
    )>,
    dec_caches_len: usize
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> MlSeqAiCache<OUT, MIDDLE, IN> {
    /// Creates MlSeqAiCache.
    ///
    /// - `input_capacity` : Capacity of input caches.
    /// - `output_capacity` : Capacity of output caches.
    /// - _Return_ : MlSeqAiCache.
    #[inline]
    pub fn new(input_capacity: usize, output_capacity: usize) -> Self {
        Self {
            enc_state_caches: vec![
                MlLstmStateCache::<MIDDLE, IN>::new(); input_capacity
            ],
            enc_state_caches_len: 0,

            enc_output_cache: Some(MlLstmOutputCache::<MIDDLE, IN>::new()),

            dec_caches: vec![
                (
                    MlLstmStateCache::<MIDDLE, MIDDLE>::new(),
                    MlLstmOutputCache::<MIDDLE, MIDDLE>::new(),
                    MlCache::<OUT, MIDDLE>::new(),
                ); output_capacity
            ],
            dec_caches_len: 0,
        }
    }

    /// Calculates output error.
    ///
    /// | Formula |
    /// |:-:|
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mrow> <mi>e</mi> <mo stretchy="false">=</mo> <mrow> <mi>o</mi> <mo stretchy="false">−</mo> <mi>t</mi> </mrow> </mrow> </semantics> </math> |
    /// | <math xmlns="http://www.w3.org/1998/Math/MathMl" display="block"> <semantics> <mtable columnalign="left"> <mtr> <mtd> <mrow> <mi>e</mi> <mo stretchy="false">≝</mo> <mtext>Error.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>o</mi> <mo stretchy="false">≝</mo> <mtext>Actual output.</mtext> </mrow> </mtd> </mtr> <mtr> <mtd> <mrow> <mi>t</mi> <mo stretchy="false">≝</mo> <mtext>Correct output.</mtext> </mrow> </mtd> </mtr> </mtable> </semantics> </math> |
    ///
    /// - `train_out` : Correct output.
    /// - `output_error` : Buffer for output error.
    #[inline]
    pub fn calc_output_error(
        &self,
        train_out: &[MathVec<OUT>],
        output_error: &mut [MathVec<OUT>]
    ) {
        train_out.iter().zip(
            &self.dec_caches
        ).zip(
            output_error
        ).for_each(
            |((train_out_one, (_, _, cache)), output_error_one)| {
                output_error_one.copy_from(&cache.output);
                *output_error_one -= train_out_one;
            }
        );
    }

    /// Gets slice of ['MlLstmStateCache'] for encoding layer.
    ///
    /// - _Return_ : slice of [`MlLstmStateCache`].
    #[inline]
    pub fn enc_state_caches(&self) -> &[MlLstmStateCache<MIDDLE, IN>] {
        &self.enc_state_caches[..self.enc_state_caches_len]
    }


    /// Gets ['MlLstmOutputCache'] for encoding layer.
    ///
    /// - _Return_ : [`MlLstmOutputCache`].
    #[inline]
    pub fn enc_output_cache(&self) -> &MlLstmOutputCache<MIDDLE, IN> {
        self.enc_output_cache.as_ref().unwrap()
    }

    #[inline]
    pub(self) fn take_enc_output_cache(
        &mut self
    ) -> Option<MlLstmOutputCache<MIDDLE, IN>> {
        self.enc_output_cache.take()
    }

    #[inline]
    pub(self) fn set_enc_output_cache(
        &mut self,
        output_cache: MlLstmOutputCache<MIDDLE, IN>
    ) {
        self.enc_output_cache = Some(output_cache);
    }

    /// Gets slice of caches for decoding layer.
    ///
    /// - _Return_ : slice of caches.
    #[inline]
    pub fn dec_caches(&self) -> &[(
        MlLstmStateCache<MIDDLE, MIDDLE>,
        MlLstmOutputCache<MIDDLE, MIDDLE>,
        MlCache<OUT, MIDDLE>
    )] {
        &self.dec_caches[..self.dec_caches_len]
    }

    #[inline]
    pub(self) fn dec_caches_mut(&mut self) -> &mut [(
        MlLstmStateCache<MIDDLE, MIDDLE>,
        MlLstmOutputCache<MIDDLE, MIDDLE>,
        MlCache<OUT, MIDDLE>
    )] {
        &mut self.dec_caches[..self.dec_caches_len]
    }

    #[inline]
    pub(self) fn enc_last_state_cache(
        &self
    ) -> Option<&MlLstmStateCache<MIDDLE, IN>> {
        self.enc_state_caches.get(self.enc_state_caches_len.wrapping_sub(1))
    }

    /// Accesses each output with closure.
    ///
    /// - `f` : Closure.
    pub fn for_each_output<F>(&self, mut f: F) where F: FnMut(&MathVec<OUT>) {
        self.dec_caches[..self.dec_caches_len].iter().for_each(
            |(_, _, cache)| {f(&cache.output);}
        );
    }
}

/// Wrapper of [`ChobitSeqAi`] for machine learning.
///
/// See [`ChobitSeqAi`] for details.
///
/// - `OUT` : Dimension of output.
/// - `MIDDLE` : Dimension of hidden layer.
/// - `IN` : Dimension of input.
#[derive(Debug, Clone, PartialEq)]
pub struct ChobitMlSeqAi<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> {
    enc_layer: MlLstm<MIDDLE, IN>,
    dec_layer: MlLstm<MIDDLE, MIDDLE>,
    output_layer: MlLayer<OUT, MIDDLE>,

    tmp_prev_state: MathVec<MIDDLE>,
    tmp_state_error: MathVec<MIDDLE>,
    tmp_enc_output: MathVec<MIDDLE>,
    tmp_output_error: Vec<MathVec<OUT>>,
    tmp_dec_output_error: MathVec<MIDDLE>,
    tmp_enc_output_error: MathVec<MIDDLE>,
    tmp_enc_output_error_one: MathVec<MIDDLE>,

    original_prev_state: MathVec<MIDDLE>,
    original_state: MathVec<MIDDLE>,
    original_enc_output: MathVec<MIDDLE>,
    original_dec_output: MathVec<MIDDLE>,

    original_tmpbuf: MathVec<MIDDLE>
}

impl<
    const OUT: usize,
    const MIDDLE: usize,
    const IN: usize
> ChobitMlSeqAi<OUT, MIDDLE, IN> {
    /// Creates ChobitMlSeqAi.
    ///
    /// - `ai` : Base [`ChobitSeqAi`].
    /// - _Return_ : ChobitMlSeqAi.
    #[inline]
    pub fn new(ai: ChobitSeqAi<OUT, MIDDLE, IN>) -> Self {
        let ChobitSeqAi::<OUT, MIDDLE, IN> {
            enc_layer,
            dec_layer,
            output_layer,
            prev_state,
            state,
            enc_output,
            dec_output,
            tmpbuf
        } = ai;

        Self {
            enc_layer: MlLstm::<MIDDLE, IN>::new(enc_layer),
            dec_layer: MlLstm::<MIDDLE, MIDDLE>::new(dec_layer),
            output_layer: MlLayer::<OUT, MIDDLE>::new(output_layer),

            tmp_prev_state: MathVec::<MIDDLE>::new(),
            tmp_state_error: MathVec::<MIDDLE>::new(),
            tmp_enc_output: MathVec::<MIDDLE>::new(),
            tmp_output_error: Vec::<MathVec<OUT>>::new(),
            tmp_dec_output_error: MathVec::<MIDDLE>::new(),
            tmp_enc_output_error: MathVec::<MIDDLE>::new(),
            tmp_enc_output_error_one: MathVec::<MIDDLE>::new(),

            original_prev_state: prev_state,
            original_state: state,
            original_enc_output: enc_output,
            original_dec_output: dec_output,
            original_tmpbuf: tmpbuf
        }
    }

    /// Drops base [`ChobitSeqAi`].
    ///
    /// - _Return_ : [`ChobitSeqAi`].
    #[inline]
    pub fn drop(self) -> ChobitSeqAi<OUT, MIDDLE, IN> {
        let ChobitMlSeqAi::<OUT, MIDDLE, IN> {
            enc_layer,
            dec_layer,
            output_layer,
            original_prev_state,
            original_state,
            original_enc_output,
            original_dec_output,
            original_tmpbuf,
            ..
        } = self;

        ChobitSeqAi::<OUT, MIDDLE, IN> {
            enc_layer: enc_layer.drop(),
            dec_layer: dec_layer.drop(),
            output_layer: output_layer.drop(),

            prev_state: original_prev_state,
            state: original_state,
            enc_output: original_enc_output,
            dec_output: original_dec_output,

            tmpbuf: original_tmpbuf
        }
    }

    /// Clears internal data for study.
    #[inline]
    pub fn clear_study_data(&mut self) {
        self.enc_layer.clear_study_data();
        self.dec_layer.clear_study_data();
        self.output_layer.clear_study_data();
    }

    /// Writes information on [`MlSeqAiCache`] for [`study()`](Self::study()).
    ///
    /// - `input` : Input.
    /// - `prev_state` : Previous state.
    /// - `output_len` : Length of output sequence.
    /// - `cache` : Cache.
    #[inline]
    pub fn ready(
        &mut self,
        input: &[MathVec<IN>],
        prev_state: &MathVec<MIDDLE>,
        output_len: usize,
        cache: &mut MlSeqAiCache<OUT, MIDDLE, IN>
    ) {
        self.ready_enc_layer(input, prev_state, cache);
        self.ready_dec_layer(output_len, cache);
    }

    fn ready_enc_layer(
        &mut self,
        input: &[MathVec<IN>],
        prev_state: &MathVec<MIDDLE>,
        cache: &mut MlSeqAiCache<OUT, MIDDLE, IN>
    ) {
        cache.enc_state_caches_len = input.len();
        if cache.enc_state_caches.len() < input.len() {
            cache.enc_state_caches.resize(
                input.len(),
                MlLstmStateCache::<MIDDLE, IN>::new(),
            );
        }

        self.tmp_prev_state.copy_from(prev_state);

        input.iter().zip(
            cache.enc_state_caches.iter_mut()
        ).for_each(|(input_one, cache)| {
            self.enc_layer.ready_state_cache(
                input_one,
                &self.tmp_prev_state,
                cache
            );

            self.tmp_prev_state.copy_from(&cache.state);
        });

        if let Some(mut output_cache) = cache.take_enc_output_cache() {
            if let Some(state_cache) = cache.enc_last_state_cache() {
                self.enc_layer.ready_output_cache(
                    state_cache,
                    &mut output_cache
                );
            }

            cache.set_enc_output_cache(output_cache);
        }
    }

    fn ready_dec_layer(
        &mut self,
        output_len: usize,
        cache: &mut MlSeqAiCache<OUT, MIDDLE, IN>
    ) {
        cache.dec_caches_len = output_len;
        if cache.dec_caches.len() < output_len {
            cache.dec_caches.resize(
                output_len,
                (
                    MlLstmStateCache::<MIDDLE, MIDDLE>::new(),
                    MlLstmOutputCache::<MIDDLE, MIDDLE>::new(),
                    MlCache::<OUT, MIDDLE>::new()
                )
            );
        }

        match cache.enc_last_state_cache() {
            Some(state_cache) => {
                self.tmp_prev_state.copy_from(&state_cache.state);
            },

            None => {self.tmp_prev_state.clear();}
        }

        if let Some(enc_output_cache) = cache.take_enc_output_cache() {
            cache.dec_caches_mut().iter_mut().for_each(
                |(dec_state_cache, dec_output_cache, output_layer_cache)| {
                    self.dec_layer.ready_state_cache(
                        &enc_output_cache.output,
                        &self.tmp_prev_state,
                        dec_state_cache
                    );

                    self.dec_layer.ready_output_cache(
                        dec_state_cache,
                        dec_output_cache
                    );

                    self.output_layer.ready(
                        &dec_output_cache.output,
                        None,
                        output_layer_cache
                    );

                    self.tmp_prev_state.copy_from(&dec_state_cache.state);
                }
            );

            cache.set_enc_output_cache(enc_output_cache);
        }
    }

    /// Accumulates gradient.
    ///
    /// - `output_error` : Backpropagated ouput error.
    /// - `cache` : Cache.
    /// - `input_error` : Error to backpropagate for previous output error.
    /// - `prev_state_error` : Error to backpropagate for previous state error.
    pub fn study(
        &mut self,
        output_error: &[MathVec<OUT>],
        cache: &MlSeqAiCache<OUT, MIDDLE, IN>,
        input_error: &mut [MathVec<IN>],
        prev_state_error: &mut MathVec<MIDDLE>,
    ) {
        self.tmp_state_error.clear();
        self.tmp_enc_output_error.clear();

        cache.dec_caches().iter().zip(
            output_error.iter()
        ).rev().for_each(
            |(
                (dec_state_cache, dec_output_cache, output_layer_cache),
                output_error_one
            )| {
                self.output_layer.study(
                    output_error_one,
                    None,
                    output_layer_cache,
                    &mut self.tmp_dec_output_error,
                    None
                );

                self.dec_layer.study(
                    &self.tmp_dec_output_error,
                    &self.tmp_state_error,
                    dec_state_cache,
                    dec_output_cache,
                    &mut self.tmp_enc_output_error_one,
                    prev_state_error
                );

                self.tmp_state_error.copy_from(prev_state_error);
                self.tmp_enc_output_error += &self.tmp_enc_output_error_one;
            }
        );

        if let Some(output_cache) = cache.enc_output_cache.as_ref() {
            let mut enc_state_caches_iter =
                cache.enc_state_caches().iter().zip(
                    input_error.iter_mut()
                ).rev();

            if let Some((state_cache, input_error_one)) =
                enc_state_caches_iter.next()
            {
                self.enc_layer.study(
                    &self.tmp_enc_output_error,
                    &self.tmp_state_error,
                    state_cache,
                    output_cache,
                    input_error_one,
                    prev_state_error
                );

                self.tmp_state_error.copy_from(prev_state_error);
            }

            enc_state_caches_iter.for_each(
                |(cache, input_error_one)| {
                    self.enc_layer.study_state(
                        &self.tmp_state_error,
                        cache,
                        input_error_one,
                        prev_state_error
                    );

                    self.tmp_state_error.copy_from(prev_state_error);
                }
            );
        }
    }

    /// Updates weights.
    ///
    /// - `rate` : Learning rate.
    #[inline]
    pub fn update(&mut self, rate: f32) {
        self.enc_layer.update(rate);
        self.dec_layer.update(rate);
        self.output_layer.update(rate);
    }

    /// Accesses each immutable total gradient with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_total_grad<F>(
        &self,
        mut f: F
    ) where F: FnMut(&f32) {
        self.enc_layer.for_each_total_grad(|val| {f(val)});
        self.dec_layer.for_each_total_grad(|val| {f(val)});
        self.output_layer.total_grad.iter().for_each(|val| {f(val)});
    }

    /// Accesses each mutable total gradient with closure.
    ///
    /// - `f` : Closure.
    #[inline]
    pub fn for_each_total_grad_mut<F>(
        &mut self,
        mut f: F
    ) where F: FnMut(&mut f32) {
        self.enc_layer.for_each_total_grad_mut(|val| {f(val)});
        self.dec_layer.for_each_total_grad_mut(|val| {f(val)});
        self.output_layer.total_grad.iter_mut().for_each(|val| {f(val)});
    }
}
