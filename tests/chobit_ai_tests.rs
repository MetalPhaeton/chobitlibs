extern crate chobitlibs;

use std::prelude::rust_2021::*;

use chobitlibs::chobit_ai::*;
use chobitlibs::chobit_rand::*;

#[test]
fn weights_test() {
    let weights_1 = Weights::<5>::new(
        [1.0, 2.0, 3.0, 4.0, 5.0],
        6.0
    );

    let weights_2 = weights_1;

    let check = Weights::<5>::new(
        [
            1.0 * 2.0,
            2.0 * 2.0,
            3.0 * 2.0,
            4.0 * 2.0,
            5.0 * 2.0
        ],
        6.0 * 2.0
    );

    assert_eq!(weights_1 + weights_2, check);
    assert_eq!((weights_1 * 3.0) - weights_2, check);
    assert_eq!((weights_1 * 4.0) / 2.0, check);

    let mut weights_1_c = weights_1.clone();
    weights_1_c += weights_2;
    assert_eq!(weights_1_c, check);

    let mut weights_1_c = weights_1.clone();
    weights_1_c *= 3.0;
    weights_1_c -= weights_2;
    assert_eq!(weights_1_c, check);

    let mut weights_1_c = weights_1.clone();
    weights_1_c *= 4.0;
    weights_1_c /= 2.0;
    assert_eq!(weights_1_c, check);

    let check = Weights::<5>::new([1.0, 2.0, 3.0, 0.0, 1.0], 2.0);

    assert_eq!(weights_1 % 4.0, check);

    let mut weights_1_c = weights_1.clone();
    weights_1_c %= 4.0;
    assert_eq!(weights_1_c, check);

    let input: [f32; 5] = [1.1, 2.2, 3.3, 4.4, 5.5];

    let check = (1.0 * 1.1)
        + (2.0 * 2.2)
        + (3.0 * 3.3)
        + (4.0 * 4.4)
        + (5.0 * 5.5)
        + 6.0;

    assert_eq!(weights_1 * input, check);

    let weights_3 = Weights::<5>::new([1.1, 2.2, 3.3, 4.4, 5.5], 6.6);

    let check = (1.0 * 1.1)
        + (2.0 * 2.2)
        + (3.0 * 3.3)
        + (4.0 * 4.4)
        + (5.0 * 5.5)
        + (6.0 * 6.6);

    assert_eq!(weights_1 * weights_3, check);
}

//--------------------//
// Data set generator //
//--------------------//
// Output data generator for data set.
// This is like multiply of 2 complex numbers.
fn gen_output(input: &[f32; 4]) -> [f32; 2] {
    [
        (input[0] * input[2]) - (input[1] * input[3]),
        (input[0] * input[3]) + (input[1] * input[2])
    ]
}

// Input data generator for data set.
fn gen_input(rand: &mut ChobitRand) -> [f32; 4] {
    // converter from [0.0, 1.0] to [-1.0, 1.0].
    fn convert(x: f32) -> f32 {(x * 2.0) - 1.0}

    [
        convert(rand.next_f64() as f32),
        convert(rand.next_f64() as f32),
        convert(rand.next_f64() as f32),
        convert(rand.next_f64() as f32)
    ]
}

// Data set generator.
fn gen_data_set(length: usize) -> Vec<([f32; 4], [f32; 2])> {
    let mut ret = Vec::<([f32; 4], [f32; 2])>::with_capacity(length);
    let mut rand = ChobitRand::new("This is a pen!".as_bytes());

    for _ in 0..length {
        let input = gen_input(&mut rand);
        let output = gen_output(&input);

        ret.push((input, output))
    }

    ret
}

