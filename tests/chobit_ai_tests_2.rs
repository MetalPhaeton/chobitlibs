extern crate chobitlibs;

use std::prelude::rust_2021::*;

use chobitlibs::chobit_ai::*;
use chobitlibs::chobit_rand::*;

// [0.0, 1.0] -> [-1.0, 1.0]
fn rand_num(rng: &mut ChobitRand) -> f32 {
    ((rng.next_f64() * 2.0) - 1.0) as f32
}

fn gen_weights_1<const N: usize>(rng: &mut ChobitRand) -> Weights<N> {
    Weights::<N>::new([0u8; N].map(|_| rand_num(rng)), rand_num(rng))
}

fn gen_weights_2<const N: usize>(rng: &mut ChobitRand) -> (Weights<N>, f32) {
    (gen_weights_1(rng), rand_num(rng))
}

fn gen_gru_neuron<const N: usize>(rng: &mut ChobitRand) -> GRUNeuron<N> {
    GRUNeuron::<N>::new(
        gen_weights_2::<N>(rng),
        gen_weights_2::<N>(rng),
        gen_weights_2::<N>(rng)
    )
}

fn gen_data_set_1<const N: usize>(
    rng: &mut ChobitRand,
    size: usize
) -> Vec<([f32; N], f32)> {
    let mut ret = Vec::<([f32; N], f32)>::with_capacity(size);

    for _ in 0..size {
        ret.push((
            [0u8; N].map(|_| rand_num(rng)),
            rand_num(rng)
        ));
    }

    ret
}

fn diff(x: f32, y: f32) -> f32 {
    let z = x - y;
    z.max(-z)
}

#[test]
fn neuron_test() {
    const N: usize = 5;
    const DATA_SET_SIZE: usize = 20;

    let mut rng = ChobitRand::new("neuron_test".as_bytes());

    let mut data_set = gen_data_set_1::<N>(&mut rng, DATA_SET_SIZE);

    let mut neuron = gen_gru_neuron::<N>(&mut rng);

    fn print(data_set: &Vec<([f32; N], f32)>, neuron: &GRUNeuron<N>) {
        let mut total_diff: f32 = 0.0;
        for data in data_set {
            let output = neuron.calc(&data.0, 0.0);
            total_diff += diff(output, data.1);
            //println!("{} | {} | {}", data.1, output, diff(output, data.1));
        }
        println!("diff: {}", total_diff / (DATA_SET_SIZE as f32));
    }

    print(&data_set, &neuron);

    println!("----------");

    const EPOCH: usize = 100000;
    const RATE: f32 = 0.001;

    for _ in 0..EPOCH {
        rng.shuffle(&mut data_set);

        for data in &data_set {
            let output = neuron.calc(&data.0, 0.0);
            let _ = neuron.study(output - data.1, &data.0, 0.0);
        }

        neuron.update(RATE);
    }

    print(&data_set, &neuron);
}

fn gen_gru_layer<const OUT: usize, const IN: usize>(
    rng: &mut ChobitRand
) -> GRULayer<OUT, IN> {
    GRULayer::<OUT, IN>::new(
        [0u8; OUT].map(|_| gen_gru_neuron::<IN>(rng)),
        [0u8; OUT].map(|_| gen_weights_1::<OUT>(rng))
    )
}

fn gen_data_set_2<const OUT: usize, const IN: usize>(
    rng: &mut ChobitRand,
    size: usize
) -> Vec<([f32; OUT], [f32; IN])> {
    let mut ret = Vec::<([f32; OUT], [f32; IN])>::with_capacity(size);

    for _ in 0..size {
        ret.push((
            [0u8; OUT].map(|_| rand_num(rng)),
            [0u8; IN].map(|_| rand_num(rng))
        ));
    }

    ret
}

fn diff_2<const N: usize>(x: &[f32; N], y: &[f32; N]) -> f32 {
    let mut total: f32 = 0.0;

    for i in 0..N {
        total += diff(x[i], y[i]);
    }

    total / (N as f32)
}

