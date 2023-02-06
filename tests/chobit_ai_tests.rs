extern crate chobitlibs;

use chobitlibs::chobit_ai::*;
use chobitlibs::chobit_rand::*;

use std::mem::size_of;
#[test]
fn to_from_label_test() {
    const COUNT: usize = 100;

    let mut rng = ChobitRand::new("to_from_label_test".as_bytes());

    macro_rules! to_from_label_test_core {
        ($type:ty, $rng:expr, $to_func:ident, $from_func:ident) => {{
            let mut vec = MathVec::<{size_of::<$type>() * 8}>::new();
            let label = rng.next_u64() as $type;

            vec.$from_func(label);
            let label_2 = vec.$to_func();

            assert_eq!(label, label_2, "{:0128b} \n {:0128b}", label, label_2);
        }};

        (u128, $rng:expr, $to_func:ident, $from_func:ident) => {{
            let mut vec = MathVec::<{size_of::<$type>() * 8}>::new();
            let label =
                ((rng.next_u64() as u128) << 64) | (rng.next_u64() as u128);

            vec.$from_func(label);
            let label_2 = vec.$to_func();

            assert_eq!(label, label_2, "{:0128b} \n {:0128b}", label, label_2);
        }};
    }

    for _ in 0..COUNT {
        to_from_label_test_core!(u8, rng, to_u8_label, load_u8_label);
        to_from_label_test_core!(u16, rng, to_u16_label, load_u16_label);
        to_from_label_test_core!(u32, rng, to_u32_label, load_u32_label);
        to_from_label_test_core!(u64, rng, to_u64_label, load_u64_label);
        to_from_label_test_core!(u128, rng, to_u128_label, load_u128_label);
    }
}

#[test]
fn math_vec_test_1() {
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 1.0});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});

        let vec_4 = &vec_1 + &vec_2;
        assert_eq!(vec_3, vec_4);

        vec_1 +=  &vec_2;
        assert_eq!(vec_1, vec_4);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 1.0});

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});

        let vec_4 = &vec_1 - &vec_2;
        assert_eq!(vec_3, vec_4);

        vec_1 -=  &vec_2;
        assert_eq!(vec_1, vec_4);
    }
}

#[test]
fn math_vec_test_2() {
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});

        let scalar: f32 = 3.0;

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 6.0});

        let vec_3 = &vec_1 * scalar;
        assert_eq!(vec_3, vec_2);

        vec_1 *= scalar;
        assert_eq!(vec_1, vec_2);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 6.0});

        let scalar: f32 = 3.0;

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});

        let vec_3 = &vec_1 / scalar;
        assert_eq!(vec_3, vec_2);

        vec_1 /= scalar;
        assert_eq!(vec_1, vec_2);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 12.0});

        let scalar: f32 = 10.0;

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});

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
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});

        let scalar_1: f32 = 2.0 * 3.0 * 10.0;

        let scalar_2 = &vec_1 * &vec_2;
        assert_eq!(scalar_2, scalar_1);
    }
}

#[test]
fn math_vec_test_4() {
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_slice().iter_mut().for_each(|x| {*x = 6.0});

        let vec_4 = vec_1.pointwise_mul(&vec_2);
        assert_eq!(vec_4, vec_3);

        vec_1.pointwise_mul_assign(&vec_2);
        assert_eq!(vec_1, vec_3);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 6.0});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});

        let vec_4 = vec_1.pointwise_div(&vec_2);
        assert_eq!(vec_4, vec_3);

        vec_1.pointwise_div_assign(&vec_2);
        assert_eq!(vec_1, vec_3);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 12.0});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 10.0});

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});

        let vec_4 = vec_1.pointwise_rem(&vec_2);
        assert_eq!(vec_4, vec_3);

        vec_1.pointwise_rem_assign(&vec_2);
        assert_eq!(vec_1, vec_3);
    }
}

#[test]
fn math_vec_test_5() {
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 1.0});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});

        vec_1.copy_from(&vec_2);
        assert_eq!(vec_1, vec_2);
    }
}

