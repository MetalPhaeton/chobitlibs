extern crate chobitlibs;

use chobitlibs::chobit_ai::{
    MathVec,
    Activation,
    ChobitDecoder,
    ChobitMLDecoder,
    MLDecoderCache
};

use chobitlibs::chobit_rand::ChobitRand;

const JAPANESE_ID: char = '日';
const ENGLISH_ID: char = 'E';

const JAPANESE_MESSAGE: &str = "これは日本語です。";
const ENGLISH_MESSAGE: &str = "This is English.";

fn write_string_to_slice(string: &str, slice: &mut [MathVec<32>]) {
    string.chars().zip(slice.iter_mut()).for_each(|(c, s)| {
        s.load_u32_label(c as u32);
    });
}

fn main() {
    const OUT: usize = 32;
    const MIDDLE: usize = 64;
    const IN: usize = 32;

    let max_message_len = JAPANESE_MESSAGE.len().max(ENGLISH_MESSAGE.len());

    let mut rng = ChobitRand::new(b"ChobitDecoder Example");

    let mut decoder =
        ChobitDecoder::<OUT, MIDDLE, IN>::new(Activation::SoftSign);

    // Randomises weights.
    decoder.for_each_weight_mut(|weight| {
        *weight = ((rng.next_f64() as f32) * 2.0) - 1.0;
    });

    let mut input = MathVec::<IN>::new();
    let mut output = vec![MathVec::<OUT>::new(); max_message_len];
    let initial_state = MathVec::<MIDDLE>::new();

    let mut decoder = ChobitMLDecoder::<OUT, MIDDLE, IN>::new(decoder);
    let mut cache = MLDecoderCache::<OUT, MIDDLE, IN>::new(max_message_len);

    let mut input_error = MathVec::<IN>::new();
    let mut output_error = vec![MathVec::<OUT>::new(); max_message_len];
    let mut prev_state_error = MathVec::<MIDDLE>::new();

    const EPOCH: usize = 10000;
    const BATCH_SIZE: usize = 1;
    const RATE: f32 = 0.01;

    let japanese_message_len = JAPANESE_MESSAGE.chars().count();
    let english_message_len = ENGLISH_MESSAGE.chars().count();

    for _ in 0..EPOCH {
        for _ in 0..BATCH_SIZE {
            //--- Learns Japanese ---//
            input.load_u32_label(JAPANESE_ID as u32);

            write_string_to_slice(&JAPANESE_MESSAGE, &mut output);

            // Writes cache.
            decoder.ready(
                &input,
                &initial_state,
                japanese_message_len,
                &mut cache
            );

            // Calculates error.
            cache.calc_output_error(&output, &mut output_error);

            // Studies.
            decoder.study(
                &output_error[..japanese_message_len],
                &cache,
                &mut input_error,
                &mut prev_state_error
            );

            //--- Learns English ---//
            input.load_u32_label(ENGLISH_ID as u32);

            write_string_to_slice(&ENGLISH_MESSAGE, &mut output);

            // Writes cache.
            decoder.ready(
                &input,
                &initial_state,
                english_message_len,
                &mut cache
            );

            // Calculates error.
            cache.calc_output_error(&output, &mut output_error);

            // Studies.
            decoder.study(
                &output_error[..english_message_len],
                &cache,
                &mut input_error,
                &mut prev_state_error
            );
        }

        // Updates weights.
        decoder.update(RATE);
    }

    // Unwrap Decoder.
    let mut decoder = decoder.drop();

    let mut output = MathVec::<OUT>::new();

    // Tests Japanese.
    // Sets input.
    input.load_u32_label(JAPANESE_ID as u32);
    decoder.input_mut().copy_from(&input);

    // Initializes state.
    decoder.state_mut().copy_from(&initial_state);

    // Outputs for each one.
    JAPANESE_MESSAGE.chars().for_each(|c| {
        decoder.output_next(&mut output);

        assert_eq!(output.to_u32_label(), c as u32);
    });

    // Tests English.
    // Sets input.
    input.load_u32_label(ENGLISH_ID as u32);
    decoder.input_mut().copy_from(&input);

    // Initializes state.
    decoder.state_mut().copy_from(&initial_state);

    // Outputs for each one.
    ENGLISH_MESSAGE.chars().for_each(|c| {
        decoder.output_next(&mut output);

        assert_eq!(output.to_u32_label(), c as u32);
    });
}
