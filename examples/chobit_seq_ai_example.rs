extern crate chobitlibs;

use chobitlibs::chobit_ai::{
    MathVec,
    Activation,
    ChobitSeqAI,
    ChobitMLSeqAI,
    MLSeqAICache
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

const JAPANESE_MESSAGE: &str = "これは日本語です。";
const ENGLISH_MESSAGE: &str = "This is English.";

fn main() {
    const OUT: usize = 32;
    const MIDDLE: usize = 64;
    const IN: usize = 32;

    const MAX_WORD_LEN: usize = 10;
    let max_message_len = JAPANESE_MESSAGE.len().max(ENGLISH_MESSAGE.len());

    let mut rng = ChobitRand::new(b"ChobitSeqAI Example");

    let mut ai = ChobitSeqAI::<OUT, MIDDLE, IN>::new(Activation::SoftSign);

    // Randomises weights.
    ai.for_each_weight_mut(|weight| {
        *weight = ((rng.next_f64() as f32) * 2.0) - 1.0;
    });

    let mut input = vec![MathVec::<IN>::new(); MAX_WORD_LEN];
    let mut output = vec![MathVec::<OUT>::new(); max_message_len];
    let initial_state = MathVec::<MIDDLE>::new();

    let mut ai = ChobitMLSeqAI::<OUT, MIDDLE, IN>::new(ai);
    let mut cache = MLSeqAICache::<OUT, MIDDLE, IN>::new(
        MAX_WORD_LEN,
        max_message_len
    );

    let mut input_error = vec![MathVec::<IN>::new(); MAX_WORD_LEN];
    let mut output_error = vec![MathVec::<OUT>::new(); max_message_len];
    let mut prev_state_error = MathVec::<MIDDLE>::new();

    const EPOCH: usize = 10000;
    const BATCH_SIZE: usize = 10;
    const RATE: f32 = 0.01;

    let japanese_message_len = JAPANESE_MESSAGE.chars().count();
    let english_message_len = ENGLISH_MESSAGE.chars().count();

    for _ in 0..EPOCH {
        for _ in 0..BATCH_SIZE {
            //--- Learns Japanese ---//
            let string = gen_word(japanese_letter, &mut rng, MAX_WORD_LEN);

            write_string_to_slice(&string, &mut input);
            write_string_to_slice(&JAPANESE_MESSAGE, &mut output);

            // Writes cache.
            ai.ready(
                &input[..string.chars().count()],
                &initial_state,
                japanese_message_len,
                &mut cache
            );

            // Calculates error.
            cache.calc_output_error(
                &output[..japanese_message_len],
                &mut output_error[..japanese_message_len]
            );

            // Studies.
            ai.study(
                &output_error[..japanese_message_len],
                &cache,
                &mut input_error,
                &mut prev_state_error
            );

            //--- Learns English ---//
            let string = gen_word(english_letter, &mut rng, MAX_WORD_LEN);

            write_string_to_slice(&string, &mut input);
            write_string_to_slice(&ENGLISH_MESSAGE, &mut output);

            // Writes cache.
            ai.ready(
                &input[..string.chars().count()],
                &initial_state,
                english_message_len,
                &mut cache
            );

            // Calculates error.
            cache.calc_output_error(
                &output[..english_message_len],
                &mut output_error[..english_message_len]
            );

            // Studies.
            ai.study(
                &output_error[..english_message_len],
                &cache,
                &mut input_error,
                &mut prev_state_error
            );
        }

        // Updates weights.
        ai.update(RATE);
    }

    // Unwrap AI.
    let mut ai = ai.drop();

    let mut output = MathVec::<OUT>::new();

    // Tests Japanese.
    for _ in 0..10 {
        let string = gen_word(japanese_letter, &mut rng, MAX_WORD_LEN);

        write_string_to_slice(&string, &mut input);

        // Initializes state.
        ai.state_mut().copy_from(&initial_state);
        
        // Inputs for each one.
        input.iter().for_each(|input_one| {
            ai.input_next(input_one);
        });

        // Outputs for each one.
        JAPANESE_MESSAGE.chars().for_each(|c| {
            ai.output_next(&mut output);

            assert_eq!(output.to_u32_label(), c as u32);
        });
    }

    // Tests English.
    for _ in 0..10 {
        let string = gen_word(english_letter, &mut rng, MAX_WORD_LEN);

        write_string_to_slice(&string, &mut input);

        // Initializes state.
        ai.state_mut().copy_from(&initial_state);
        
        // Inputs for each one.
        input.iter().for_each(|input_one| {
            ai.input_next(input_one);
        });

        // Outputs for each one.
        ENGLISH_MESSAGE.chars().for_each(|c| {
            ai.output_next(&mut output);

            assert_eq!(output.to_u32_label(), c as u32);
        });
    }
}
