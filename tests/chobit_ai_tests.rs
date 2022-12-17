extern crate chobitlibs;

use std::prelude::rust_2021::*;

use chobitlibs::chobit_ai::*;
use chobitlibs::chobit_rand::*;

#[test]
fn to_from_label_test() {
    const COUNT: usize = 100;

    let mut rng = ChobitRand::new("to_from_label_test".as_bytes());

    macro_rules! to_from_label_test_core {
        ($type:ty, $rng:expr, $to_func:ident, $from_func:ident) => {{
            let label = rng.next_u64() as $type;

            let label_2 = $to_func(&$from_func(label));

            assert_eq!(label, label_2, "{:0128b} \n {:0128b}", label, label_2);
        }};

        (u128, $rng:expr, $to_func:ident, $from_func:ident) => {{
            let label =
                ((rng.next_u64() as u128) << 64) | (rng.next_u64() as u128);

            let label_2 = $to_func(&$from_func(label));

            assert_eq!(label, label_2, "{:0128b} \n {:0128b}", label, label_2);
        }};
    }

    for _ in 0..COUNT {
        to_from_label_test_core!(u8, rng, to_u8_label, from_u8_label);
        to_from_label_test_core!(u16, rng, to_u16_label, from_u16_label);
        to_from_label_test_core!(u32, rng, to_u32_label, from_u32_label);
        to_from_label_test_core!(u64, rng, to_u64_label, from_u64_label);
        to_from_label_test_core!(u128, rng, to_u128_label, from_u128_label);
    }
}

#[test]
fn math_vec_test_1() {
    let mut vec_1 = MathVec::<10>::new();
    vec_1.as_slice_mut().iter_mut().for_each(|x| {*x = 1.0});

    let mut vec_2 = MathVec::<10>::new();
    vec_2.as_slice_mut().iter_mut().for_each(|x| {*x = 2.0});

    let mut vec_3 = MathVec::<10>::new();
    vec_3.as_slice_mut().iter_mut().for_each(|x| {*x = 3.0});

    let vec_4 = &vec_1 + &vec_2;
    assert_eq!(vec_3, vec_4);

    vec_1 +=  &vec_2;
    assert_eq!(vec_1, vec_4);
}

#[test]
fn math_vec_test_2() {
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_slice_mut().iter_mut().for_each(|x| {*x = 2.0});

        let scalar: f32 = 3.0;

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_slice_mut().iter_mut().for_each(|x| {*x = 6.0});

        let vec_3 = &vec_1 * scalar;
        assert_eq!(vec_3, vec_2);

        vec_1 *= scalar;
        assert_eq!(vec_1, vec_2);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_slice_mut().iter_mut().for_each(|x| {*x = 6.0});

        let scalar: f32 = 3.0;

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_slice_mut().iter_mut().for_each(|x| {*x = 2.0});

        let vec_3 = &vec_1 / scalar;
        assert_eq!(vec_3, vec_2);

        vec_1 /= scalar;
        assert_eq!(vec_1, vec_2);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_slice_mut().iter_mut().for_each(|x| {*x = 12.0});

        let scalar: f32 = 10.0;

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_slice_mut().iter_mut().for_each(|x| {*x = 2.0});

        let vec_3 = &vec_1 % scalar;
        assert_eq!(vec_3, vec_2);

        vec_1 %= scalar;
        assert_eq!(vec_1, vec_2);
    }
}

#[test]
fn math_vec_test_3() {
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_slice_mut().iter_mut().for_each(|x| {*x = 2.0});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_slice_mut().iter_mut().for_each(|x| {*x = 3.0});

        let scalar_1: f32 = 2.0 * 3.0 * 10.0;

        let scalar_2 = &vec_1 * &vec_2;
        assert_eq!(scalar_2, scalar_1);
    }
}

#[test]
fn math_vec_test_4() {
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_slice_mut().iter_mut().for_each(|x| {*x = 2.0});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_slice_mut().iter_mut().for_each(|x| {*x = 3.0});

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_slice_mut().iter_mut().for_each(|x| {*x = 6.0});

        let vec_4 = vec_1.pointwise_mul(&vec_2);
        assert_eq!(vec_4, vec_3);

        vec_1.pointwise_mul_assign(&vec_2);
        assert_eq!(vec_1, vec_3);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_slice_mut().iter_mut().for_each(|x| {*x = 6.0});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_slice_mut().iter_mut().for_each(|x| {*x = 3.0});

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_slice_mut().iter_mut().for_each(|x| {*x = 2.0});

        let vec_4 = vec_1.pointwise_div(&vec_2);
        assert_eq!(vec_4, vec_3);

        vec_1.pointwise_div_assign(&vec_2);
        assert_eq!(vec_1, vec_3);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_slice_mut().iter_mut().for_each(|x| {*x = 12.0});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_slice_mut().iter_mut().for_each(|x| {*x = 10.0});

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_slice_mut().iter_mut().for_each(|x| {*x = 2.0});

        let vec_4 = vec_1.pointwise_rem(&vec_2);
        assert_eq!(vec_4, vec_3);

        vec_1.pointwise_rem_assign(&vec_2);
        assert_eq!(vec_1, vec_3);
    }
}

