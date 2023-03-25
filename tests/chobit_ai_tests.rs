extern crate chobitlibs;

use chobitlibs::chobit_ai::*;
use chobitlibs::chobit_rand::*;

use std::mem::size_of;

#[inline]
fn rand_num(rng: &mut ChobitRand) -> f32 {
    ((rng.next_f64() * 2.0) - 1.0) as f32
}

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

fn rand_math_vec<const N: usize>(rng: &mut ChobitRand, vec: &mut MathVec<N>) {
    vec.iter_mut().for_each(|val| {*val = rand_num(rng)});
}

#[test]
fn math_vec_test_1() {
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_array().iter_mut().for_each(|x| {*x = 1.0;});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_array().iter_mut().for_each(|x| {*x = 2.0;});

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_array().iter_mut().for_each(|x| {*x = 3.0;});

        let vec_4 = &vec_1 + &vec_2;
        assert_eq!(vec_3, vec_4);

        vec_1 +=  &vec_2;
        assert_eq!(vec_1, vec_4);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_array().iter_mut().for_each(|x| {*x = 3.0;});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_array().iter_mut().for_each(|x| {*x = 1.0;});

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_array().iter_mut().for_each(|x| {*x = 2.0;});

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
        vec_1.as_mut_array().iter_mut().for_each(|x| {*x = 2.0;});

        let scalar: f32 = 3.0;

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_array().iter_mut().for_each(|x| {*x = 6.0;});

        let vec_3 = &vec_1 * scalar;
        assert_eq!(vec_3, vec_2);

        vec_1 *= scalar;
        assert_eq!(vec_1, vec_2);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_array().iter_mut().for_each(|x| {*x = 6.0;});

        let scalar: f32 = 3.0;

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_array().iter_mut().for_each(|x| {*x = 2.0;});

        let vec_3 = &vec_1 / scalar;
        assert_eq!(vec_3, vec_2);

        vec_1 /= scalar;
        assert_eq!(vec_1, vec_2);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_array().iter_mut().for_each(|x| {*x = 12.0;});

        let scalar: f32 = 10.0;

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_array().iter_mut().for_each(|x| {*x = 2.0;});

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
        vec_1.as_mut_array().iter_mut().for_each(|x| {*x = 2.0;});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_array().iter_mut().for_each(|x| {*x = 3.0;});

        let scalar_1: f32 = 2.0 * 3.0 * 10.0;

        let scalar_2 = &vec_1 * &vec_2;
        assert_eq!(scalar_2, scalar_1);
    }
}

#[test]
fn math_vec_test_4() {
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_array().iter_mut().for_each(|x| {*x = 2.0;});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_array().iter_mut().for_each(|x| {*x = 3.0;});

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_array().iter_mut().for_each(|x| {*x = 6.0;});

        let vec_4 = vec_1.pointwise_mul(&vec_2);
        assert_eq!(vec_4, vec_3);

        vec_1.pointwise_mul_assign(&vec_2);
        assert_eq!(vec_1, vec_3);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_array().iter_mut().for_each(|x| {*x = 6.0;});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_array().iter_mut().for_each(|x| {*x = 3.0;});

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_array().iter_mut().for_each(|x| {*x = 2.0;});

        let vec_4 = vec_1.pointwise_div(&vec_2);
        assert_eq!(vec_4, vec_3);

        vec_1.pointwise_div_assign(&vec_2);
        assert_eq!(vec_1, vec_3);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_array().iter_mut().for_each(|x| {*x = 12.0;});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_array().iter_mut().for_each(|x| {*x = 10.0;});

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_array().iter_mut().for_each(|x| {*x = 2.0;});

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
        vec_1.as_mut_array().iter_mut().for_each(|x| {*x = 1.0;});

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_array().iter_mut().for_each(|x| {*x = 2.0;});

        vec_1.copy_from(&vec_2);
        assert_eq!(vec_1, vec_2);
    }
}