#[test]
fn gru_layer_test() {
    const OUT: usize = 7;
    const IN: usize = 5;

    const DATA_SET_SIZE: usize = 10;
    const INITIAL_STATE: [f32; OUT] = [0.3; OUT];

    let mut rng = ChobitRand::new("gru_layer_test".as_bytes());

    let mut data_set = gen_data_set_2::<OUT, IN>(&mut rng, DATA_SET_SIZE);

    let mut layer = gen_gru_layer::<OUT, IN>(&mut rng);

    fn print(
        data_set: &Vec<([f32; OUT], [f32; IN])>,
        layer: &GRULayer<OUT, IN>
    ) {
        let mut total: f32 = 0.0;

        for data in data_set {
            let output = layer.calc(&data.1, &INITIAL_STATE);
            total += diff_2(&output, &data.0);
        }

        println!("diff: {}", total / (DATA_SET_SIZE as f32));
    }

    print(&data_set, &layer);

    println!("----------");

    const EPOCH: usize = 10000;
    const RATE: f32 = 0.01;

    for _ in 0..EPOCH {
        rng.shuffle(&mut data_set);

        for data in &data_set {
            let mut loss = layer.calc(&data.1, &INITIAL_STATE);

            for i in 0..OUT {
                loss[i] -= data.0[i];
            }

            let _ = layer.study(&loss, &data.1, &INITIAL_STATE);
        }

        layer.update(RATE);
    }

    print(&data_set, &layer);
}

fn gen_encoder<const OUT: usize, const IN: usize>(
    rng: &mut ChobitRand
) -> ChobitEncoder<OUT, IN> {
    ChobitEncoder::<OUT, IN>::new(gen_gru_layer::<OUT, IN>(rng))
}
fn gen_data_set_3<const OUT: usize, const IN: usize>(
    rng: &mut ChobitRand,
    size: usize
) -> Vec<([f32; OUT], Vec<[f32; IN]>)> {
    let mut ret = Vec::<([f32; OUT], Vec<[f32; IN]>)>::with_capacity(size);

    for _ in 0..size {
        let input_size = (rng.next_u64() % 20 + 1) as usize;

        ret.push((
            [0u8; OUT].map(|_| rand_num(rng)),
            {
                let mut vec = Vec::<[f32; IN]>::with_capacity(input_size);
                for _ in 0..input_size {
                    vec.push([0u8; IN].map(|_| (rng.next_u64() % 20) as f32));
                }
                vec
            }
        ));
    }

    ret
}

#[test]
fn encoder_test() {
    const OUT: usize = 7;
    const IN: usize = 5;

    const DATA_SET_SIZE: usize = 10;
    const INITIAL_STATE: [f32; OUT] = [0.3; OUT];

    let mut rng = ChobitRand::new("encoder_test".as_bytes());

    let mut data_set = gen_data_set_3::<OUT, IN>(&mut rng, DATA_SET_SIZE);

    let mut encoder = gen_encoder::<OUT, IN>(&mut rng);

    fn print(
        data_set: &Vec<([f32; OUT], Vec<[f32; IN]>)>,
        encoder: &ChobitEncoder<OUT, IN>
    ) {
        let mut total: f32 = 0.0;

        for data in data_set {
            let output = encoder.calc(&mut data.1.iter(), &INITIAL_STATE);
            total += diff_2(&output, &data.0);
        }

        println!("diff: {}", total / (DATA_SET_SIZE as f32));
    }

    print(&data_set, &encoder);

    println!("----------");

    const EPOCH: usize = 1000;
    const RATE: f32 = 0.1;

    for _ in 0..EPOCH {
        rng.shuffle(&mut data_set);

        for data in &data_set {
            let _ = encoder.study(&data.0, &mut data.1.iter(), &INITIAL_STATE);
        }

        encoder.update(RATE);
    }

    print(&data_set, &encoder);
}

