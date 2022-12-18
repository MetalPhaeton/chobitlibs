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
        to_from_label_test_core!(u8, rng, to_u8_label, from_u8_label);
        to_from_label_test_core!(u16, rng, to_u16_label, from_u16_label);
        to_from_label_test_core!(u32, rng, to_u32_label, from_u32_label);
        to_from_label_test_core!(u64, rng, to_u64_label, from_u64_label);
        to_from_label_test_core!(u128, rng, to_u128_label, from_u128_label);
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
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 1.0});
        let mut weights_1 = Weights::new(vec_1, 1.0);

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});
        let weights_2 = Weights::new(vec_2, 2.0);

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});
        let weights_3 = Weights::new(vec_3, 3.0);

        let weights_4 = &weights_1 + &weights_2;
        assert_eq!(weights_3, weights_4);

        weights_1 +=  &weights_2;
        assert_eq!(weights_1, weights_4);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});
        let mut weights_1 = Weights::new(vec_1, 3.0);

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 1.0});
        let weights_2 = Weights::new(vec_2, 1.0);

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});
        let weights_3 = Weights::new(vec_3, 2.0);

        let weights_4 = &weights_1 - &weights_2;
        assert_eq!(weights_3, weights_4);

        weights_1 -=  &weights_2;
        assert_eq!(weights_1, weights_4);
    }
}

#[test]
fn weights_test_2() {
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});
        let mut weights_1 = Weights::new(vec_1, 3.0);

        let scalar: f32 = 2.0;

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 6.0});
        let weights_2 = Weights::new(vec_2, 6.0);

        let weights_3 = &weights_1 * scalar;
        assert_eq!(weights_2, weights_3);

        weights_1 *= scalar;
        assert_eq!(weights_1, weights_3);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 6.0});
        let mut weights_1 = Weights::new(vec_1, 6.0);

        let scalar: f32 = 2.0;

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});
        let weights_2 = Weights::new(vec_2, 3.0);

        let weights_3 = &weights_1 / scalar;
        assert_eq!(weights_2, weights_3);

        weights_1 /= scalar;
        assert_eq!(weights_1, weights_3);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 12.0});
        let mut weights_1 = Weights::new(vec_1, 12.0);

        let scalar: f32 = 10.0;

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});
        let weights_2 = Weights::new(vec_2, 2.0);

        let weights_3 = &weights_1 % scalar;
        assert_eq!(weights_2, weights_3);

        weights_1 %= scalar;
        assert_eq!(weights_1, weights_3);
    }
}

#[test]
fn weights_test_3() {
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});
        let weights_1 = Weights::new(vec_1, 2.0);

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});
        let weights_2 = Weights::new(vec_2, 3.0);

        let scalar_1: f32 = 2.0 * 3.0 * 11.0;

        let scalar_2 = &weights_1 * &weights_2;
        assert_eq!(scalar_2, scalar_1);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});
        let weights_1 = Weights::new(vec_1, 2.0);

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});

        let scalar_1: f32 = (2.0 * 3.0 * 10.0) + 2.0;

        let scalar_2 = &weights_1 * &vec_2;
        assert_eq!(scalar_2, scalar_1);

        let scalar_3 = &vec_2 * &weights_1;
        assert_eq!(scalar_3, scalar_1);
    }
}

#[test]
fn weights_test_4() {
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});
        let mut weights_1 = Weights::new(vec_1, 2.0);

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});
        let weights_2 = Weights::new(vec_2, 3.0);

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_slice().iter_mut().for_each(|x| {*x = 6.0});
        let weights_3 = Weights::new(vec_3, 6.0);

        let weights_4 = weights_1.pointwise_mul(&weights_2);
        assert_eq!(weights_4, weights_3);

        weights_1.pointwise_mul_assign(&weights_2);
        assert_eq!(weights_1, weights_3);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 6.0});
        let mut weights_1 = Weights::new(vec_1, 6.0);

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});
        let weights_2 = Weights::new(vec_2, 2.0);

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});
        let weights_3 = Weights::new(vec_3, 3.0);

        let weights_4 = weights_1.pointwise_div(&weights_2);
        assert_eq!(weights_4, weights_3);

        weights_1.pointwise_div_assign(&weights_2);
        assert_eq!(weights_1, weights_3);
    }
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 12.0});
        let mut weights_1 = Weights::new(vec_1, 12.0);

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 10.0});
        let weights_2 = Weights::new(vec_2, 10.0);

        let mut vec_3 = MathVec::<10>::new();
        vec_3.as_mut_slice().iter_mut().for_each(|x| {*x = 2.0});
        let weights_3 = Weights::new(vec_3, 2.0);

        let weights_4 = weights_1.pointwise_rem(&weights_2);
        assert_eq!(weights_4, weights_3);

        weights_1.pointwise_rem_assign(&weights_2);
        assert_eq!(weights_1, weights_3);
    }
}

