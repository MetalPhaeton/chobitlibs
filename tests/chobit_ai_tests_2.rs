extern crate chobitlibs;

use chobitlibs::chobit_ai::*;
use chobitlibs::chobit_rand::*;

#[inline]
fn rand_num(rng: &mut ChobitRand) -> f32 {
    ((rng.next_f64() * 2.0) - 1.0) as f32
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

fn gen_data_set_1<const OUT: usize, const IN: usize>(
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

fn gen_gru_gate<const OUT: usize, const IN: usize>(
    rng: &mut ChobitRand,
) -> GRUGate<OUT, IN> {
    let mut ret = GRUGate::<OUT, IN>::new(Activation::SoftSign);

    ret.x_matrix_mut().iter_mut().for_each(
        |weights| {
            weights.w_mut().iter_mut().for_each(
                |x| {*x = rand_num(rng);}
            );
            *weights.b_mut() = rand_num(rng);
        }
    );

    ret.s_matrix_mut().iter_mut().for_each(
        |weights| {
            weights.w_mut().iter_mut().for_each(
                |x| {*x = rand_num(rng);}
            );
            *weights.b_mut() = rand_num(rng);
        }
    );

    ret
}

fn gen_state<const OUT: usize>(value: f32) -> MathVec<OUT> {
    let mut ret = MathVec::<OUT>::new();

    ret.iter_mut().for_each(|x| {*x = value;});

    ret
}

#[test]
fn gru_gate_tests() {
    const OUT: usize = 15;
    const IN: usize = 10;
    const DATA_SET_SIZE: usize = 50;

    let mut rng = ChobitRand::new("gru_gate_tests_test".as_bytes());

    let mut data_set = gen_data_set_1::<OUT, IN>(&mut rng, DATA_SET_SIZE);

    let mut gate = gen_gru_gate::<OUT, IN>(&mut rng);
    let state = gen_state(0.5);

    fn print(
        data_set: &Vec<(MathVec<OUT>, MathVec<IN>)>,
        gate: &mut GRUGate<OUT, IN>,
        state: &MathVec<OUT>
    ) {
        let mut total: f32 = 0.0;
        let mut output = MathVec::<OUT>::new();

        for data in data_set {
            output.copy_from(gate.calc(&data.1, state));

            output -= &data.0;
            output.iter().for_each(|x| total += (*x).max(-(*x)));
        }

        println!("loss: {}", total / ((data_set.len() * OUT) as f32));
        println!("----------");
    }

    print(&data_set, &mut gate, &state);

    const EPOCH: usize = 1000;
    const RATE: f32 = 0.01;

    let mut output = MathVec::<OUT>::new();
    for _ in 0..EPOCH {
        rng.shuffle(&mut data_set);

        for data in &data_set {
            output.copy_from(gate.calc(&data.1, &state));

            output -= &data.0;

            let _ = gate.study(&output, &data.1, &state);
        }

        gate.update(RATE);
    }

    print(&data_set, &mut gate, &state);
}

fn gen_gru_layer<const OUT: usize, const IN: usize>(
    rng: &mut ChobitRand,
) -> GRULayer<OUT, IN> {
    let mut ret = GRULayer::<OUT, IN>::new();

    ret.z_gate_mut().x_matrix_mut().iter_mut().for_each(
        |weights| {
            weights.w_mut().iter_mut().for_each(
                |x| {*x = rand_num(rng);}
            );
            *weights.b_mut() = rand_num(rng);
        }
    );

    ret.z_gate_mut().s_matrix_mut().iter_mut().for_each(
        |weights| {
            weights.w_mut().iter_mut().for_each(
                |x| {*x = rand_num(rng);}
            );
            *weights.b_mut() = rand_num(rng);
        }
    );

    ret.r_gate_mut().x_matrix_mut().iter_mut().for_each(
        |weights| {
            weights.w_mut().iter_mut().for_each(
                |x| {*x = rand_num(rng);}
            );
            *weights.b_mut() = rand_num(rng);
        }
    );

    ret.r_gate_mut().s_matrix_mut().iter_mut().for_each(
        |weights| {
            weights.w_mut().iter_mut().for_each(
                |x| {*x = rand_num(rng);}
            );
            *weights.b_mut() = rand_num(rng);
        }
    );

    ret.h_gate_mut().x_matrix_mut().iter_mut().for_each(
        |weights| {
            weights.w_mut().iter_mut().for_each(
                |x| {*x = rand_num(rng);}
            );
            *weights.b_mut() = rand_num(rng);
        }
    );

    ret.h_gate_mut().s_matrix_mut().iter_mut().for_each(
        |weights| {
            weights.w_mut().iter_mut().for_each(
                |x| {*x = rand_num(rng);}
            );
            *weights.b_mut() = rand_num(rng);
        }
    );

    ret
}

#[test]
fn gru_layer_test() {
    const OUT: usize = 15;
    const IN: usize = 10;
    const DATA_SET_SIZE: usize = 50;

    let mut rng = ChobitRand::new("gru_layer_tests_test".as_bytes());

    let mut data_set = gen_data_set_1::<OUT, IN>(&mut rng, DATA_SET_SIZE);

    let mut layer = gen_gru_layer::<OUT, IN>(&mut rng);
    let state = gen_state(0.5);

    fn print(
        data_set: &Vec<(MathVec<OUT>, MathVec<IN>)>,
        layer: &mut GRULayer<OUT, IN>,
        state: &MathVec<OUT>
    ) {
        let mut total: f32 = 0.0;
        let mut output = MathVec::<OUT>::new();

        for data in data_set {
            output.copy_from(layer.calc(&data.1, state));

            output -= &data.0;
            output.iter().for_each(|x| total += (*x).max(-(*x)));
        }

        println!("loss: {}", total / ((data_set.len() * OUT) as f32));
        println!("----------");
    }

    print(&data_set, &mut layer, &state);

    const EPOCH: usize = 1000;
    const RATE: f32 = 0.01;

    let mut output = MathVec::<OUT>::new();
    for _ in 0..EPOCH {
        rng.shuffle(&mut data_set);

        for data in &data_set {
            output.copy_from(layer.calc(&data.1, &state));

            output -= &data.0;

            let _ = layer.study(&output, &data.1, &state);
        }

        layer.update(RATE);
    }

    print(&data_set, &mut layer, &state);
}
