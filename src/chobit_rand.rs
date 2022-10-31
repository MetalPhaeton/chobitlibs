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

//! Random number generator library.

use core::mem::size_of;

/// Random number generator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChobitRand {
    body: [u64; 4]
}

impl ChobitRand {
    /// Creates `ChobitRand`.
    ///
    /// * `seed` : A seed value. Its length must be more than 1.
    /// * _Return_ : Instance.
    #[inline]
    pub fn new(seed: &[u8]) -> Self {
        let mut ret = Self {body: Self::to_64_array(seed)};

        let _ = ret.next_u64();

        ret
    }

    fn to_64_array(seed: &[u8]) -> [u64; 4] {
        let array = Self::to_u8_array(seed);

        let ptr = array.as_ptr() as *const u64;

        unsafe {
            [
                u64::from_le(*ptr),
                u64::from_le(*ptr.add(1)),
                u64::from_le(*ptr.add(2)),
                u64::from_le(*ptr.add(3))
            ]
        }
    }

    fn to_u8_array(seed: &[u8]) -> [u8; size_of::<u64>() * 4] {
        let mut ret = [0u8; size_of::<u64>() * 4];

        let seed_len = seed.len();

        for i in 0..ret.len() {
            ret[i] = seed[i % seed_len];
        }

        ret
    }

    /// Generates a random `u64`.
    ///
    /// * _Return_ : A random number.
    ///
    /// ```
    /// use chobit::chobit_rand::ChobitRand;
    ///
    /// let mut rand = ChobitRand::new(&[123u8; 100]);
    ///
    /// assert_ne!(rand.next_u64(), rand.next_u64());
    /// ```
    pub fn next_u64(&mut self) -> u64 {
        let ret = self.body[0]
            .wrapping_add(self.body[3])
            .rotate_right(23)
            .wrapping_add(self.body[0]);

        let t = self.body[1] << 17;

        self.body[2] ^= self.body[0];
        self.body[3] ^= self.body[1];
        self.body[1] ^= self.body[2];
        self.body[0] ^= self.body[3];

        self.body[2] ^= t;
        self.body[3] = self.body[3].rotate_left(45);

        ret
    }

    /// Generates a random `f64`.
    ///
    /// * _Return_ : A random number.
    ///
    /// ```
    /// use chobit::chobit_rand::ChobitRand;
    ///
    /// let mut rand = ChobitRand::new(&[123u8; 100]);
    ///
    /// assert_ne!(rand.next_f64(), rand.next_f64());
    /// ```
    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        f64::from_bits(0x3ff0000000000000 | (self.next_u64() >> 12)) - 1.0
    }

    /// Shuffles a slice.
    ///
    /// * slice : A slice that you want to shuffle.
    ///
    /// ```
    /// use chobit::chobit_rand::ChobitRand;
    ///
    /// let mut rand = ChobitRand::new(&[123u8; 100]);
    ///
    /// let cards_1: [i32; 13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    /// let mut cards_2: [i32; 13] = cards_1.clone();
    ///
    /// rand.shuffle(&mut cards_2);
    /// assert_ne!(cards_1, cards_2);
    /// ```
    #[inline]
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let len = slice.len();

        for i in 0..(len - 1) {
            let j = ((self.next_u64() as usize) % (len - i)) + i;
            slice.swap(i, j);
        }
    }
}