fn rand_weights<const OUT: usize, const IN: usize>(
    rng: &mut ChobitRand,
    weights: &mut Weights<OUT, IN>
) {
    weights.iter_mut().for_each(|val| {*val = rand_num(rng)});
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
        let mut weights = Weights::<OUT, IN>::new(true);
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

        let len_s_1 = weights.state_weights().unwrap().len();
        let len_s_2 = weights.state_weights_mut().unwrap().len();
        assert_eq!(len_s_1, OUT);
        assert_eq!(len_s_2, OUT);

        let len_s_1 = len_s_1 * weights.state_weights().unwrap()[0].len();
        let len_s_2 = len_s_2 * weights.state_weights_mut().unwrap()[0].len();
        assert_eq!(len_s_1, STATE_WEIGHTS_LEN);
        assert_eq!(len_s_2, STATE_WEIGHTS_LEN);

        assert_eq!(len_b_1 + len_i_1 + len_s_1, LENGTH);
        assert_eq!(len_b_2 + len_i_2 + len_s_2, LENGTH);
    }
    {
        let mut weights = Weights::<OUT, IN>::new(true);

        let ptr_1 = weights.bias().as_ptr();
        let ptr_2 = weights.bias_mut().as_ptr();
        assert_eq!(ptr_1, ptr_2);

        let ptr_1 = weights.input_weights().as_ptr();
        let ptr_2 = weights.input_weights_mut().as_ptr();
        assert_eq!(ptr_1, ptr_2);

        let ptr_1 = weights.state_weights().unwrap().as_ptr();
        let ptr_2 = weights.state_weights_mut().unwrap().as_ptr();
        assert_eq!(ptr_1, ptr_2);
    }
    {
        let weights = Weights::<OUT, IN>::new(true);
        let whole = weights.as_slice();

        let bias = weights.bias();
        let input_weights = weights.input_weights();
        let state_weights = weights.state_weights().unwrap();

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
        let weights = Weights::<OUT, IN>::new(true);

        let weights_2 = weights.clone();
        let whole = weights_2.as_slice();

        let bias = weights_2.bias();
        let input_weights = weights_2.input_weights();
        let state_weights = weights_2.state_weights().unwrap();

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
    const BIAS_LEN: usize = OUT;
    const INPUT_WEIGHTS_LEN: usize = OUT * IN;
    const LENGTH: usize = BIAS_LEN + INPUT_WEIGHTS_LEN;

    {
        let mut weights = Weights::<OUT, IN>::new(false);
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

        assert!(weights.state_weights().is_none());
        assert!(weights.state_weights_mut().is_none());

        assert_eq!(len_b_1 + len_i_1, LENGTH);
        assert_eq!(len_b_2 + len_i_2, LENGTH);
    }
    {
        let mut weights = Weights::<OUT, IN>::new(false);

        let ptr_1 = weights.bias().as_ptr();
        let ptr_2 = weights.bias_mut().as_ptr();
        assert_eq!(ptr_1, ptr_2);

        let ptr_1 = weights.input_weights().as_ptr();
        let ptr_2 = weights.input_weights_mut().as_ptr();
        assert_eq!(ptr_1, ptr_2);
    }
    {
        let weights = Weights::<OUT, IN>::new(false);
        let whole = weights.as_slice();

        let bias = weights.bias();
        let input_weights = weights.input_weights();

        let whole_ptr = whole.as_ptr();
        let bias_ptr = bias.as_ptr();
        let input_weights_ptr = input_weights.as_ptr() as *const f32;

        assert_eq!(whole_ptr, bias_ptr);
        assert_eq!(unsafe {whole_ptr.add(BIAS_LEN)}, input_weights_ptr);
        assert_eq!(unsafe {bias_ptr.add(BIAS_LEN)}, input_weights_ptr);
        assert_eq!(
            unsafe {whole_ptr.add(LENGTH)},
            unsafe {input_weights_ptr.add(INPUT_WEIGHTS_LEN)}
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
        let weights = Weights::<OUT, IN>::new(false);

        let weights_2 = weights.clone();
        let whole = weights_2.as_slice();

        let bias = weights_2.bias();
        let input_weights = weights_2.input_weights();

        let whole_ptr = whole.as_ptr();
        let bias_ptr = bias.as_ptr();
        let input_weights_ptr = input_weights.as_ptr() as *const f32;

        assert_eq!(whole_ptr, bias_ptr);
        assert_eq!(unsafe {whole_ptr.add(BIAS_LEN)}, input_weights_ptr);
        assert_eq!(unsafe {bias_ptr.add(BIAS_LEN)}, input_weights_ptr);
    }
}

#[test]
fn weights_test_3() {
    const OUT: usize = 7;
    const IN: usize = 13;

    const WEIGHT_B: f32 = 1.0;
    const WEIGHT_I: f32 = 2.0;
    const WEIGHT_S: f32 = 3.0;

    let mut weights = Weights::<OUT, IN>::new(true);
    weights.bias_mut().iter_mut().for_each(|val| {*val = WEIGHT_B;});

    let mut weight = WEIGHT_I;
    for weights in weights.input_weights_mut() {
        weights.iter_mut().for_each(|val| {*val = weight;});
        weight += 0.1;
    }

    let mut weight = WEIGHT_S;
    for weights in weights.state_weights_mut().unwrap() {
        weights.iter_mut().for_each(|val| {*val = weight;});
        weight += 0.1;
    }

    const VALUE_I: f32 = 5.0;
    const VALUE_S: f32 = 7.0;
    let mut input = MathVec::<IN>::new();
    input.iter_mut().for_each(|val| {*val = VALUE_I;});

    let mut state = MathVec::<OUT>::new();
    state.iter_mut().for_each(|val| {*val = VALUE_S;});

    let mut output = MathVec::<OUT>::new();

    weights.calc(&input, Some(&state), &mut output);

    let mut weight_i = WEIGHT_I;
    let mut weight_s = WEIGHT_S;
    for output_val in output.as_array() {
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
fn weights_test_4() {
    const OUT: usize = 7;
    const IN: usize = 13;

    const WEIGHT_B: f32 = 1.0;
    const WEIGHT_I: f32 = 2.0;

    let mut weights = Weights::<OUT, IN>::new(false);
    weights.bias_mut().iter_mut().for_each(|val| {*val = WEIGHT_B;});

    let mut weight = WEIGHT_I;
    for weights in weights.input_weights_mut() {
        weights.iter_mut().for_each(|val| {*val = weight;});
        weight += 0.1;
    }

    const VALUE_I: f32 = 5.0;
    const VALUE_S: f32 = 7.0;
    let mut input = MathVec::<IN>::new();
    input.iter_mut().for_each(|val| {*val = VALUE_I;});

    let mut state = MathVec::<OUT>::new();
    state.iter_mut().for_each(|val| {*val = VALUE_S;});

    let mut output = MathVec::<OUT>::new();

    weights.calc(&input, Some(&state), &mut output);

    let mut weight_i = WEIGHT_I;
    for output_val in output.as_array() {
        let output_val = ((*output_val) * 10.0).round();

        let val_i = weight_i * VALUE_I * (IN as f32);

        let val = val_i + WEIGHT_B;
        let val = (val * 10.0).round();

        assert_eq!(output_val, val);

        weight_i += 0.1;
    }
}

#[test]
fn weights_test_5() {
    const OUT: usize = 7;
    const IN: usize = 13;

    const WEIGHT_B: f32 = 1.0;
    const WEIGHT_I: f32 = 2.0;
    const WEIGHT_S: f32 = 3.0;

    let mut weights = Weights::<OUT, IN>::new(true);

    weights.bias_mut().iter_mut().for_each(|val| {*val = WEIGHT_B;});

    let mut weight = WEIGHT_I;
    for weights in weights.input_weights_mut() {
        weights.iter_mut().for_each(|val| {*val = weight;});
        weight += 0.1;
    }

    let mut weight = WEIGHT_S;
    for weights in weights.state_weights_mut().unwrap() {
        weights.iter_mut().for_each(|val| {*val = weight;});
        weight += 0.1;
    }

    const VALUE_I: f32 = 5.0;
    const VALUE_S: f32 = 7.0;
    let mut input = MathVec::<IN>::new();
    input.iter_mut().for_each(|val| {*val = VALUE_I;});

    let mut state = MathVec::<OUT>::new();
    state.iter_mut().for_each(|val| {*val = VALUE_S;});

    let mut feedback = MathVec::<OUT>::new();
    let mut v: f32 = 1.0;
    for feedback_val in feedback.as_mut_array() {
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
                check += feedback[j] * weights.state_weights().unwrap()[j][i];
            }

            assert_eq!(
                (grad[i] * 10.0).round(),
                (check * 10.0).round()
            );
        }
    }
    {
        let mut grad = Weights::<OUT, IN>::new(true);

        Weights::grad_with_weights(&feedback, &input, Some(&state), &mut grad);

        assert_eq!(grad.bias(), feedback.as_array());

        for i in 0..OUT {
            for j in 0..IN {
                let check = feedback[i] * input[j];
                assert_eq!(
                    (grad.input_weights()[i][j] * 10.0).round(),
                    (check * 10.0).round()
                );
            }
        }

        for i in 0..OUT {
            for j in 0..OUT {
                let check = feedback[i] * state[j];
                assert_eq!(
                    (grad.state_weights().unwrap()[i][j] * 10.0).round(),
                    (check * 10.0).round()
                );
            }
        }
    }
}

#[test]
fn weights_test_6() {
    const OUT: usize = 7;
    const IN: usize = 13;

    const WEIGHT_B: f32 = 1.0;
    const WEIGHT_I: f32 = 2.0;

    let mut weights = Weights::<OUT, IN>::new(false);

    weights.bias_mut().iter_mut().for_each(|val| {*val = WEIGHT_B;});

    let mut weight = WEIGHT_I;
    for weights in weights.input_weights_mut() {
        weights.iter_mut().for_each(|val| {*val = weight;});
        weight += 0.1;
    }

    const VALUE_I: f32 = 5.0;
    const VALUE_S: f32 = 7.0;
    let mut input = MathVec::<IN>::new();
    input.iter_mut().for_each(|val| {*val = VALUE_I;});

    let mut state = MathVec::<OUT>::new();
    state.iter_mut().for_each(|val| {*val = VALUE_S;});

    let mut feedback = MathVec::<OUT>::new();
    let mut v: f32 = 1.0;
    for feedback_val in feedback.as_mut_array() {
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

        grad.iter().for_each(|val| {assert_eq!(*val, 0.0)});
    }
    {
        let mut grad = Weights::<OUT, IN>::new(false);

        Weights::grad_with_weights(&feedback, &input, Some(&state), &mut grad);

        assert_eq!(grad.bias(), feedback.as_array());

        for i in 0..OUT {
            for j in 0..IN {
                let check = feedback[i] * input[j];
                assert_eq!(
                    (grad.input_weights()[i][j] * 10.0).round(),
                    (check * 10.0).round()
                );
            }
        }
    }
}

#[test]
fn weights_test_9() {
    const OUT: usize = 7;
    const IN: usize = 13;

    let mut rng = ChobitRand::new("weights_test_9".as_bytes());

    {
        let mut weights_1 = Weights::<OUT, IN>::new(true);
        rand_weights(&mut rng, &mut weights_1);

        let mut weights_2 = Weights::<OUT, IN>::new(true);
        rand_weights(&mut rng, &mut weights_2);

        assert_ne!(weights_1, weights_2);

        weights_1.copy_from(&weights_2);

        assert_eq!(weights_1, weights_2);
    }
    {
        let mut weights_1 = Weights::<OUT, IN>::new(false);
        rand_weights(&mut rng, &mut weights_1);

        let mut weights_2 = Weights::<OUT, IN>::new(false);
        rand_weights(&mut rng, &mut weights_2);

        assert_ne!(weights_1, weights_2);

        weights_1.copy_from(&weights_2);

        assert_eq!(weights_1, weights_2);
    }
    {
        let mut weights_1 = Weights::<OUT, IN>::new(true);
        rand_weights(&mut rng, &mut weights_1);

        let mut weights_2 = Weights::<OUT, IN>::new(false);
        rand_weights(&mut rng, &mut weights_2);

        assert_ne!(weights_1, weights_2);

        weights_1.copy_from(&weights_2);

        assert_ne!(weights_1, weights_2);

        assert_eq!(&weights_1[..weights_2.len()], weights_2.as_slice());
    }
    {
        let mut weights_1 = Weights::<OUT, IN>::new(false);
        rand_weights(&mut rng, &mut weights_1);

        let mut weights_2 = Weights::<OUT, IN>::new(true);
        rand_weights(&mut rng, &mut weights_2);

        assert_ne!(weights_1, weights_2);

        weights_1.copy_from(&weights_2);

        assert_ne!(weights_1, weights_2);

        assert_eq!(weights_1.as_slice(), &weights_2[..weights_1.len()]);
    }
}

#[test]
fn weights_test_10() {
    const OUT: usize = 7;
    const IN: usize = 13;

    let mut rng = ChobitRand::new("weights_test_8".as_bytes());

    {
        let mut weights_1 = Weights::<OUT, IN>::new(true);
        rand_weights(&mut rng, &mut weights_1);

        let mut weights_2 = Weights::<OUT, IN>::new(true);
        rand_weights(&mut rng, &mut weights_2);

        assert_ne!(weights_1, weights_2);

        weights_1.copy_to(&mut weights_2);

        assert_eq!(weights_1, weights_2);
    }
    {
        let mut weights_1 = Weights::<OUT, IN>::new(false);
        rand_weights(&mut rng, &mut weights_1);

        let mut weights_2 = Weights::<OUT, IN>::new(false);
        rand_weights(&mut rng, &mut weights_2);

        assert_ne!(weights_1, weights_2);

        weights_1.copy_to(&mut weights_2);

        assert_eq!(weights_1, weights_2);
    }
    {
        let mut weights_1 = Weights::<OUT, IN>::new(true);
        rand_weights(&mut rng, &mut weights_1);

        let mut weights_2 = Weights::<OUT, IN>::new(false);
        rand_weights(&mut rng, &mut weights_2);

        assert_ne!(weights_1, weights_2);

        weights_1.copy_to(&mut weights_2);

        assert_ne!(weights_1, weights_2);

        assert_eq!(&weights_1[..weights_2.len()], weights_2.as_slice());
    }
    {
        let mut weights_1 = Weights::<OUT, IN>::new(false);
        rand_weights(&mut rng, &mut weights_1);

        let mut weights_2 = Weights::<OUT, IN>::new(true);
        rand_weights(&mut rng, &mut weights_2);

        assert_ne!(weights_1, weights_2);

        weights_1.copy_to(&mut weights_2);

        assert_ne!(weights_1, weights_2);

        assert_eq!(weights_1.as_slice(), &weights_2[..weights_1.len()]);
    }
}

#[test]
fn layer_test_1() {
    const OUT: usize = 7;
    const IN: usize = 13;

    // ready
    let mut rng = ChobitRand::new("layer_test_1".as_bytes());

    let mut layer_1 = Layer::<OUT, IN>::new(Activation::SoftSign, true);

    rand_weights(&mut rng, layer_1.mut_weights());

    let mut layer_2 = Layer::<OUT, IN>::new(Activation::SoftSign, true);

    rand_weights(&mut rng, layer_2.mut_weights());

    assert_ne!(layer_1, layer_2);

    let mut input = MathVec::<IN>::new();
    rand_math_vec(&mut rng, &mut input);

    let mut state = MathVec::<OUT>::new();
    rand_math_vec(&mut rng, &mut state);

    let mut output_1 = MathVec::<OUT>::new();
    let mut output_2 = MathVec::<OUT>::new();

    layer_1.calc(&input, Some(&state), &mut output_1);
    layer_2.calc(&input, Some(&state), &mut output_2);

    // checks before machine learning.
    const EPSILON_1: f32 = 0.01;
    for i in 0..OUT {
        let diff = (output_1[i] - output_2[i]).abs();
        assert!(diff > EPSILON_1);
    }

    // machine learning.
    const EPOCH: usize = 1000;
    const BATCH_SIZE: usize = 10;
    const RATE: f32 = 0.1;

    let mut ml_layer = MLLayer::new(layer_2);
    let mut cache = MLCache::new();
    let mut output_error = MathVec::<OUT>::new();
    let mut prev_state_error = MathVec::<OUT>::new();
    let mut input_error = MathVec::<IN>::new();
    let mut state_error = MathVec::<OUT>::new();

    for _ in 0..EPOCH {
        for _ in 0..BATCH_SIZE {
            ml_layer.ready(&input, Some(&state), &mut cache);

            cache.calc_output_error(&output_1, &mut output_error);

            prev_state_error.clear();

            ml_layer.study(
                &output_error,
                Some(&prev_state_error),
                &cache,
                &mut input_error,
                Some(&mut state_error)
            );
        }

        ml_layer.update(RATE);
    }

    let layer_2 = ml_layer.drop();

    layer_1.calc(&input, Some(&state), &mut output_1);
    layer_2.calc(&input, Some(&state), &mut output_2);

    // checks after machine learning.
    const EPSILON_2: f32 = 0.000001;
    for i in 0..OUT {
        let diff = (output_1[i] - output_2[i]).abs();
        assert!(diff < EPSILON_2);
    }
}

#[test]
fn layer_test_2() {
    const OUT: usize = 7;
    const IN: usize = 13;

    // ready
    let mut rng = ChobitRand::new("layer_test_2".as_bytes());

    let mut layer_1 = Layer::<OUT, IN>::new(Activation::SoftSign, false);
    rand_weights(&mut rng, layer_1.mut_weights());

    let mut layer_2 = Layer::<OUT, IN>::new(Activation::SoftSign, false);
    rand_weights(&mut rng, layer_2.mut_weights());

    assert_ne!(layer_1, layer_2);

    let mut input = MathVec::<IN>::new();
    rand_math_vec(&mut rng, &mut input);

    let mut output_1 = MathVec::<OUT>::new();
    let mut output_2 = MathVec::<OUT>::new();

    layer_1.calc(&input, None, &mut output_1);
    layer_2.calc(&input, None, &mut output_2);

    // checks before machine learning.
    const EPSILON_1: f32 = 0.05;
    for i in 0..OUT {
        let diff = (output_1[i] - output_2[i]).abs();
        assert!(diff > EPSILON_1);
    }

    // machine learning.
    const EPOCH: usize = 1000;
    const BATCH_SIZE: usize = 10;
    const RATE: f32 = 0.1;

    let mut ml_layer = MLLayer::new(layer_2);
    let mut cache = MLCache::new();
    let mut output_error = MathVec::<OUT>::new();
    let mut input_error = MathVec::<IN>::new();

    for _ in 0..EPOCH {
        for _ in 0..BATCH_SIZE {
            ml_layer.ready(&input, None, &mut cache);
            cache.calc_output_error(&output_1, &mut output_error);

            ml_layer.study(
                &output_error,
                None,
                &cache,
                &mut input_error,
                None
            );
        }

        ml_layer.update(RATE);
    }

    let layer_2 = ml_layer.drop();

    layer_1.calc(&input, None, &mut output_1);
    layer_2.calc(&input, None, &mut output_2);

    // checks after machine learning.
    const EPSILON_2: f32 = 0.000001;
    for i in 0..OUT {
        let diff = (output_1[i] - output_2[i]).abs();
        assert!(diff < EPSILON_2);
    }
}

//#[test]
//fn chobit_ai_test_1() {
//    const OUT: usize = 8;
//    const MIDDLE: usize = 32;
//    const IN: usize = 16;
//
//    const COUNT: usize = 100;
//
//    let mut rng = ChobitRand::new("chobit_ai_test_1".as_bytes());
//
//    let mut ai = ChobitAI::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//    let mut input = MathVec::<IN>::new();
//    let mut output = MathVec::<OUT>::new();
//    let mut tmpbuf = MathVec::<MIDDLE>::new();
//
//    rand_weights(&mut rng, ai.middle_layer_mut().mut_weights());
//    rand_weights(&mut rng, ai.output_layer_mut().mut_weights());
//
//    for _ in 0..COUNT {
//        let label_in = rng.next_u64() as u16;
//
//        input.load_u16_label(label_in);
//        ai.calc(&input, &mut output, &mut tmpbuf);
//    }
//
//    const EPOCH: usize = 10;
//    const BATCH_SIZE: usize = 10;
//    const RATE: f32 = 0.01;
//
//    let mut ai = ChobitMLAI::<OUT, MIDDLE, IN>::new(ai);
//
//    for _ in 0..EPOCH {
//        for _ in 0..BATCH_SIZE {
//            let label = rng.next_u64() as u32;
//            input.load_u16_label(label as u16);
//            output.load_u8_label(label as u8);
//
//            ai.study(&input, &output);
//        }
//
//        ai.update(RATE);
//    }
//
//    let ai = ai.drop();
//
//    for _ in 0..COUNT {
//        let label_in = rng.next_u64() as u16;
//
//        input.load_u16_label(label_in);
//        ai.calc(&input, &mut output, &mut tmpbuf);
//    }
//}
//
//#[cfg(not(debug_assertions))]
//#[test]
//fn chobit_ai_test_2() {
//    const OUT: usize = 32;
//    const MIDDLE: usize = 128;
//    const IN: usize = 32;
//
//    const COUNT: usize = 100;
//
//    let mut rng = ChobitRand::new("chobit_ai_test_2".as_bytes());
//
//    let mut ai = ChobitAI::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//    let mut input = MathVec::<IN>::new();
//    let mut output = MathVec::<OUT>::new();
//    let mut tmpbuf = MathVec::<MIDDLE>::new();
//
//    rand_weights(&mut rng, ai.middle_layer_mut().mut_weights());
//    rand_weights(&mut rng, ai.output_layer_mut().mut_weights());
//
//    // before machine learning.
//    for _ in 0..COUNT {
//        let label_in = rng.next_u64() as u32;
//
//        input.load_u32_label(label_in);
//        ai.calc(&input, &mut output, &mut tmpbuf);
//
//        let label_out = output.to_u32_label();
//
//        assert_ne!(label_out, label_in);
//    }
//
//    // machine learning.
//    const EPOCH: usize = 3000;
//    const BATCH_SIZE: usize = 100;
//    const RATE: f32 = 0.01;
//
//    let mut ai = ChobitMLAI::<OUT, MIDDLE, IN>::new(ai);
//
//    for _ in 0..EPOCH {
//        for _ in 0..BATCH_SIZE {
//            let label = rng.next_u64() as u32;
//            input.load_u32_label(label);
//            output.load_u32_label(label);
//
//            ai.study(&input, &output);
//        }
//
//        ai.update(RATE);
//    }
//
//    let ai = ai.drop();
//
//    // after machine learning.
//    for _ in 0..COUNT {
//        let label_in = rng.next_u64() as u32;
//
//        input.load_u32_label(label_in);
//        ai.calc(&input, &mut output, &mut tmpbuf);
//
//        let label_out = output.to_u32_label();
//
//        assert_eq!(label_out, label_in);
//    }
//}
//
//#[test]
//fn chobit_ai_test_3() {
//    const OUT: usize = 11;
//    const MIDDLE: usize = 7;
//    const IN: usize = 5;
//
//    let mut rng = ChobitRand::new("chobit_ai_test_3".as_bytes());
//
//    let mut ai_1 = ChobitAI::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//    let mut ai_2 = ChobitAI::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//
//    ai_1.for_each_weight_mut(|val| {
//        *val = rand_num(&mut rng);
//    });
//
//    ai_2.for_each_weight_mut(|val| {
//        *val = rand_num(&mut rng);
//    });
//
//    assert_ne!(ai_1, ai_2);
//
//    let mut vec = Vec::<f32>::new();
//
//    ai_1.for_each_weight(|val| {
//        vec.push(*val);
//    });
//
//    let mut vec_iter = vec.iter();
//
//    ai_2.for_each_weight_mut(|val| {
//        *val = *vec_iter.next().unwrap();
//    });
//
//    assert_eq!(ai_1, ai_2);
//}
//
//fn letter_data(
//    rng: &mut ChobitRand,
//    letters: &[char],
//    data: &mut Vec<MathVec<32>>
//) {
//    static DUMMY: [char; 10] = [
//        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
//    ];
//
//    let half_size: usize = ((rng.next_u64() % 5) + 1) as usize;
//
//    data.resize(half_size * 2, MathVec::<32>::new());
//
//    data[..half_size].iter_mut().for_each(|vec| {
//        let letter = letters[(rng.next_u64() as usize) % letters.len()];
//        let label = letter as u32;
//        vec.load_u32_label(label);
//    });
//
//    data[half_size..].iter_mut().for_each(|vec| {
//        let letter = DUMMY[(rng.next_u64() as usize) % DUMMY.len()];
//        let label = letter as u32;
//        vec.load_u32_label(label);
//    });
//
//    rng.shuffle(data);
//}
//
//const JAPANESE: char = '日';
//fn japanese_data(rng: &mut ChobitRand, data: &mut Vec<MathVec<32>>) {
//    let letters: [char; 10] = [
//        'あ', 'い', 'う', 'え', 'お', 'か', 'き', 'く', 'け', 'こ'
//    ];
//
//    letter_data(rng, &letters, data);
//}
//
//#[cfg(not(debug_assertions))]
//const ENGLISH: char = 'E';
//
//#[cfg(not(debug_assertions))]
//fn english_data(rng: &mut ChobitRand, data: &mut Vec<MathVec<32>>) {
//    let letters: [char; 10] = [
//        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'
//    ];
//
//    letter_data(rng, &letters, data);
//}
//
//#[cfg(not(debug_assertions))]
//fn data_to_string(data: &[MathVec<32>]) -> String {
//    let mut ret = String::new();
//
//    data.iter().for_each(|vec| {
//        let label = vec.to_u32_label();
//        ret.push(char::from_u32(label).unwrap());
//    });
//
//    ret
//}
//
//fn gen_lstm<const OUT: usize, const IN: usize>(
//    rng: &mut ChobitRand
//) -> LSTM<OUT, IN> {
//    let mut ret = LSTM::<OUT, IN>::new();
//
//    rand_weights(rng, ret.main_layer_mut().mut_weights());
//    rand_weights(rng, ret.f_gate_mut().mut_weights());
//    rand_weights(rng, ret.i_gate_mut().mut_weights());
//    rand_weights(rng, ret.o_gate_mut().mut_weights());
//
//    ret
//}
//
//fn gen_layer<const OUT: usize, const IN: usize>(
//    rng: &mut ChobitRand
//) -> Layer<OUT, IN> {
//    let mut ret = Layer::<OUT, IN>::new(Activation::SoftSign, false);
//
//    rand_weights(rng, ret.mut_weights());
//
//    ret
//}
//
//#[test]
//fn lstm_test_1() {
//    const OUT: usize = 32;
//    const IN: usize = 32;
//
//    let mut rng = ChobitRand::new("lstm_test_1".as_bytes());
//
//    let mut data = Vec::<MathVec<32>>::new();
//    let mut japanese = MathVec::<OUT>::new();
//    japanese.load_u32_label(JAPANESE as u32);
//
//    let mut prev_state = MathVec::<OUT>::new();
//    let mut output = MathVec::<OUT>::new();
//    let mut state = MathVec::<OUT>::new();
//    let mut tmpbuf = MathVec::<OUT>::new();
//
//    const COUNT: usize = 10;
//
//    let lstm = gen_lstm::<OUT, IN>(&mut rng);
//    let output_layer = gen_layer::<OUT, OUT>(&mut rng);
//
//    for _ in 0..COUNT {
//        japanese_data(&mut rng, &mut data);
//        prev_state.clear();
//
//        for i in 0..(data.len() - 1) {
//            lstm.calc_state(
//                &data[i],
//                &prev_state,
//                &mut state,
//                &mut tmpbuf,
//            );
//
//            prev_state.copy_from(&state);
//        }
//
//        prev_state.copy_from(&state);
//
//        lstm.calc(
//            data.last().unwrap(),
//            &prev_state,
//            &mut output,
//            &mut state,
//            &mut tmpbuf
//        );
//
//        tmpbuf.copy_from(&output);
//        output_layer.calc(&tmpbuf, None, &mut output);
//    }
//
//    const EPOCH: usize = 10;
//    const BATCH_SIZE: usize = 10;
//    const RATE: f32 = 0.01;
//
//    let mut lstm = MLLSTM::<OUT, IN>::new(lstm);
//    let mut output_layer = MLLayer::<OUT, OUT>::new(output_layer);
//
//    let mut lstm_state_caches = vec![MLLSTMStateCache::<OUT, IN>::new(); 30];
//    let mut lstm_output_cache = MLLSTMOutputCache::<OUT, IN>::new();
//    let mut output_layer_cache = MLCache::<OUT, OUT>::new();
//    let mut output_error = MathVec::<OUT>::new();
//    let mut state_error = MathVec::<OUT>::new();
//    let mut input_error = MathVec::<IN>::new();
//    let mut tmp_input_error = MathVec::<IN>::new();
//    let mut prev_state_error = MathVec::<OUT>::new();
//    for _ in 0..EPOCH {
//        for _ in 0..BATCH_SIZE {
//            japanese_data(&mut rng, &mut data);
//            prev_state.clear();
//
//            for i in 0..data.len() {
//                lstm.ready_state_cache(
//                    &data[i],
//                    &prev_state,
//                    &mut lstm_state_caches[i]
//                );
//                prev_state.copy_from(&lstm_state_caches[i].state());
//            }
//
//            let tail: usize = data.len() - 1;
//
//            lstm.ready_output_cache(
//                &lstm_state_caches[tail],
//                &mut lstm_output_cache
//            );
//
//            output_layer.ready(
//                lstm_output_cache.output(),
//                None,
//                &mut output_layer_cache
//            );
//
//            output_layer_cache.calc_output_error(&japanese, &mut output_error);
//
//            tmpbuf.copy_from(&output_error);
//
//            output_layer.study(
//                &tmpbuf,
//                None,
//                &output_layer_cache,
//                &mut output_error,
//                None
//            );
//
//            state_error.clear();
//            lstm.study(
//                &output_error,
//                &state_error,
//                &lstm_state_caches[tail],
//                &lstm_output_cache,
//                &mut input_error,
//                &mut prev_state_error
//            );
//            state_error.copy_from(&prev_state_error);
//
//            for i in 1..data.len() {
//                let i = tail - i;
//
//                lstm.study_state(
//                    &state_error,
//                    &lstm_state_caches[i],
//                    &mut tmp_input_error,
//                    &mut prev_state_error
//                );
//
//                input_error += &tmp_input_error;
//                state_error.copy_from(&prev_state_error);
//            }
//        }
//
//        lstm.update(RATE);
//        output_layer.update(RATE);
//    }
//}
//
//#[cfg(not(debug_assertions))]
//#[test]
//fn lstm_test_2() {
//    const OUT: usize = 32;
//    const IN: usize = 32;
//
//    let mut rng = ChobitRand::new("lstm_test_2".as_bytes());
//
//    let mut data = Vec::<MathVec<32>>::new();
//    let mut japanese = MathVec::<OUT>::new();
//    japanese.load_u32_label(JAPANESE as u32);
//    let mut english = MathVec::<OUT>::new();
//    english.load_u32_label(ENGLISH as u32);
//
//    let mut prev_state = MathVec::<OUT>::new();
//    let mut output = MathVec::<OUT>::new();
//    let mut state = MathVec::<OUT>::new();
//    let mut tmpbuf = MathVec::<OUT>::new();
//
//    const COUNT: usize = 10;
//
//    let lstm = gen_lstm::<OUT, IN>(&mut rng);
//    let output_layer = gen_layer::<OUT, OUT>(&mut rng);
//
//    for _ in 0..COUNT {
//        japanese_data(&mut rng, &mut data);
//        prev_state.clear();
//
//        for i in 0..(data.len() - 1) {
//            lstm.calc_state(
//                &data[i],
//                &prev_state,
//                &mut state,
//                &mut tmpbuf,
//            );
//
//            prev_state.copy_from(&state);
//        }
//
//        prev_state.copy_from(&state);
//        lstm.calc(
//            data.last().unwrap(),
//            &prev_state,
//            &mut output,
//            &mut state,
//            &mut tmpbuf
//        );
//
//        tmpbuf.copy_from(&output);
//        output_layer.calc(&tmpbuf, None, &mut output);
//
//        assert_ne!(
//            output.to_u32_label(),
//            japanese.to_u32_label(),
//        );
//
//        english_data(&mut rng, &mut data);
//        prev_state.clear();
//
//        for i in 0..data.len() {
//            lstm.calc_state(
//                &data[i],
//                &prev_state,
//                &mut state,
//                &mut tmpbuf,
//            );
//
//            prev_state.copy_from(&state);
//        }
//
//        prev_state.copy_from(&state);
//        lstm.calc(
//            data.last().unwrap(),
//            &prev_state,
//            &mut output,
//            &mut state,
//            &mut tmpbuf
//        );
//
//        tmpbuf.copy_from(&output);
//        output_layer.calc(&tmpbuf, None, &mut output);
//
//        assert_ne!(
//            output.to_u32_label(),
//            english.to_u32_label(),
//        );
//    }
//
//    const EPOCH: usize = 1000;
//    const BATCH_SIZE: usize = 100;
//    const RATE: f32 = 0.01;
//
//    let mut lstm = MLLSTM::<OUT, IN>::new(lstm);
//    let mut output_layer = MLLayer::<OUT, OUT>::new(output_layer);
//
//    let mut lstm_state_caches = vec![MLLSTMStateCache::<OUT, IN>::new(); 30];
//    let mut lstm_output_cache = MLLSTMOutputCache::<OUT, IN>::new();
//    let mut output_layer_cache = MLCache::<OUT, OUT>::new();
//    let mut output_error = MathVec::<OUT>::new();
//    let mut state_error = MathVec::<OUT>::new();
//    let mut input_error = MathVec::<IN>::new();
//    let mut tmp_input_error = MathVec::<IN>::new();
//    let mut prev_state_error = MathVec::<OUT>::new();
//    for _ in 0..EPOCH {
//        for _ in 0..BATCH_SIZE {
//            japanese_data(&mut rng, &mut data);
//            prev_state.clear();
//
//            for i in 0..data.len() {
//                lstm.ready_state_cache(
//                    &data[i],
//                    &prev_state,
//                    &mut lstm_state_caches[i]
//                );
//                prev_state.copy_from(&lstm_state_caches[i].state());
//            }
//
//            let tail: usize = data.len() - 1;
//
//            lstm.ready_output_cache(
//                &lstm_state_caches[tail],
//                &mut lstm_output_cache
//            );
//
//            output_layer.ready(
//                lstm_output_cache.output(),
//                None,
//                &mut output_layer_cache
//            );
//
//            output_layer_cache.calc_output_error(&japanese, &mut output_error);
//
//            tmpbuf.copy_from(&output_error);
//
//            output_layer.study(
//                &tmpbuf,
//                None,
//                &output_layer_cache,
//                &mut output_error,
//                None
//            );
//
//            state_error.clear();
//            lstm.study(
//                &output_error,
//                &state_error,
//                &lstm_state_caches[tail],
//                &lstm_output_cache,
//                &mut input_error,
//                &mut prev_state_error
//            );
//            state_error.copy_from(&prev_state_error);
//
//            for i in 1..data.len() {
//                let i = tail - i;
//
//                lstm.study_state(
//                    &state_error,
//                    &lstm_state_caches[i],
//                    &mut tmp_input_error,
//                    &mut prev_state_error
//                );
//
//                input_error += &tmp_input_error;
//                state_error.copy_from(&prev_state_error);
//            }
//
//            english_data(&mut rng, &mut data);
//            prev_state.clear();
//
//            for i in 0..data.len() {
//                lstm.ready_state_cache(
//                    &data[i],
//                    &prev_state,
//                    &mut lstm_state_caches[i]
//                );
//                prev_state.copy_from(&lstm_state_caches[i].state());
//            }
//
//            let tail: usize = data.len() - 1;
//
//            lstm.ready_output_cache(
//                &lstm_state_caches[tail],
//                &mut lstm_output_cache
//            );
//
//            output_layer.ready(
//                lstm_output_cache.output(),
//                None,
//                &mut output_layer_cache
//            );
//
//            output_layer_cache.calc_output_error(&english, &mut output_error);
//
//            tmpbuf.copy_from(&output_error);
//
//            output_layer.study(
//                &tmpbuf,
//                None,
//                &output_layer_cache,
//                &mut output_error,
//                None
//            );
//
//            state_error.clear();
//            lstm.study(
//                &output_error,
//                &state_error,
//                &lstm_state_caches[tail],
//                &lstm_output_cache,
//                &mut input_error,
//                &mut prev_state_error
//            );
//            state_error.copy_from(&prev_state_error);
//
//            for i in 1..data.len() {
//                let i = tail - i;
//
//                lstm.study_state(
//                    &state_error,
//                    &lstm_state_caches[i],
//                    &mut tmp_input_error,
//                    &mut prev_state_error
//                );
//
//                input_error += &tmp_input_error;
//                state_error.copy_from(&prev_state_error);
//            }
//        }
//
//        lstm.update(RATE);
//        output_layer.update(RATE);
//    }
//
//    let lstm = lstm.drop();
//    let output_layer = output_layer.drop();
//
//    for _ in 0..COUNT {
//        japanese_data(&mut rng, &mut data);
//        prev_state.clear();
//
//        for i in 0..(data.len() - 1) {
//            lstm.calc_state(
//                &data[i],
//                &prev_state,
//                &mut state,
//                &mut tmpbuf,
//            );
//
//            prev_state.copy_from(&state);
//        }
//
//        prev_state.copy_from(&state);
//        lstm.calc(
//            data.last().unwrap(),
//            &prev_state,
//            &mut output,
//            &mut state,
//            &mut tmpbuf
//        );
//
//        tmpbuf.copy_from(&output);
//        output_layer.calc(&tmpbuf, None, &mut output);
//
//        assert_eq!(
//            output.to_u32_label(),
//            japanese.to_u32_label(),
//        );
//
//        english_data(&mut rng, &mut data);
//        prev_state.clear();
//
//        for i in 0..(data.len() - 1) {
//            lstm.calc_state(
//                &data[i],
//                &prev_state,
//                &mut state,
//                &mut tmpbuf,
//            );
//
//            prev_state.copy_from(&state);
//        }
//
//        prev_state.copy_from(&state);
//        lstm.calc(
//            data.last().unwrap(),
//            &prev_state,
//            &mut output,
//            &mut state,
//            &mut tmpbuf
//        );
//
//        tmpbuf.copy_from(&output);
//        output_layer.calc(&tmpbuf, None, &mut output);
//
//        assert_eq!(
//            output.to_u32_label(),
//            english.to_u32_label(),
//        );
//    }
//}
//
//#[test]
//fn lstm_test_3() {
//    const OUT: usize = 11;
//    const IN: usize = 5;
//
//    let mut rng = ChobitRand::new("lstm_test_3".as_bytes());
//
//    let mut lstm_1 = LSTM::<OUT, IN>::new();
//    let mut lstm_2 = LSTM::<OUT, IN>::new();
//
//    lstm_1.for_each_weight_mut(|val| {
//        *val = rand_num(&mut rng);
//    });
//
//    lstm_2.for_each_weight_mut(|val| {
//        *val = rand_num(&mut rng);
//    });
//
//    assert_ne!(lstm_1, lstm_2);
//
//    let mut vec = Vec::<f32>::new();
//
//    lstm_1.for_each_weight(|val| {
//        vec.push(*val);
//    });
//
//    let mut vec_iter = vec.iter();
//
//    lstm_2.for_each_weight_mut(|val| {
//        *val = *vec_iter.next().unwrap();
//    });
//
//    assert_eq!(lstm_1, lstm_2);
//}
//
//fn gen_encoder<
//    const OUT: usize,
//    const MIDDLE: usize,
//    const IN: usize
//>(rng: &mut ChobitRand) -> ChobitEncoder<OUT, MIDDLE, IN> {
//    let mut ret = ChobitEncoder::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//
//    rand_weights(rng, ret.lstm_mut().main_layer_mut().mut_weights());
//    rand_weights(rng, ret.lstm_mut().f_gate_mut().mut_weights());
//    rand_weights(rng, ret.lstm_mut().i_gate_mut().mut_weights());
//    rand_weights(rng, ret.lstm_mut().o_gate_mut().mut_weights());
//
//    rand_weights(rng, ret.output_layer_mut().mut_weights());
//
//    ret
//}
//
//#[test]
//fn chobit_encoder_test_1() {
//    const OUT: usize = 32;
//    const MIDDLE: usize = 64;
//    const IN: usize = 32;
//
//    let mut rng = ChobitRand::new("chobit_encoder_test_1".as_bytes());
//
//    let mut data = Vec::<MathVec<32>>::new();
//    let mut japanese = MathVec::<OUT>::new();
//    japanese.load_u32_label(JAPANESE as u32);
//
//    const COUNT: usize = 10;
//
//    let mut encoder = gen_encoder::<OUT, MIDDLE, IN>(&mut rng);
//    let mut output = MathVec::<OUT>::new();
//
//    for _ in 0..COUNT {
//        japanese_data(&mut rng, &mut data);
//        encoder.state_mut().clear();
//
//        data.iter().for_each(|data_one| {
//            encoder.input_next(data_one);
//        });
//
//        encoder.output(&mut output);
//    }
//
//    const EPOCH: usize = 10;
//    const BATCH_SIZE: usize = 10;
//    const RATE: f32 = 0.01;
//
//    let mut encoder = ChobitMLEncoder::<OUT, MIDDLE, IN>::new(encoder);
//    let prev_state = MathVec::<MIDDLE>::new();
//
//    for _ in 0..EPOCH {
//        for _ in 0..BATCH_SIZE {
//            japanese_data(&mut rng, &mut data);
//
//            encoder.study(
//                &data,
//                &prev_state,
//                &japanese,
//            );
//        }
//
//        encoder.update(RATE);
//    }
//}
//
//#[cfg(not(debug_assertions))]
//#[test]
//fn chobit_encoder_test_2() {
//    const OUT: usize = 32;
//    const MIDDLE: usize = 64;
//    const IN: usize = 32;
//
//    let mut rng = ChobitRand::new("chobit_encoder_test_2".as_bytes());
//
//    let mut data = Vec::<MathVec<32>>::new();
//    let mut japanese = MathVec::<OUT>::new();
//    japanese.load_u32_label(JAPANESE as u32);
//    let mut english = MathVec::<OUT>::new();
//    english.load_u32_label(ENGLISH as u32);
//
//    const COUNT: usize = 10;
//
//    let mut encoder = gen_encoder::<OUT, MIDDLE, IN>(&mut rng);
//    let mut output = MathVec::<OUT>::new();
//
//    for _ in 0..COUNT {
//        japanese_data(&mut rng, &mut data);
//        encoder.state_mut().clear();
//
//        data.iter().for_each(|data_one| {
//            encoder.input_next(data_one);
//        });
//
//        encoder.output(&mut output);
//
//        assert_ne!(
//            output.to_u32_label(),
//            japanese.to_u32_label(),
//        );
//        println!(
//            "{}, {:?}",
//            data_to_string(&data),
//            char::from_u32(output.to_u32_label())
//        )
//    }
//
//    for _ in 0..COUNT {
//        english_data(&mut rng, &mut data);
//        encoder.state_mut().clear();
//
//        data.iter().for_each(|data_one| {
//            encoder.input_next(data_one);
//        });
//
//        encoder.output(&mut output);
//
//        assert_ne!(
//            output.to_u32_label(),
//            english.to_u32_label(),
//        );
//        println!(
//            "{}, {:?}",
//            data_to_string(&data),
//            char::from_u32(output.to_u32_label())
//        )
//    }
//
//    const EPOCH: usize = 1000;
//    const BATCH_SIZE: usize = 100;
//    const RATE: f32 = 0.01;
//
//    let mut encoder = ChobitMLEncoder::<OUT, MIDDLE, IN>::new(encoder);
//    let prev_state = MathVec::<MIDDLE>::new();
//
//    for _ in 0..EPOCH {
//        for _ in 0..BATCH_SIZE {
//            japanese_data(&mut rng, &mut data);
//
//            encoder.study(
//                &data,
//                &prev_state,
//                &japanese,
//            );
//
//            english_data(&mut rng, &mut data);
//
//            encoder.study(
//                &data,
//                &prev_state,
//                &english,
//            );
//        }
//
//        encoder.update(RATE);
//    }
//
//    let mut encoder = encoder.drop();
//
//    for _ in 0..COUNT {
//        japanese_data(&mut rng, &mut data);
//        encoder.state_mut().clear();
//
//        data.iter().for_each(|data_one| {
//            encoder.input_next(data_one);
//        });
//
//        encoder.output(&mut output);
//
//        assert_eq!(
//            output.to_u32_label(),
//            japanese.to_u32_label(),
//        );
//        println!(
//            "{}, {:?}",
//            data_to_string(&data),
//            char::from_u32(output.to_u32_label())
//        )
//    }
//
//    for _ in 0..COUNT {
//        english_data(&mut rng, &mut data);
//        encoder.state_mut().clear();
//
//        data.iter().for_each(|data_one| {
//            encoder.input_next(data_one);
//        });
//
//        encoder.output(&mut output);
//
//        assert_eq!(
//            output.to_u32_label(),
//            english.to_u32_label(),
//        );
//        println!(
//            "{}, {:?}",
//            data_to_string(&data),
//            char::from_u32(output.to_u32_label())
//        )
//    }
//}
//
//#[test]
//fn chobit_encoder_test_3() {
//    const OUT: usize = 11;
//    const MIDDLE: usize = 7;
//    const IN: usize = 5;
//
//    let mut rng = ChobitRand::new("chobit_encoder_test_3".as_bytes());
//
//    let mut encoder_1 =
//        ChobitEncoder::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//    encoder_1.for_each_weight_mut(|val| {
//        *val = rand_num(&mut rng);
//    });
//
//    let mut encoder_2 =
//        ChobitEncoder::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//    encoder_2.for_each_weight_mut(|val| {
//        *val = rand_num(&mut rng);
//    });
//
//    assert_ne!(encoder_1, encoder_2);
//
//    let mut vec = Vec::<f32>::new();
//    encoder_1.for_each_weight(|val| {
//        vec.push(*val);
//    });
//
//    let mut vec_iter = vec.iter();
//    encoder_2.for_each_weight_mut(|val| {
//        *val = *vec_iter.next().unwrap();
//    });
//
//    assert_eq!(encoder_1, encoder_2);
//}
//
//fn gen_decoder<
//    const OUT: usize,
//    const MIDDLE: usize,
//    const IN: usize
//>(rng: &mut ChobitRand) -> ChobitDecoder<OUT, MIDDLE, IN> {
//    let mut ret = ChobitDecoder::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//
//    rand_weights(rng, ret.lstm_mut().main_layer_mut().mut_weights());
//    rand_weights(rng, ret.lstm_mut().f_gate_mut().mut_weights());
//    rand_weights(rng, ret.lstm_mut().i_gate_mut().mut_weights());
//    rand_weights(rng, ret.lstm_mut().o_gate_mut().mut_weights());
//
//    rand_weights(rng, ret.output_layer_mut().mut_weights());
//
//    ret
//}
//
//#[test]
//fn chobit_decoder_test_1() {
//    const OUT: usize = 32;
//    const MIDDLE: usize = 64;
//    const IN: usize = 32;
//
//    let mut rng = ChobitRand::new("chobit_decoder_test_1".as_bytes());
//
//    let mut japanese_in = MathVec::<IN>::new();
//    japanese_in.load_u32_label(JAPANESE as u32);
//
//    let mut japanese_out = Vec::<MathVec<OUT>>::new();
//    "これは日本語です。\x00".chars().for_each(|c| {
//        let mut vec = MathVec::<OUT>::new();
//        vec.load_u32_label(c as u32);
//
//        japanese_out.push(vec);
//    });
//
//    let mut decoder = gen_decoder(&mut rng);
//
//    {
//        decoder.input_mut().copy_from(&japanese_in);
//        decoder.state_mut().clear();
//
//        let mut output = vec![MathVec::<OUT>::new(); japanese_out.len()];
//        output.iter_mut().for_each(|output_one| {
//            decoder.output_next(output_one);
//        });
//    }
//
//    const EPOCH: usize = 3;
//    const BATCH_SIZE: usize = 3;
//    const RATE: f32 = 0.01;
//
//    let mut decoder = ChobitMLDecoder::<OUT, MIDDLE, IN>::new(decoder);
//
//    let prev_state = MathVec::<MIDDLE>::new();
//
//    for _ in 0..EPOCH {
//        for _ in 0..BATCH_SIZE {
//            decoder.study(
//                &japanese_in,
//                &prev_state,
//                japanese_out.as_slice()
//            );
//        }
//
//        decoder.update(RATE);
//    }
//}
//
//#[cfg(not(debug_assertions))]
//#[test]
//fn chobit_decoder_test_2() {
//    const OUT: usize = 32;
//    const MIDDLE: usize = 64;
//    const IN: usize = 32;
//
//    let mut rng = ChobitRand::new("chobit_decoder_test_1".as_bytes());
//
//    let mut japanese_in = MathVec::<IN>::new();
//    japanese_in.load_u32_label(JAPANESE as u32);
//
//    let mut japanese_out = Vec::<MathVec<OUT>>::new();
//    "これは日本語です。\x00".chars().for_each(|c| {
//        let mut vec = MathVec::<OUT>::new();
//        vec.load_u32_label(c as u32);
//
//        japanese_out.push(vec);
//    });
//
//    let mut english_in = MathVec::<IN>::new();
//    english_in.load_u32_label(ENGLISH as u32);
//
//    let mut english_out = Vec::<MathVec<OUT>>::new();
//    "this is english.\x00".chars().for_each(|c| {
//        let mut vec = MathVec::<OUT>::new();
//        vec.load_u32_label(c as u32);
//
//        english_out.push(vec);
//    });
//
//    let mut decoder = gen_decoder(&mut rng);
//
//    {
//        decoder.input_mut().copy_from(&japanese_in);
//        decoder.state_mut().clear();
//
//        let mut output = vec![MathVec::<OUT>::new(); japanese_out.len()];
//        output.iter_mut().for_each(|output_one| {
//            decoder.output_next(output_one);
//        });
//
//        assert_ne!(output, japanese_out);
//    }
//
//    {
//        decoder.input_mut().copy_from(&english_in);
//        decoder.state_mut().clear();
//
//        let mut output = vec![MathVec::<OUT>::new(); english_out.len()];
//        output.iter_mut().for_each(|output_one| {
//            decoder.output_next(output_one);
//        });
//
//        assert_ne!(output, english_out);
//    }
//
//    const EPOCH: usize = 50000;
//    const BATCH_SIZE: usize = 1;
//    const RATE: f32 = 0.01;
//
//    let mut decoder = ChobitMLDecoder::<OUT, MIDDLE, IN>::new(decoder);
//
//    let prev_state_j = MathVec::<MIDDLE>::new();
//    let prev_state_e = MathVec::<MIDDLE>::new();
//
//    for _ in 0..EPOCH {
//        for _ in 0..BATCH_SIZE {
//            decoder.study(
//                &japanese_in,
//                &prev_state_j,
//                japanese_out.as_slice()
//            );
//
//            decoder.study(
//                &english_in,
//                &prev_state_e,
//                english_out.as_slice()
//            );
//        }
//
//        decoder.update(RATE);
//    }
//
//    let mut decoder = decoder.drop();
//
//    {
//        decoder.input_mut().copy_from(&japanese_in);
//        decoder.state_mut().clear();
//
//        let mut output = vec![MathVec::<OUT>::new(); japanese_out.len()];
//        output.iter_mut().for_each(|output_one| {
//            decoder.output_next(output_one);
//            println!("{:?}", char::from_u32(output_one.to_u32_label()));
//        });
//
//        assert_eq!(
//            data_to_string(output.as_slice()),
//            data_to_string(japanese_out.as_slice())
//        );
//    }
//
//    println!("-----");
//
//    {
//        decoder.input_mut().copy_from(&english_in);
//        decoder.state_mut().clear();
//
//
//        let mut output = vec![MathVec::<OUT>::new(); english_out.len()];
//        output.iter_mut().for_each(|output_one| {
//            decoder.output_next(output_one);
//            println!("{:?}", char::from_u32(output_one.to_u32_label()));
//        });
//
//        assert_eq!(
//            data_to_string(output.as_slice()),
//            data_to_string(english_out.as_slice())
//        );
//    }
//}
//
//#[test]
//fn chobit_decoder_test_3() {
//    const OUT: usize = 11;
//    const MIDDLE: usize = 7;
//    const IN: usize = 5;
//
//    let mut rng = ChobitRand::new("chobit_decoder_test_3".as_bytes());
//
//    let mut decoder_1 =
//        ChobitDecoder::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//    decoder_1.for_each_weight_mut(|val| {
//        *val = rand_num(&mut rng);
//    });
//
//    let mut decoder_2 =
//        ChobitDecoder::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//    decoder_2.for_each_weight_mut(|val| {
//        *val = rand_num(&mut rng);
//    });
//
//    assert_ne!(decoder_1, decoder_2);
//
//    let mut vec = Vec::<f32>::new();
//    decoder_1.for_each_weight(|val| {
//        vec.push(*val);
//    });
//
//    let mut vec_iter = vec.iter();
//    decoder_2.for_each_weight_mut(|val| {
//        *val = *vec_iter.next().unwrap();
//    });
//
//    assert_eq!(decoder_1, decoder_2);
//}
//
//fn gen_seq_ai<
//    const OUT: usize,
//    const MIDDLE: usize,
//    const IN: usize
//>(rng: &mut ChobitRand) -> ChobitSeqAI<OUT, MIDDLE, IN> {
//    let mut ret = ChobitSeqAI::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//
//    rand_weights(rng, ret.enc_layer_mut().main_layer_mut().mut_weights());
//    rand_weights(rng, ret.enc_layer_mut().f_gate_mut().mut_weights());
//    rand_weights(rng, ret.enc_layer_mut().i_gate_mut().mut_weights());
//    rand_weights(rng, ret.enc_layer_mut().o_gate_mut().mut_weights());
//
//    rand_weights(rng, ret.dec_layer_mut().main_layer_mut().mut_weights());
//    rand_weights(rng, ret.dec_layer_mut().f_gate_mut().mut_weights());
//    rand_weights(rng, ret.dec_layer_mut().i_gate_mut().mut_weights());
//    rand_weights(rng, ret.dec_layer_mut().o_gate_mut().mut_weights());
//
//    rand_weights(rng, ret.output_layer_mut().mut_weights());
//
//    ret
//}
//
//#[test]
//fn chobit_seq_ai_test_1() {
//    const OUT: usize = 32;
//    const MIDDLE: usize = 64;
//    const IN: usize = 32;
//
//    let mut rng = ChobitRand::new("chobit_seq_ai_test_1".as_bytes());
//
//    let mut data = Vec::<MathVec<32>>::new();
//
//    let mut japanese_out = Vec::<MathVec<OUT>>::new();
//    "これは日本語です。\x00".chars().for_each(|c| {
//        let mut vec = MathVec::<OUT>::new();
//        vec.load_u32_label(c as u32);
//
//        japanese_out.push(vec);
//    });
//
//    let mut ai = gen_seq_ai::<OUT, MIDDLE, IN>(&mut rng);
//    let mut output = vec![MathVec::<OUT>::new(); 30];
//
//    const COUNT: usize = 10;
//
//    for _ in 0..COUNT {
//        japanese_data(&mut rng, &mut data);
//        ai.state_mut().clear();
//
//        data.iter().for_each(|data_one| {
//            ai.input_next(data_one);
//        });
//
//        japanese_out.iter().zip(output.as_mut_slice()).for_each(
//            |(_, output_one)| {
//                ai.output_next(output_one);
//            }
//        );
//    }
//
//    const EPOCH: usize = 3;
//    const BATCH_SIZE: usize = 3;
//    const RATE: f32 = 0.01;
//
//    let mut ai = ChobitMLSeqAI::<OUT, MIDDLE, IN>::new(ai);
//
//    let prev_state = MathVec::<MIDDLE>::new();
//
//    for _ in 0..EPOCH {
//        for _ in 0..BATCH_SIZE {
//            japanese_data(&mut rng, &mut data);
//            ai.study(
//                data.as_slice(),
//                &prev_state,
//                &japanese_out
//            );
//        }
//        ai.update(RATE);
//    }
//}
//
//#[cfg(not(debug_assertions))]
//#[test]
//fn chobit_seq_ai_test_2() {
//    const OUT: usize = 32;
//    const MIDDLE: usize = 64;
//    const IN: usize = 32;
//
//    let mut rng = ChobitRand::new("chobit_seq_ai_test_2".as_bytes());
//
//    let mut data = Vec::<MathVec<32>>::new();
//
//    let mut japanese_out = Vec::<MathVec<OUT>>::new();
//    "これは日本語です。\x00".chars().for_each(|c| {
//        let mut vec = MathVec::<OUT>::new();
//        vec.load_u32_label(c as u32);
//
//        japanese_out.push(vec);
//    });
//
//    let mut english_out = Vec::<MathVec<OUT>>::new();
//    "this is english.\x00".chars().for_each(|c| {
//        let mut vec = MathVec::<OUT>::new();
//        vec.load_u32_label(c as u32);
//
//        english_out.push(vec);
//    });
//
//    let mut ai = gen_seq_ai::<OUT, MIDDLE, IN>(&mut rng);
//    let mut output = vec![MathVec::<OUT>::new(); 30];
//
//    const COUNT: usize = 10;
//
//    for _ in 0..COUNT {
//        japanese_data(&mut rng, &mut data);
//        ai.state_mut().clear();
//
//        data.iter().for_each(|data_one| {
//            ai.input_next(data_one);
//        });
//
//        japanese_out.iter().zip(output.as_mut_slice()).for_each(
//            |(_, output_one)| {
//                ai.output_next(output_one);
//            }
//        );
//
//        japanese_out.iter().zip(output.as_slice()).for_each(
//            |(train_out, output_one)| {
//                assert_ne!(
//                    char::from_u32(train_out.to_u32_label()),
//                    char::from_u32(output_one.to_u32_label()),
//                );
//            }
//        );
//    }
//
//    for _ in 0..COUNT {
//        english_data(&mut rng, &mut data);
//        ai.state_mut().clear();
//
//        data.iter().for_each(|data_one| {
//            ai.input_next(data_one);
//        });
//
//        english_out.iter().zip(output.as_mut_slice()).for_each(
//            |(_, output_one)| {
//                ai.output_next(output_one);
//            }
//        );
//
//        english_out.iter().zip(output.as_slice()).for_each(
//            |(train_out, output_one)| {
//                assert_ne!(
//                    char::from_u32(train_out.to_u32_label()),
//                    char::from_u32(output_one.to_u32_label()),
//                );
//            }
//        );
//    }
//
//    const EPOCH: usize = 3000;
//    const BATCH_SIZE: usize = 20;
//    const RATE: f32 = 0.01;
//
//    let mut ai = ChobitMLSeqAI::<OUT, MIDDLE, IN>::new(ai);
//
//    let prev_state = MathVec::<MIDDLE>::new();
//
//    for _ in 0..EPOCH {
//        for _ in 0..BATCH_SIZE {
//            japanese_data(&mut rng, &mut data);
//            ai.study(
//                data.as_slice(),
//                &prev_state,
//                &japanese_out
//            );
//
//            english_data(&mut rng, &mut data);
//            ai.study(
//                data.as_slice(),
//                &prev_state,
//                &english_out
//            );
//        }
//        ai.update(RATE);
//    }
//
//    let mut ai = ai.drop();
//
//    for _ in 0..COUNT {
//        japanese_data(&mut rng, &mut data);
//        ai.state_mut().clear();
//
//        data.iter().for_each(|data_one| {
//            ai.input_next(data_one);
//        });
//
//        japanese_out.iter().zip(output.as_mut_slice()).for_each(
//            |(_, output_one)| {
//                ai.output_next(output_one);
//            }
//        );
//
//        japanese_out.iter().zip(output.as_slice()).for_each(
//            |(train_out, output_one)| {
//                assert_eq!(
//                    char::from_u32(train_out.to_u32_label()),
//                    char::from_u32(output_one.to_u32_label()),
//                );
//            }
//        );
//
//        println!("{}", data_to_string(&output[..japanese_out.len()]))
//    }
//
//    for _ in 0..COUNT {
//        english_data(&mut rng, &mut data);
//        ai.state_mut().clear();
//
//        data.iter().for_each(|data_one| {
//            ai.input_next(data_one);
//        });
//
//        english_out.iter().zip(output.as_mut_slice()).for_each(
//            |(_, output_one)| {
//                ai.output_next(output_one);
//            }
//        );
//
//        english_out.iter().zip(output.as_slice()).for_each(
//            |(train_out, output_one)| {
//                assert_eq!(
//                    char::from_u32(train_out.to_u32_label()),
//                    char::from_u32(output_one.to_u32_label()),
//                );
//            }
//        );
//
//        println!("{}", data_to_string(&output[..english_out.len()]))
//    }
//}
//
//#[test]
//fn chobit_seq_ai_test_3() {
//    const OUT: usize = 11;
//    const MIDDLE: usize = 7;
//    const IN: usize = 5;
//
//    let mut rng = ChobitRand::new("chobit_seq_ai_test_3".as_bytes());
//
//    let mut ai_1 =
//        ChobitSeqAI::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//    ai_1.for_each_weight_mut(|val| {
//        *val = rand_num(&mut rng);
//    });
//
//    let mut ai_2 =
//        ChobitSeqAI::<OUT, MIDDLE, IN>::new(Activation::SoftSign);
//    ai_2.for_each_weight_mut(|val| {
//        *val = rand_num(&mut rng);
//    });
//
//    assert_ne!(ai_1, ai_2);
//
//    let mut vec = Vec::<f32>::new();
//    ai_1.for_each_weight(|val| {
//        vec.push(*val);
//    });
//
//    let mut vec_iter = vec.iter();
//    ai_2.for_each_weight_mut(|val| {
//        *val = *vec_iter.next().unwrap();
//    });
//
//    assert_eq!(ai_1, ai_2);
//}
