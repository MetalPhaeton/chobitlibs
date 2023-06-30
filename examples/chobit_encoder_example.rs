extern crate chobitlibs;

use chobitlibs::chobit_ai::{
    MathVec,
    Activation,
    ChobitEncoder,
    ChobitMlEncoder,
    MlEncoderCache
};

use chobitlibs::chobit_rand::ChobitRand;

fn japanese_letter(rng: &mut ChobitRand) -> char {
    let letters = [
        'あ', 'い', 'う', 'え', 'お',
        'か', 'き', 'く', 'け', 'こ',
        'さ', 'し', 'す', 'せ', 'そ'
    ];

    letters[(rng.next_u64() as usize) % letters.len()]
}

fn english_letter(rng: &mut ChobitRand) -> char {
    let letters = [
        'a', 'b', 'c', 'd', 'e',
        'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o'
    ];

    letters[(rng.next_u64() as usize) % letters.len()]
}

const JAPANESE_ID: char = '日';
const ENGLISH_ID: char = 'E';

fn gen_word(
    f: fn(&mut ChobitRand) -> char,
    rng: &mut ChobitRand,
    max_len: usize
) -> String {
    let word_len = ((rng.next_u64() as usize) % max_len) + 1;

    let mut ret = String::with_capacity(word_len);

    for _ in 0..word_len {
        ret.push(f(rng));
    }

    ret
}

fn write_string_to_slice(string: &str, slice: &mut [MathVec<32>]) {
    string.chars().zip(slice.iter_mut()).for_each(|(c, s)| {
        s.load_u32_label(c as u32);
    });
}

fn main() {
    const OUT: usize = 32;
    const MIDDLE: usize = 64;
    const IN: usize = 32;

    const MAX_WORD_LEN: usize = 10;

    let mut rng = ChobitRand::new(b"ChobitEncoder Example");

    let mut encoder =
        ChobitEncoder::<OUT, MIDDLE, IN>::new(Activation::SoftSign);

    // Randomises weights.
    encoder.for_each_weight_mut(|weight| {
        *weight = ((rng.next_f64() as f32) * 2.0) - 1.0;
    });

    let mut input = vec![MathVec::<IN>::new(); MAX_WORD_LEN];
    let mut output = MathVec::<OUT>::new();
    let initial_state = MathVec::<MIDDLE>::new();

    let mut encoder = ChobitMlEncoder::<OUT, MIDDLE, IN>::new(encoder);
    let mut cache = MlEncoderCache::<OUT, MIDDLE, IN>::new(MAX_WORD_LEN);

    let mut input_error = vec![MathVec::<IN>::new(); MAX_WORD_LEN];
    let mut output_error = MathVec::<OUT>::new();
    let mut prev_state_error = MathVec::<MIDDLE>::new();

    const EPOCH: usize = 1000;
    const BATCH_SIZE: usize = 100;
    const RATE: f32 = 0.01;

    for _ in 0..EPOCH {
        for _ in 0..BATCH_SIZE {
            //--- Learns Japanese ---//
            let string = gen_word(japanese_letter, &mut rng, MAX_WORD_LEN);

            write_string_to_slice(&string, &mut input);

            output.load_u32_label(JAPANESE_ID as u32);

            // Writes cache.
            encoder.ready(
                &input[..string.chars().count()],
                &initial_state,
                &mut cache
            );

            // Calculates error.
            cache.calc_output_error(&output, &mut output_error);

            // Studies.
            encoder.study(
                &output_error,
                &cache,
                &mut input_error,
                &mut prev_state_error
            );

            //--- Learns English ---//
            let string = gen_word(english_letter, &mut rng, MAX_WORD_LEN);

            write_string_to_slice(&string, &mut input);

            output.load_u32_label(ENGLISH_ID as u32);

            // Writes cache.
            encoder.ready(
                &input[..string.chars().count()],
                &initial_state,
                &mut cache
            );

            // Calculates error.
            cache.calc_output_error(&output, &mut output_error);

            // Studies.
            encoder.study(
                &output_error,
                &cache,
                &mut input_error,
                &mut prev_state_error
            );
        }

        // Updates weights.
        encoder.update(RATE);
    }

    // Unwrap Encoder.
    let mut encoder = encoder.drop();

    // Tests Japanese.
    for _ in 0..10 {
        let string = gen_word(japanese_letter, &mut rng, MAX_WORD_LEN);

        write_string_to_slice(&string, &mut input);

        // Initializes state.
        encoder.state_mut().copy_from(&initial_state);

        // Inputs for each one.
        input[..string.chars().count()].iter().for_each(|input_one| {
            encoder.input_next(input_one)
        });

        // Outputs.
        encoder.output(&mut output);

        assert_eq!(output.to_u32_label(), JAPANESE_ID as u32);
    }

    // Tests English.
    for _ in 0..10 {
        let string = gen_word(english_letter, &mut rng, MAX_WORD_LEN);

        write_string_to_slice(&string, &mut input);

        // Initializes state.
        encoder.state_mut().copy_from(&initial_state);

        // Inputs for each one.
        input[..string.chars().count()].iter().for_each(|input_one| {
            encoder.input_next(input_one)
        });

        // Outputs.
        encoder.output(&mut output);

        assert_eq!(output.to_u32_label(), ENGLISH_ID as u32);
    }
}
