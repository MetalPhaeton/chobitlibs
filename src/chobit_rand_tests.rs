use std::prelude::rust_2021::*;

use crate::chobit_rand::*;

const SEED: [u8; core::mem::size_of::<u64>() * 10] = [
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88
];

#[test]
fn chobit_rand_next_u64_test() {
    const COUNT: usize = 5000;

    let mut rand = ChobitRand::new(&SEED);
    let mut v1 = Vec::<u64>::with_capacity(COUNT);

    for _ in 0..COUNT {
        v1.push(rand.next_u64());
    }

    let v2 = v1.clone();

    for i in 0..COUNT {
        for j in 0..COUNT {
            if i == j {
                assert_eq!(v1[i], v2[j]);
            } else {
                assert_ne!(v1[i], v2[j]);
            }
        }
    }

    const PRINT_COUNT: usize = 10;
    for value in &v1[..PRINT_COUNT] {
        std::println!("chobit_rand_next_u64_test: {}", value)
    }
}

#[test]
fn chobit_rand_next_f64_test() {
    const COUNT: usize = 5000;

    let mut rand = ChobitRand::new(&SEED);
    let mut v1 = Vec::<f64>::with_capacity(COUNT);

    for _ in 0..COUNT {
        v1.push(rand.next_f64());
    }

    let v2 = v1.clone();

    for i in 0..COUNT {
        assert!((v1[i] >= 0.0) && (v1[i] <= 1.0));

        for j in 0..COUNT {
            if i == j {
                assert_eq!(v1[i], v2[j]);
            } else {
                assert_ne!(v1[i], v2[j]);
            }
        }
    }

    const PRINT_COUNT: usize = 10;
    for value in &v1[..PRINT_COUNT] {
        std::println!("chobit_rand_next_f64_test: {}", value)
    }
}

#[test]
fn chobit_rand_shubble_test() {
    const COUNT: usize = 20;

    let mut rand = ChobitRand::new(&SEED);

    let mut cards: Vec<u32> = (1..=13).collect();

    let mut v1 = Vec::<Vec<u32>>::with_capacity(COUNT);
    for _ in 0..COUNT {
        rand.shuffle(&mut cards);

        v1.push(cards.clone());
    }

    let v2 = v1.clone();

    for i in 0..COUNT {
        for j in 0..COUNT {
            if i == j {
                assert_eq!(v1[i], v2[j]);
            } else {
                assert_ne!(v1[i], v2[j]);
            }
        }
    }

    const PRINT_COUNT: usize = 10;
    for value in &v1[..PRINT_COUNT] {
        std::println!("chobit_rand_shuffle_test: {:?}", value)
    }
}