fn gen_output_layer<const OUT: usize, const IN: usize>(
    rng: &mut ChobitRand
) -> Layer<OUT, IN> {
    Layer::<OUT, IN>::new([0u8; OUT].map(
        |_| Neuron::new(gen_weights_1::<IN>(rng), Activation::SoftSign)
    ))
}

fn gen_decoder<const OUT: usize, const IN: usize>(
    rng: &mut ChobitRand
) -> ChobitDecoder<OUT, IN> {
    ChobitDecoder::<OUT, IN>::new(
        gen_gru_layer::<OUT, IN>(rng),
        gen_output_layer::<OUT, OUT>(rng)
    )
}

fn gen_data_set_4<const OUT: usize, const IN: usize>(
    rng: &mut ChobitRand,
    size: usize
) -> Vec<(Vec<[f32; OUT]>, [f32; IN])> {
    let mut ret = Vec::<(Vec<[f32; OUT]>, [f32; IN])>::with_capacity(size);

    for _ in 0..size {
        let output_size = (rng.next_u64() % 20 + 1) as usize;

        ret.push((
            {
                let mut vec = Vec::<[f32; OUT]>::with_capacity(output_size);
                for _ in 0..output_size {
                    vec.push([0u8; OUT].map(|_| rand_num(rng)));
                }
                vec
            },
            [0u8; IN].map(|_| rand_num(rng))
        ));
    }

    ret
}

fn diff_3<const N: usize>(x: &Vec<[f32; N]>, y: &Vec<[f32; N]>) -> f32 {
    let mut total_x: f32 = 0.0;
    let mut total_y: f32 = 0.0;

    let mut iter_x = x.iter();
    let mut iter_y = y.iter();

    let mut deno: f32 = 0.0;
    loop {
        let x_opt = iter_x.next();
        let y_opt = iter_y.next();

        if x_opt.is_none() && y_opt.is_none() {break;}

        total_x += match x_opt {
            Some(ary) => {
                let mut total: f32 = 0.0;
                ary.iter().for_each(|val| {total += *val;});

                total / (N as f32)
            },

            None => 0.0
        };

        total_y += match y_opt {
            Some(ary) => {
                let mut total: f32 = 0.0;
                ary.iter().for_each(|val| {total += *val;});

                total / (N as f32)
            },

            None => 0.0
        };

        deno += 1.0;
    }

    diff(total_x, total_y) / deno
}

#[test]
fn decoder_test() {
    const OUT: usize = 16;
    const IN: usize = 5;

    const DATA_SET_SIZE: usize = 10;
    //const INITIAL_STATE: [f32; OUT] = [0.3; OUT];

    let mut rng = ChobitRand::new("decoder_test".as_bytes());

    let mut data_set = gen_data_set_4::<OUT, IN>(&mut rng, DATA_SET_SIZE);

    let mut decoder = gen_decoder::<OUT, IN>(&mut rng);

    fn print(
        data_set: &Vec<(Vec<[f32; OUT]>, [f32; IN])>,
        decoder: &ChobitDecoder<OUT, IN>
    ) {
        let mut total: f32 = 0.0;

        for data in data_set {
            let data_len = data.0.len();
            let mut iter = decoder.start_calc(&data.1);

            let mut output = Vec::<[f32; OUT]>::with_capacity(data_len);
            for _ in 0..data_len {
                output.push(iter.next().unwrap());
            }

            total += diff_3(&output, &data.0);
        }

        println!("diff: {}", total / (DATA_SET_SIZE as f32));
    }

    print(&data_set, &decoder);

    println!("----------");

    const EPOCH: usize = 1000;
    const RATE: f32 = 0.01;

    for _ in 0..EPOCH {
        rng.shuffle(&mut data_set);

        for data in &data_set {
            let _ = decoder.study(&mut data.0.iter(), &data.1);
        }

        decoder.update(RATE);
    }

    print(&data_set, &decoder);
}