//#[test]
//fn weights_test() {
//    let weights_1 = Weights::<5>::new(
//        &[1.0, 2.0, 3.0, 4.0, 5.0],
//        6.0
//    );
//
//    let weights_2 = weights_1;
//
//    let check = Weights::<5>::new(
//        &[
//            1.0 * 2.0,
//            2.0 * 2.0,
//            3.0 * 2.0,
//            4.0 * 2.0,
//            5.0 * 2.0
//        ],
//        6.0 * 2.0
//    );
//
//    assert_eq!(weights_1 + weights_2, check);
//    assert_eq!((weights_1 * 3.0) - weights_2, check);
//    assert_eq!((weights_1 * 4.0) / 2.0, check);
//
//    let mut weights_1_c = weights_1.clone();
//    weights_1_c += weights_2;
//    assert_eq!(weights_1_c, check);
//
//    let mut weights_1_c = weights_1.clone();
//    weights_1_c *= 3.0;
//    weights_1_c -= weights_2;
//    assert_eq!(weights_1_c, check);
//
//    let mut weights_1_c = weights_1.clone();
//    weights_1_c *= 4.0;
//    weights_1_c /= 2.0;
//    assert_eq!(weights_1_c, check);
//
//    let check = Weights::<5>::new(&[1.0, 2.0, 3.0, 0.0, 1.0], 2.0);
//
//    assert_eq!(weights_1 % 4.0, check);
//
//    let mut weights_1_c = weights_1.clone();
//    weights_1_c %= 4.0;
//    assert_eq!(weights_1_c, check);
//
//    let input: [f32; 5] = [1.1, 2.2, 3.3, 4.4, 5.5];
//
//    let check = (1.0 * 1.1)
//        + (2.0 * 2.2)
//        + (3.0 * 3.3)
//        + (4.0 * 4.4)
//        + (5.0 * 5.5)
//        + 6.0;
//
//    assert_eq!(weights_1 * input, check);
//
//    let weights_3 = Weights::<5>::new(&[1.1, 2.2, 3.3, 4.4, 5.5], 6.6);
//
//    let check = (1.0 * 1.1)
//        + (2.0 * 2.2)
//        + (3.0 * 3.3)
//        + (4.0 * 4.4)
//        + (5.0 * 5.5)
//        + (6.0 * 6.6);
//
//    assert_eq!(weights_1 * weights_3, check);
//}
//
////--------------------//
//// Data set generator //
////--------------------//
//// Output data generator for data set.
//// This is like multiply of 2 complex numbers.
//fn gen_output(input: &[f32; 4]) -> [f32; 2] {
//    [
//        (input[0] * input[2]) - (input[1] * input[3]),
//        (input[0] * input[3]) + (input[1] * input[2])
//    ]
//}
//
//// Input data generator for data set.
//fn gen_input(rand: &mut ChobitRand) -> [f32; 4] {
//    // converter from [0.0, 1.0] to [-1.0, 1.0].
//    fn convert(x: f32) -> f32 {(x * 2.0) - 1.0}
//
//    [
//        convert(rand.next_f64() as f32),
//        convert(rand.next_f64() as f32),
//        convert(rand.next_f64() as f32),
//        convert(rand.next_f64() as f32)
//    ]
//}
//
//// Data set generator.
//fn gen_data_set(length: usize) -> Vec<([f32; 4], [f32; 2])> {
//    let mut ret = Vec::<([f32; 4], [f32; 2])>::with_capacity(length);
//    let mut rand = ChobitRand::new("This is a pen!".as_bytes());
//
//    for _ in 0..length {
//        let input = gen_input(&mut rand);
//        let output = gen_output(&input);
//
//        ret.push((input, output))
//    }
//
//    ret
//}
//
//#[test]
//fn chobit_ai_test_1() {
//    //----------------//
//    // Ready data set //
//    //----------------//
//    // Generates data set.
//    let length: usize = 128;
//    let data_set = gen_data_set(length);
//
//    // Separates data_set into train_data and test_data.
//    let length = length / 2;
//    let train_data = data_set[..length].to_vec();
//    let test_data = data_set[length..].to_vec();
//
//    // Separates train_data into 4 batches.
//    let length = length / 4;
//    let mut batches = [
//        train_data[..length].to_vec(),
//        train_data[length..(length * 2)].to_vec(),
//        train_data[(length * 2)..(length * 3)].to_vec(),
//        train_data[(length * 3)..].to_vec()
//    ];
//
//    // Generates random number generator with seed bytes.
//    let mut rand = ChobitRand::new("Hello! I love to play game!".as_bytes());
//
//    // this is converter from [0.0, 1.0] into [-1.0, 1.0].
//    fn convert(x: f32) -> f32 {(x * 2.0) - 1.0}
//
//    //----------//
//    // Ready AI //
//    //----------//
//    // Decides numbers of input nodes, middle layer nodes, output nodes.
//    const IN: usize = 4;
//    const MIDDLE: usize = 32;
//    const OUT: usize = 2;
//
//    // Gererates weights of output nodes with random numbers.
//    let out_weights = [0u8; OUT].iter().map(|_| {
//        Weights::<MIDDLE>::new(
//            //[0u8; MIDDLE].map(|_| {
//            //    convert(rand.next_f64() as f32)
//            //}),
//            &[0u8; MIDDLE].iter().map(|_| {
//                convert(rand.next_f64() as f32)
//            }).collect::<Vec<f32>>().try_into().unwrap(),
//            convert(rand.next_f64() as f32)  // bias
//        )
//    }).collect::<Vec<Weights<MIDDLE>>>().try_into().unwrap();
//
//    // Gererates weights of middle nodes with random numbers.
//    let middle_weights = [0u8; MIDDLE].iter().map(|_| {
//        Weights::<IN>::new(
//            &[0u8; IN].iter().map(|_| {
//                convert(rand.next_f64() as f32)
//            }).collect::<Vec<f32>>().try_into().unwrap(),
//            convert(rand.next_f64() as f32)  // bias
//        )
//    }).collect::<Vec<Weights<IN>>>().try_into().unwrap();
//
//    // Generates output neurons with activate function.
//    let out_neurons: [Neuron<MIDDLE>; OUT] = out_weights.iter().map(|weights| {
//        Neuron::<MIDDLE>::new(*weights, Activation::Linear)
//    }).collect::<Vec<Neuron<MIDDLE>>>().try_into().unwrap();
//
//    // Generates middle neurons with activate function.
//    let middle_neurons: [Neuron<IN>; MIDDLE] = middle_weights.iter().map(
//        |weights| {
//            Neuron::<IN>::new(*weights, Activation::ReLU)
//        }
//    ).collect::<Vec<Neuron<IN>>>().try_into().unwrap();
//
//    // Generates output layer.
//    let output_layer = Layer::<OUT, MIDDLE>::new(out_neurons);
//
//    // Generates middle layer.
//    let middle_layer = Layer::<MIDDLE, IN>::new(middle_neurons);
//
//    // Generates AI.
//    let mut ai = ChobitAI::<OUT, MIDDLE, IN>::new(output_layer, middle_layer);
//
//    //-----------------------//
//    // Test without learning //
//    //-----------------------//
//    // Calculates test inputs.
//    let mut ai_output = Vec::<[f32; OUT]>::new();
//
//    test_data.iter().for_each(
//        |(input, _)| ai_output.push(ai.calc(input))
//    );
//
//    // Calculates loss.
//    let mut before_loss: f32 = 0.0;
//    for i in 0..test_data.len() {
//        for j in 0..OUT {
//            before_loss += (ai_output[i][j] - test_data[i].1[j]).abs();
//        }
//    }
//    std::println!("Before learning: {}", before_loss);
//
//    //----------//
//    // Learning //
//    //----------//
//    // Decides epoch and learning rate.
//    const EPOCH: usize = 100;
//    const RATE: f32 = 0.001;
//
//    // Learns.
//    for _ in 0..EPOCH {
//        // Gets one batch.
//        for batch in &mut batches {
//            // Shuffle the batch.
//            rand.shuffle(batch);
//
//            // Learns each data.
//            for data in batch {
//                // Studies gradients. (not update weights yet.)
//                ai.study(&data.1, &data.0);
//            }
//
//            // Updates weights with gradients.
//            ai.update(RATE);
//        }
//    }
//
//    //---------------------//
//    // Test after learning //
//    //---------------------//
//    // Calculates test inputs.
//    let mut ai_output = Vec::<[f32; OUT]>::new();
//
//    test_data.iter().for_each(
//        |(input, _)| ai_output.push(ai.calc(input))
//    );
//
//    // Calculates loss.
//    let mut after_loss: f32 = 0.0;
//    for i in 0..test_data.len() {
//        for j in 0..OUT {
//            after_loss += (ai_output[i][j] - test_data[i].1[j]).abs();
//        }
//    }
//    std::println!("After learning: {}", after_loss);
//
//    // Wishes to pass the following assertion...
//    assert!(after_loss < before_loss);
//}