#[test]
fn weights_test_1() {
    const OUT: usize = 7;
    const IN: usize = 13;
    const BIAS_LEN: usize = OUT;
    const INPUT_WEIGHTS_LEN: usize = OUT * IN;
    const STATE_WEIGHTS_LEN: usize = OUT * OUT;
    const LENGTH: usize = BIAS_LEN + INPUT_WEIGHTS_LEN + STATE_WEIGHTS_LEN;

    {
        let mut weights = Weights::<OUT, IN>::new();
        assert_eq!(weights.as_slice().len(), LENGTH);
        assert_eq!(weights.as_mut_slice().len(), LENGTH);

        let len_b_1 = weights.bias().len();
        let len_b_2 = weights.bias_mut().len();
        assert_eq!(len_b_1, BIAS_LEN);
        assert_eq!(len_b_2, BIAS_LEN);

        let len_i_1 = weights.input_weights().len();
        let len_i_2 = weights.input_weights_mut().len();
        assert_eq!(len_i_1, OUT);
        assert_eq!(len_i_2, OUT);

        let len_i_1 = len_i_1 * weights.input_weights()[0].len();
        let len_i_2 = len_i_2 * weights.input_weights_mut()[0].len();
        assert_eq!(len_i_1, INPUT_WEIGHTS_LEN);
        assert_eq!(len_i_2, INPUT_WEIGHTS_LEN);

        let len_s_1 = weights.state_weights().len();
        let len_s_2 = weights.state_weights_mut().len();
        assert_eq!(len_s_1, OUT);
        assert_eq!(len_s_2, OUT);

        let len_s_1 = len_s_1 * weights.state_weights()[0].len();
        let len_s_2 = len_s_2 * weights.state_weights_mut()[0].len();
        assert_eq!(len_s_1, STATE_WEIGHTS_LEN);
        assert_eq!(len_s_2, STATE_WEIGHTS_LEN);

        assert_eq!(len_b_1 + len_i_1 + len_s_1, LENGTH);
        assert_eq!(len_b_2 + len_i_2 + len_s_2, LENGTH);
    }
    {
        let mut weights = Weights::<OUT, IN>::new();

        let ptr_1 = weights.bias().as_ptr();
        let ptr_2 = weights.bias_mut().as_ptr();
        assert_eq!(ptr_1, ptr_2);

        let ptr_1 = weights.input_weights().as_ptr();
        let ptr_2 = weights.input_weights_mut().as_ptr();
        assert_eq!(ptr_1, ptr_2);

        let ptr_1 = weights.state_weights().as_ptr();
        let ptr_2 = weights.state_weights_mut().as_ptr();
        assert_eq!(ptr_1, ptr_2);
    }
    {
        let weights = Weights::<OUT, IN>::new();
        let whole = weights.as_slice();

        let bias = weights.bias();
        let input_weights = weights.input_weights();
        let state_weights = weights.state_weights();

        let whole_ptr = whole.as_ptr();
        let bias_ptr = bias.as_ptr();
        let input_weights_ptr = input_weights.as_ptr() as *const f32;
        let state_weights_ptr = state_weights.as_ptr() as *const f32;

        assert_eq!(whole_ptr, bias_ptr);
        assert_eq!(unsafe {whole_ptr.add(BIAS_LEN)}, input_weights_ptr);
        assert_eq!(unsafe {bias_ptr.add(BIAS_LEN)}, input_weights_ptr);
        assert_eq!(
            unsafe {whole_ptr.add(BIAS_LEN + INPUT_WEIGHTS_LEN)},
            state_weights_ptr
        );
        assert_eq!(
            unsafe {input_weights_ptr.add(INPUT_WEIGHTS_LEN)},
            state_weights_ptr
        );
        assert_eq!(
            unsafe {whole_ptr.add(LENGTH)},
            unsafe {state_weights_ptr.add(STATE_WEIGHTS_LEN)}
        );

        let weights_2 = weights.clone();
        let whole_2 = weights_2.as_slice();
        assert_eq!(whole_2, whole);
        assert!(
            if (whole_2.as_ptr() as usize) < (whole.as_ptr() as usize) {
                unsafe {
                    (whole_2.as_ptr().add(LENGTH) as usize)
                        < (whole.as_ptr() as usize)
                }
            } else if (whole_2.as_ptr() as usize) > (whole.as_ptr() as usize) {
                unsafe {
                    (whole.as_ptr().add(LENGTH) as usize)
                        < (whole_2.as_ptr() as usize)
                }
            } else {
                false
            }
        );
    }
    {
        let weights = Weights::<OUT, IN>::new();

        let weights_2 = weights.clone();
        let whole = weights_2.as_slice();

        let bias = weights_2.bias();
        let input_weights = weights_2.input_weights();
        let state_weights = weights_2.state_weights();

        let whole_ptr = whole.as_ptr();
        let bias_ptr = bias.as_ptr();
        let input_weights_ptr = input_weights.as_ptr() as *const f32;
        let state_weights_ptr = state_weights.as_ptr() as *const f32;

        assert_eq!(whole_ptr, bias_ptr);
        assert_eq!(unsafe {whole_ptr.add(BIAS_LEN)}, input_weights_ptr);
        assert_eq!(unsafe {bias_ptr.add(BIAS_LEN)}, input_weights_ptr);
        assert_eq!(
            unsafe {whole_ptr.add(BIAS_LEN + INPUT_WEIGHTS_LEN)},
            state_weights_ptr
        );
        assert_eq!(
            unsafe {input_weights_ptr.add(INPUT_WEIGHTS_LEN)},
            state_weights_ptr
        );
        assert_eq!(
            unsafe {whole_ptr.add(LENGTH)},
            unsafe {state_weights_ptr.add(STATE_WEIGHTS_LEN)}
        );
    }
}