#[test]
fn chobit_ai_test_1() {
    //----------------//
    // Ready data set //
    //----------------//
    // Generates data set.
    let length: usize = 128;
    let data_set = gen_data_set(length);

    // Separates data_set into train_data and test_data.
    let length = length / 2;
    let train_data = data_set[..length].to_vec();
    let test_data = data_set[length..].to_vec();

    // Separates train_data into 4 batches.
    let length = length / 4;
    let mut batches = [
        train_data[..length].to_vec(),
        train_data[length..(length * 2)].to_vec(),
        train_data[(length * 2)..(length * 3)].to_vec(),
        train_data[(length * 3)..].to_vec()
    ];

    // Generates random number generator with seed bytes.
    let mut rand = ChobitRand::new("Hello! I love to play game!".as_bytes());

    // this is converter from [0.0, 1.0] into [-1.0, 1.0].
    fn convert(x: f32) -> f32 {(x * 2.0) - 1.0}

    //----------//
    // Ready AI //
    //----------//
    // Decides numbers of input nodes, middle layer nodes, output nodes.
    const IN: usize = 4;
    const MIDDLE: usize = 32;
    const OUT: usize = 2;

    // Gererates weights of output nodes with random numbers.
    let out_weights = [0u8; OUT].map(|_| {
        Weights::<MIDDLE>::new(
            [0u8; MIDDLE].map(|_| {
                convert(rand.next_f64() as f32)
            }),
            convert(rand.next_f64() as f32)  // bias
        )
    });

    // Gererates weights of middle nodes with random numbers.
    let middle_weights = [0u8; MIDDLE].map(|_| {
        Weights::<IN>::new(
            [0u8; IN].map(|_| {
                convert(rand.next_f64() as f32)
            }),
            convert(rand.next_f64() as f32)  // bias
        )
    });

    // Generates output neurons with activate function.
    let out_neurons = out_weights.map(|weights| {
        Neuron::<MIDDLE>::new(weights, Activation::Linear)
    });

    // Generates middle neurons with activate function.
    let middle_neurons = middle_weights.map(|weights| {
        Neuron::<IN>::new(weights, Activation::ReLU)
    });

    // Generates output layer.
    let output_layer = Layer::<OUT, MIDDLE>::new(out_neurons);

    // Generates middle layer.
    let middle_layer = Layer::<MIDDLE, IN>::new(middle_neurons);

    // Generates AI.
    let mut ai = ChobitAI::<OUT, MIDDLE, IN>::new(output_layer, middle_layer);

    //-----------------------//
    // Test without learning //
    //-----------------------//
    // Calculates test inputs.
    let mut ai_output = Vec::<[f32; OUT]>::new();

    test_data.iter().for_each(
        |(input, _)| ai_output.push(ai.calc(input))
    );

    // Calculates loss.
    let mut before_loss: f32 = 0.0;
    for i in 0..test_data.len() {
        for j in 0..OUT {
            before_loss += (ai_output[i][j] - test_data[i].1[j]).abs();
        }
    }
    std::println!("Before learning: {}", before_loss);

    //----------//
    // Learning //
    //----------//
    // Decides epoch and learning rate.
    const EPOCH: usize = 100;
    const RATE: f32 = 0.001;

    // Learns.
    for _ in 0..EPOCH {
        // Gets one batch.
        for batch in &mut batches {
            // Shuffle the batch.
            rand.shuffle(batch);

            // Learns each data.
            for data in batch {
                // Studies gradients. (not update weights yet.)
                ai.study(&data.1, &data.0);
            }

            // Updates weights with gradients.
            ai.update(RATE);
        }
    }

    //---------------------//
    // Test after learning //
    //---------------------//
    // Calculates test inputs.
    let mut ai_output = Vec::<[f32; OUT]>::new();

    test_data.iter().for_each(
        |(input, _)| ai_output.push(ai.calc(input))
    );

    // Calculates loss.
    let mut after_loss: f32 = 0.0;
    for i in 0..test_data.len() {
        for j in 0..OUT {
            after_loss += (ai_output[i][j] - test_data[i].1[j]).abs();
        }
    }
    std::println!("After learning: {}", after_loss);

    // Wishes to pass the following assertion...
    assert!(after_loss < before_loss);
}