#[test]
fn weights_test_5() {
    {
        let mut vec_1 = MathVec::<10>::new();
        vec_1.as_mut_slice().iter_mut().for_each(|x| {*x = 1.0});
        let mut weights_1 = Weights::new(vec_1, 2.0);

        let mut vec_2 = MathVec::<10>::new();
        vec_2.as_mut_slice().iter_mut().for_each(|x| {*x = 3.0});
        let weights_2 = Weights::new(vec_2, 4.0);

        weights_1.copy_from(&weights_2);
        assert_eq!(weights_1, weights_2);
    }
}

#[inline]
fn rand_num(rng: &mut ChobitRand) -> f32 {
    ((rng.next_f64() * 2.0) - 1.0) as f32
}

fn gen_neuron<const N: usize>(
    rng: &mut ChobitRand,
) -> Neuron<N> {
    let mut ret = Neuron::<N>::new(Activation::SoftSign);

    ret.weights_mut().w_mut().iter_mut().for_each(|w| *w = rand_num(rng));
    *ret.weights_mut().b_mut() = rand_num(rng);

    ret
}

fn gen_data_set_1<const N: usize>(
    rng: &mut ChobitRand,
    size: usize
) -> Vec<(f32, MathVec<N>)> {
    let mut param = MathVec::<N>::new();
    param.iter_mut().for_each(|x| *x = rand_num(rng));

    let mut ret = Vec::<(f32, MathVec<N>)>::with_capacity(size);

    let activation = Activation::SoftSign;

    for _ in 0..size {
        let mut v = MathVec::<N>::new();
        v.iter_mut().for_each(|x| *x = rand_num(rng));

        let ans = activation.activate(&param * &v);

        ret.push((ans, v))
    }

    ret
}

#[test]
fn neuron_test() {
    const N: usize = 10;
    const DATA_SET_SIZE: usize = 200;

    let mut rng = ChobitRand::new("neuron_test".as_bytes());

    let mut data_set = gen_data_set_1::<N>(&mut rng, DATA_SET_SIZE);

    let mut neuron = gen_neuron::<N>(&mut rng);

    fn print(data_set: &Vec<(f32, MathVec<N>)>, neuron: &Neuron<N>) {
        let mut total: f32 = 0.0;

        for data in data_set {
            let output = neuron.calc(&data.1);

            let diff = output - data.0;

            total += diff.max(-diff)
        }

        println!("loss: {}", total / (data_set.len() as f32));
        println!("----------");
    }

    print(&data_set, &neuron);

    const EPOCH: usize = 3000;
    const RATE: f32 = 0.01;

    for _ in 0..EPOCH {
        rng.shuffle(&mut data_set);

        for data in &data_set {
            let output = neuron.calc(&data.1);

            let diff = output - data.0;

            let _ = neuron.study(diff, &data.1);
        }

        neuron.update(RATE);
    }

    print(&data_set, &neuron);
}

fn gen_matrix<const OUT: usize, const IN: usize>(
    rng: &mut ChobitRand,
) -> Vec<MathVec<IN>> {
    let mut ret = Vec::<MathVec<IN>>::with_capacity(OUT);

    for _ in 0..OUT {
        let mut vec = MathVec::<IN>::new();
        vec.iter_mut().for_each(|x| *x = rand_num(rng));

        ret.push(vec);
    }

    ret
}

fn gen_data_set_2<const OUT: usize, const IN: usize>(
    rng: &mut ChobitRand,
    size: usize
) -> Vec<(MathVec<OUT>, MathVec<IN>)> {
    let mut ret = Vec::<(MathVec<OUT>, MathVec<IN>)>::with_capacity(size);

    let param = gen_matrix::<OUT, IN>(rng);

    let activation = Activation::SoftSign;

    for _ in 0..size {
        let mut train_in = MathVec::<IN>::new();
        train_in.iter_mut().for_each(|x| *x = rand_num(rng));

        let mut train_out = MathVec::<OUT>::new();

        for i in 0..OUT {
            train_out[i] = activation.activate(&param[i] * &train_in);
        }

        ret.push((train_out, train_in));
    }

    ret
}