#[test]
fn weights_test_2() {
    const OUT: usize = 7;
    const IN: usize = 13;

    const WEIGHT_B: f32 = 1.0;
    const WEIGHT_I: f32 = 2.0;
    const WEIGHT_S: f32 = 3.0;

    let mut weights = Weights::<OUT, IN>::new();
    weights.bias_mut().iter_mut().for_each(|val| *val = WEIGHT_B);

    let mut weight = WEIGHT_I;
    for weights in weights.input_weights_mut() {
        weights.iter_mut().for_each(|val| *val = weight);
        weight += 0.1;
    }

    let mut weight = WEIGHT_S;
    for weights in weights.state_weights_mut() {
        weights.iter_mut().for_each(|val| *val = weight);
        weight += 0.1;
    }

    const VALUE_I: f32 = 5.0;
    const VALUE_S: f32 = 7.0;
    let mut input = MathVec::<IN>::new();
    input.iter_mut().for_each(|val| *val = VALUE_I);

    let mut state = MathVec::<OUT>::new();
    state.iter_mut().for_each(|val| *val = VALUE_S);

    let mut output = MathVec::<OUT>::new();

    weights.calc(&input, Some(&state), &mut output);

    let mut weight_i = WEIGHT_I;
    let mut weight_s = WEIGHT_S;
    for output_val in output.as_slice() {
        let output_val = ((*output_val) * 10.0).round();

        let val_i = weight_i * VALUE_I * (IN as f32);
        let val_s = weight_s * VALUE_S * (OUT as f32);

        let val = val_i + val_s + WEIGHT_B;
        let val = (val * 10.0).round();

        assert_eq!(output_val, val);

        weight_i += 0.1;
        weight_s += 0.1;
    }
}

#[test]
fn weights_test_3() {
    const OUT: usize = 7;
    const IN: usize = 13;

    const WEIGHT_B: f32 = 1.0;
    const WEIGHT_I: f32 = 2.0;
    const WEIGHT_S: f32 = 3.0;

    let mut weights = Weights::<OUT, IN>::new();

    weights.bias_mut().iter_mut().for_each(|val| *val = WEIGHT_B);

    let mut weight = WEIGHT_I;
    for weights in weights.input_weights_mut() {
        weights.iter_mut().for_each(|val| *val = weight);
        weight += 0.1;
    }

    let mut weight = WEIGHT_S;
    for weights in weights.state_weights_mut() {
        weights.iter_mut().for_each(|val| *val = weight);
        weight += 0.1;
    }

    const VALUE_I: f32 = 5.0;
    const VALUE_S: f32 = 7.0;
    let mut input = MathVec::<IN>::new();
    input.iter_mut().for_each(|val| *val = VALUE_I);

    let mut state = MathVec::<OUT>::new();
    state.iter_mut().for_each(|val| *val = VALUE_S);

    let mut feedback = MathVec::<OUT>::new();
    let mut v: f32 = 1.0;
    for feedback_val in feedback.as_mut_slice() {
        *feedback_val = v;

        v += 1.0;
    }
    {
        let mut grad = MathVec::<IN>::new();

        weights.grad_with_input(&feedback, &mut grad);

        for i in 0..grad.len() {
            let mut check: f32 = 0.0;
            for j in 0..feedback.len() {
                check += feedback[j] * weights.input_weights()[j][i];
            }

            assert_eq!(
                (grad[i] * 10.0).round(),
                (check * 10.0).round()
            );
        }
    }
    {
        let mut grad = MathVec::<OUT>::new();

        weights.grad_with_state(&feedback, &mut grad);

        for i in 0..grad.len() {
            let mut check: f32 = 0.0;
            for j in 0..feedback.len() {
                check += feedback[j] * weights.state_weights()[j][i];
            }

            assert_eq!(
                (grad[i] * 10.0).round(),
                (check * 10.0).round()
            );
        }
    }
    {
        let mut grad = Weights::<OUT, IN>::new();

        Weights::grad_with_weights(&feedback, &input, Some(&state), &mut grad);

        assert_eq!(grad.bias(), feedback.as_slice());

        for i in 0..OUT {
            for j in 0..IN {
                let check = feedback[i] * input[j];
                println!("{}, {}", i, j);
                assert_eq!(
                    (grad.input_weights()[i][j] * 10.0).round(),
                    (check * 10.0).round()
                );
            }
        }

        for i in 0..OUT {
            for j in 0..OUT {
                let check = feedback[i] * state[j];
                println!("{}, {}", i, j);
                assert_eq!(
                    (grad.state_weights()[i][j] * 10.0).round(),
                    (check * 10.0).round()
                );
            }
        }
    }
}

