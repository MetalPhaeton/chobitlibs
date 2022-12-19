extern crate chobitlibs;

use chobitlibs::chobit_ai::*;
use chobitlibs::chobit_rand::*;

use std::mem::size_of;

//==================//
// ChobitAI Example //
//==================//

const IN: usize = size_of::<u32>() * 8;
const MIDDLE: usize = 64;
const OUT: usize = size_of::<u16>() * 8;

// Random number (-1.0, +1.0) -------------------------------------------------

fn rand(rng: &mut ChobitRand) -> f32 {
    ((rng.next_f64() * 2.0) - 1.0) as f32
}

// Generates data set ----------------------------------------------------------

fn gen_data_set(
    rng: &mut ChobitRand,
    size: usize
) -> Vec<(MathVec<OUT>, MathVec<IN>)> {
    let mut ret = Vec::<(MathVec<OUT>, MathVec<IN>)>::with_capacity(size);

    for _ in 0..size {
        let mut vec_out = MathVec::<OUT>::new();
        vec_out.load_u16_label(rng.next_u64() as u16);

        let mut vec_in = MathVec::<IN>::new();
        vec_in.load_u32_label(rng.next_u64() as u32);

        ret.push((vec_out, vec_in));
    }

    ret
}

// Generates AI ---------------------------------------------------------------

fn gen_ai(rng: &mut ChobitRand) -> ChobitAI<OUT, MIDDLE, IN> {
    let mut ret = ChobitAI::<OUT, MIDDLE, IN>::new(Activation::SoftSign);

    // Initializes output layer.
    ret.output_layer_mut().neurons_mut().iter_mut().for_each(
        |neuron| {
            neuron.weights_mut().w_mut().iter_mut().for_each(
                |x| {*x = rand(rng)}
            );

            *neuron.weights_mut().b_mut() = rand(rng);
        }
    );

    // Initializes middle layer.
    ret.middle_layer_mut().neurons_mut().iter_mut().for_each(
        |neuron| {
            neuron.weights_mut().w_mut().iter_mut().for_each(
                |x| {*x = rand(rng)}
            );

            *neuron.weights_mut().b_mut() = rand(rng);
        }
    );

    ret
}

// Prints result --------------------------------------------------------------

fn print_result(
    data: &(MathVec<OUT>, MathVec<IN>),
    ai_output: &MathVec<OUT>
) {
    let input = data.1.to_u32_label();
    let correct = data.0.to_u16_label();
    let ai_result = ai_output.to_u16_label();
    let diff = if correct >= ai_result {
        correct - ai_result
    } else {
        ai_result - correct
    };

    println!(
        "input: {} | correct: {} | ai: {} | diff {}",
        input,
        correct,
        ai_result,
        diff
    );
}

#[test]
fn ai_example() {
    // Generates random number generator.
    let mut rng = ChobitRand::new("This is ChobitAI Example".as_bytes());

    // Generates AI.
    let mut ai = gen_ai(&mut rng);

    // Generates data set.
    const DATA_SET_SIZE: usize = 20;
    let mut data_set = gen_data_set(&mut rng, DATA_SET_SIZE);

    // Prints result before machine learning.
    let mut ai_output = MathVec::<OUT>::new();

    println!("-----------------------");
    println!("Before machine learning");
    println!("-----------------------");
    for data in &data_set {
        ai.calc(&data.1, &mut ai_output);

        print_result(data, &ai_output);
    }

    // Machine learning.
    const EPOCH: usize = 1000;
    const RATE: f32 = 0.01;

    for _ in 0..EPOCH {
        rng.shuffle(&mut data_set);

        for data in &data_set {
            let _ = ai.study(&data.0, &data.1);
        }

        ai.update(RATE);
    }

    // Prints result after machine learning.
    println!("----------------------");
    println!("After machine learning");
    println!("----------------------");
    for data in &data_set {
        ai.calc(&data.1, &mut ai_output);

        print_result(data, &ai_output);
    }
}