fn gen_ai<const OUT: usize, const MIDDLE: usize, const IN: usize>(
    rng: &mut ChobitRand
) -> ChobitSeqAI<OUT, MIDDLE, IN> {
    ChobitSeqAI::new(
        gen_decoder::<OUT, MIDDLE>(rng),
        gen_encoder::<MIDDLE, IN>(rng)
    )
}

// ----------------------------------------------------------------------------

const CHAR_SET_LEN: usize = 26 + 46;

static CHAR_SET: [char; CHAR_SET_LEN] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z',
    'あ', 'い', 'う', 'え', 'お',
    'か', 'き', 'く', 'け', 'こ',
    'さ', 'し', 'す', 'せ', 'そ',
    'た', 'ち', 'つ', 'て', 'と',
    'な', 'に', 'ぬ', 'ね', 'の',
    'は', 'ひ', 'ふ', 'へ', 'ほ',
    'ま', 'み', 'む', 'め', 'も',
    'や', 'ゆ', 'よ',
    'ら', 'り', 'る', 'れ', 'ろ',
    'わ', 'を', 'ん'
];

fn gen_string(rng: &mut ChobitRand, max_length: usize) -> String {
    let length = ((rng.next_u64() as usize) % (max_length - 1)) + 1;

    let mut ret = String::with_capacity(length);

    for _ in 0..length {
        ret.push(CHAR_SET[(rng.next_u64() as usize) % CHAR_SET_LEN]);
    }

    ret
}

fn text_to_vec(text: &String) -> Vec<[f32; 32]> {
    text.chars().map(|c| from_u32_label(c as u32)).collect()
}

fn vec_to_text(vec: &Vec<[f32; 32]>) -> String {
    vec.iter().map(
        |x| match char::from_u32(to_u32_label(x)) {
            Some(c) => c,
            None => '無'
        }
    ).collect()
}

fn gen_data_set_5(
    rng: &mut ChobitRand,
    size: usize,
    max_length: usize
) -> Vec<String> {
    let mut ret = Vec::<String>::with_capacity(size);

    for _ in 0..size {
        ret.push(gen_string(rng, max_length));
    }

    ret
}

#[test]
fn ai_test() {
    const OUT: usize = 32;
    const MIDDLE: usize = 128 + 16;
    const IN: usize = 32;

    const DATA_SET_SIZE: usize = 10;
    const MAX_TEXT_LENGTH: usize = 10;
    const INITIAL_STATE: [f32; MIDDLE] = [0.0; MIDDLE];

    let mut rng = ChobitRand::new("ai_test".as_bytes());

    let data_set = gen_data_set_5(&mut rng, DATA_SET_SIZE, MAX_TEXT_LENGTH);
    let mut data_set: Vec<Vec<[f32; 32]>> =
        data_set.iter().map(|text| text_to_vec(text)).collect();

    let mut ai = gen_ai::<OUT, MIDDLE, IN>(&mut rng);

    fn print(
        data_set: &Vec<Vec<[f32; 32]>>,
        ai: &ChobitSeqAI<OUT, MIDDLE, IN>
    ) {
        for data in data_set {
            let data_len = data.len();
            let mut iter = ai.start_calc(&mut data.iter(), &INITIAL_STATE);

            let mut output = Vec::<[f32; OUT]>::with_capacity(data_len);
            for _ in 0..data_len {
                output.push(iter.next().unwrap());
            }

            println!("{} | {}", vec_to_text(data), vec_to_text(&output));
        }
    }

    print(&data_set, &ai);

    println!("----------");

    const EPOCH: usize = 5000;
    const RATE: f32 = 0.002;

    for _ in 0..EPOCH {
        rng.shuffle(&mut data_set);

        for data in &data_set {
            let data_2 = data.clone();
            let _ = ai.study(
                &mut data.iter(),
                &mut data_2.iter(),
                &INITIAL_STATE
            );
        }

        ai.update(RATE);
    }

    print(&data_set, &ai);
}