//
//#[inline]
//fn rand_num(rng: &mut ChobitRand) -> f32 {
//    ((rng.next_f64() * 2.0) - 1.0) as f32
//}
//
//fn gen_neuron<const N: usize>(
//    rng: &mut ChobitRand,
//) -> Neuron<N> {
//    let mut ret = Neuron::<N>::new(Activation::SoftSign);
//
//    ret.weights_mut().w_mut().iter_mut().for_each(|w| *w = rand_num(rng));
//    *ret.weights_mut().b_mut() = rand_num(rng);
//
//    ret
//}
//
//fn gen_data_set_1<const N: usize>(
//    rng: &mut ChobitRand,
//    size: usize
//) -> Vec<(f32, MathVec<N>)> {
//    let mut param = MathVec::<N>::new();
//    param.iter_mut().for_each(|x| *x = rand_num(rng));
//
//    let mut ret = Vec::<(f32, MathVec<N>)>::with_capacity(size);
//
//    let activation = Activation::SoftSign;
//
//    for _ in 0..size {
//        let mut v = MathVec::<N>::new();
//        v.iter_mut().for_each(|x| *x = rand_num(rng));
//
//        let ans = activation.activate(&param * &v);
//
//        ret.push((ans, v))
//    }
//
//    ret
//}
//
//#[test]
//fn neuron_test() {
//    const N: usize = 10;
//    const DATA_SET_SIZE: usize = 200;
//
//    let mut rng = ChobitRand::new("neuron_test".as_bytes());
//
//    let mut data_set = gen_data_set_1::<N>(&mut rng, DATA_SET_SIZE);
//
//    let mut neuron = gen_neuron::<N>(&mut rng);
//
//    fn print(data_set: &Vec<(f32, MathVec<N>)>, neuron: &mut Neuron<N>) {
//        let mut total: f32 = 0.0;
//
//        for data in data_set {
//            let output = neuron.calc(&data.1);
//
//            let diff = output - data.0;
//
//            total += diff.max(-diff)
//        }
//
//        println!("loss: {}", total / (data_set.len() as f32));
//        println!("----------");
//    }
//
//    print(&data_set, &mut neuron);
//
//    const EPOCH: usize = 3000;
//    const RATE: f32 = 0.01;
//
//    for _ in 0..EPOCH {
//        rng.shuffle(&mut data_set);
//
//        for data in &data_set {
//            let output = neuron.calc(&data.1);
//
//            let diff = output - data.0;
//
//            let _ = neuron.study(diff);
//        }
//
//        neuron.update(RATE);
//    }
//
//    print(&data_set, &mut neuron);
//}
//
//fn gen_matrix<const OUT: usize, const IN: usize>(
//    rng: &mut ChobitRand,
//) -> Vec<MathVec<IN>> {
//    let mut ret = Vec::<MathVec<IN>>::with_capacity(OUT);
//
//    for _ in 0..OUT {
//        let mut vec = MathVec::<IN>::new();
//        vec.iter_mut().for_each(|x| *x = rand_num(rng));
//
//        ret.push(vec);
//    }
//
//    ret
//}
//
//fn gen_data_set_2<const OUT: usize, const IN: usize>(
//    rng: &mut ChobitRand,
//    size: usize
//) -> Vec<(MathVec<OUT>, MathVec<IN>)> {
//    let mut ret = Vec::<(MathVec<OUT>, MathVec<IN>)>::with_capacity(size);
//
//    let param = gen_matrix::<OUT, IN>(rng);
//
//    let activation = Activation::SoftSign;
//
//    for _ in 0..size {
//        let mut train_in = MathVec::<IN>::new();
//        train_in.iter_mut().for_each(|x| *x = rand_num(rng));
//
//        let mut train_out = MathVec::<OUT>::new();
//
//        for i in 0..OUT {
//            train_out[i] = activation.activate(&param[i] * &train_in);
//        }
//
//        ret.push((train_out, train_in));
//    }
//
//    ret
//}
//
//fn gen_layer<const OUT: usize, const IN: usize>(
//    rng: &mut ChobitRand
//) -> Layer<OUT, IN> {
//    let mut ret = Layer::new(Activation::SoftSign);
//
//    ret.neurons_mut().iter_mut().for_each(
//        |neuron| {
//            neuron.weights_mut().w_mut().iter_mut().for_each(
//                |x| *x = rand_num(rng)
//            );
//
//            *neuron.weights_mut().b_mut() = rand_num(rng);
//        }
//    );
//
//    ret
//}
//
//#[test]
//fn layer_test() {
//    const OUT: usize = 15;
//    const IN: usize = 10;
//    const DATA_SET_SIZE: usize = 50;
//
//    let mut rng = ChobitRand::new("layer_test".as_bytes());
//
//    let mut data_set = gen_data_set_2::<OUT, IN>(&mut rng, DATA_SET_SIZE);
//
//    let mut layer = gen_layer::<OUT, IN>(&mut rng);
//
//    fn print(
//        data_set: &Vec<(MathVec<OUT>, MathVec<IN>)>,
//        layer: &mut Layer<OUT, IN>
//    ) {
//        let mut total: f32 = 0.0;
//        let mut output = MathVec::<OUT>::new();
//
//        for data in data_set {
//            output.copy_from(layer.calc(&data.1));
//
//            output -= &data.0;
//            output.iter().for_each(|x| total += (*x).max(-(*x)));
//        }
//
//        println!("loss: {}", total / ((data_set.len() * OUT) as f32));
//        println!("----------");
//    }
//
//    print(&data_set, &mut layer);
//
//    const EPOCH: usize = 5000;
//    const RATE: f32 = 0.01;
//
//    let mut output = MathVec::<OUT>::new();
//    for _ in 0..EPOCH {
//        rng.shuffle(&mut data_set);
//
//        for data in &data_set {
//            output.copy_from(layer.calc(&data.1));
//
//            output -= &data.0;
//
//            let _ = layer.study(&output);
//        }
//
//        layer.update(RATE);
//    }
//
//    print(&data_set, &mut layer);
//}
//
//fn gen_ai<const OUT: usize, const MIDDLE: usize, const IN: usize>(
//    rng: &mut ChobitRand
//) -> ChobitAI<OUT, MIDDLE, IN> {
//    let mut ret = ChobitAI::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//
//    {
//        ret.output_layer_mut().neurons_mut().iter_mut().for_each(
//            |neuron| {
//                let weights = neuron.weights_mut();
//                weights.w_mut().iter_mut().for_each(|x| *x = rand_num(rng));
//                *weights.b_mut() = rand_num(rng);
//            }
//        );
//    }
//
//    {
//        ret.middle_layer_mut().neurons_mut().iter_mut().for_each(
//            |neuron| {
//                let weights = neuron.weights_mut();
//                weights.w_mut().iter_mut().for_each(|x| *x = rand_num(rng));
//                *weights.b_mut() = rand_num(rng);
//            }
//        );
//    }
//
//    ret
//}
//
//#[test]
//fn ai_test() {
//    const OUT: usize = 15;
//    const MIDDLE: usize = 20;
//    const IN: usize = 10;
//    const DATA_SET_SIZE: usize = 50;
//
//    let mut rng = ChobitRand::new("ai_test".as_bytes());
//
//    let mut data_set = gen_data_set_2::<OUT, IN>(&mut rng, DATA_SET_SIZE);
//
//    let mut ai = gen_ai::<OUT, MIDDLE, IN>(&mut rng);
//
//    fn print(
//        data_set: &Vec<(MathVec<OUT>, MathVec<IN>)>,
//        ai: &mut ChobitAI<OUT, MIDDLE, IN>
//    ) {
//        let mut total: f32 = 0.0;
//        let mut output = MathVec::<OUT>::new();
//
//        for data in data_set {
//            output.copy_from(ai.calc(&data.1));
//
//            output -= &data.0;
//            output.iter().for_each(|x| total += (*x).max(-(*x)));
//        }
//
//        println!("loss: {}", total / ((data_set.len() * OUT) as f32));
//        println!("----------");
//    }
//
//    print(&data_set, &mut ai);
//
//    const EPOCH: usize = 2500;
//    const RATE: f32 = 0.02;
//
//    for _ in 0..EPOCH {
//        rng.shuffle(&mut data_set);
//
//        for data in &data_set {
//            let _ = ai.study(&data.0, &data.1);
//        }
//
//        ai.update(RATE);
//    }
//
//    print(&data_set, &mut ai);
//}
//