fn gen_layer<const OUT: usize, const IN: usize>(
    rng: &mut ChobitRand
) -> Layer<OUT, IN> {
    let mut ret = Layer::new(Activation::SoftSign);

    ret.neurons_mut().iter_mut().for_each(
        |neuron| {
            neuron.weights_mut().w_mut().iter_mut().for_each(
                |x| *x = rand_num(rng)
            );

            *neuron.weights_mut().b_mut() = rand_num(rng);
        }
    );

    ret
}

#[test]
fn layer_test() {
    const OUT: usize = 15;
    const IN: usize = 10;
    const DATA_SET_SIZE: usize = 50;

    let mut rng = ChobitRand::new("layer_test".as_bytes());

    let mut data_set = gen_data_set_2::<OUT, IN>(&mut rng, DATA_SET_SIZE);

    let mut layer = gen_layer::<OUT, IN>(&mut rng);

    fn print(
        data_set: &Vec<(MathVec<OUT>, MathVec<IN>)>,
        layer: &Layer<OUT, IN>
    ) {
        let mut total: f32 = 0.0;
        let mut output = MathVec::<OUT>::new();

        for data in data_set {
            output.clear();
            layer.calc(&data.1, &mut output);

            output -= &data.0;
            output.iter().for_each(|x| total += (*x).max(-(*x)));
        }

        println!("loss: {}", total / ((data_set.len() * OUT) as f32));
        println!("----------");
    }

    print(&data_set, &layer);

    const EPOCH: usize = 5000;
    const RATE: f32 = 0.01;

    let mut output = MathVec::<OUT>::new();
    for _ in 0..EPOCH {
        rng.shuffle(&mut data_set);

        for data in &data_set {
            output.clear();
            layer.calc(&data.1, &mut output);

            output -= &data.0;

            let _ = layer.study(&output, &data.1);
        }

        layer.update(RATE);
    }

    print(&data_set, &layer);
}

fn gen_ai<const OUT: usize, const MIDDLE: usize, const IN: usize>(
    rng: &mut ChobitRand
) -> ChobitAI<OUT, MIDDLE, IN> {
    let mut ret = ChobitAI::<OUT, MIDDLE, IN>::new(Activation::SoftSign);

    {
        ret.output_layer_mut().neurons_mut().iter_mut().for_each(
            |neuron| {
                let weights = neuron.weights_mut();
                weights.w_mut().iter_mut().for_each(|x| *x = rand_num(rng));
                *weights.b_mut() = rand_num(rng);
            }
        );
    }

    {
        ret.middle_layer_mut().neurons_mut().iter_mut().for_each(
            |neuron| {
                let weights = neuron.weights_mut();
                weights.w_mut().iter_mut().for_each(|x| *x = rand_num(rng));
                *weights.b_mut() = rand_num(rng);
            }
        );
    }

    ret
}

#[test]
fn ai_test() {
    const OUT: usize = 15;
    const MIDDLE: usize = 20;
    const IN: usize = 10;
    const DATA_SET_SIZE: usize = 50;

    let mut rng = ChobitRand::new("ai_test".as_bytes());

    let mut data_set = gen_data_set_2::<OUT, IN>(&mut rng, DATA_SET_SIZE);

    let mut ai = gen_ai::<OUT, MIDDLE, IN>(&mut rng);

    fn print(
        data_set: &Vec<(MathVec<OUT>, MathVec<IN>)>,
        ai: &mut ChobitAI<OUT, MIDDLE, IN>
    ) {
        let mut total: f32 = 0.0;
        let mut output = MathVec::<OUT>::new();

        for data in data_set {
            output.clear();
            ai.calc(&data.1, &mut output);

            output -= &data.0;
            output.iter().for_each(|x| total += (*x).max(-(*x)));
        }

        println!("loss: {}", total / ((data_set.len() * OUT) as f32));
        println!("----------");
    }

    print(&data_set, &mut ai);

    const EPOCH: usize = 2500;
    const RATE: f32 = 0.02;

    for _ in 0..EPOCH {
        rng.shuffle(&mut data_set);

        for data in &data_set {
            let _ = ai.study(&data.0, &data.1);
        }

        ai.update(RATE);
    }

    print(&data_set, &mut ai);
}

