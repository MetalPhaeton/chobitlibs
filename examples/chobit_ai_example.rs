extern crate chobitlibs;

use chobitlibs::chobit_ai::{
    MathVec,
    Activation,
    ChobitAi,
    ChobitMlAi,
    MlAiCache
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

fn main() {
    const OUT: usize = 32;
    const MIDDLE: usize = 64;
    const IN: usize = 32;

    let mut rng = ChobitRand::new(b"ChobitAi Example");

    let mut ai = ChobitAi::<OUT, MIDDLE, IN>::new(Activation::SoftSign);

    // Randomises weights.
    ai.for_each_weight_mut(|weight| {
        *weight = ((rng.next_f64() as f32) * 2.0) - 1.0;
    });

    let mut input = MathVec::<IN>::new();
    let mut output = MathVec::<OUT>::new();

    let mut ai = ChobitMlAi::<OUT, MIDDLE, IN>::new(ai);
    let mut cache = MlAiCache::<OUT, MIDDLE, IN>::new();

    let mut input_error = MathVec::<IN>::new();
    let mut output_error = MathVec::<OUT>::new();

    const EPOCH: usize = 1000;
    const BATCH_SIZE: usize = 100;
    const RATE: f32 = 0.01;

    for _ in 0..EPOCH {
        for _ in 0..BATCH_SIZE {
            //--- Learns Japanese ---//
            input.load_u32_label(japanese_letter(&mut rng) as u32);
            output.load_u32_label(JAPANESE_ID as u32);

            // Writes cache.
            ai.ready(&input, &mut cache);

            // Calculates error.
            cache.calc_output_error(&output, &mut output_error);

            // Studies.
            ai.study(&output_error, &cache, &mut input_error);

            //--- Learns English ---//
            input.load_u32_label(english_letter(&mut rng) as u32);
            output.load_u32_label(ENGLISH_ID as u32);

            // Writes cache.
            ai.ready(&input, &mut cache);

            // Calculates error.
            cache.calc_output_error(&output, &mut output_error);

            // Studies.
            ai.study(&output_error, &cache, &mut input_error);
        }

        // Updates weights.
        ai.update(RATE);
    }

    // Unwrap AI.
    let ai = ai.drop();

    let mut tmpbuf = MathVec::<MIDDLE>::new();

    // Tests Japanese.
    for _ in 0..10 {
        input.load_u32_label(japanese_letter(&mut rng) as u32);

        ai.calc(&input, &mut output, &mut tmpbuf);

        assert_eq!(output.to_u32_label(), JAPANESE_ID as u32);
    }

    // Tests English.
    for _ in 0..10 {
        input.load_u32_label(english_letter(&mut rng) as u32);

        ai.calc(&input, &mut output, &mut tmpbuf);

        assert_eq!(output.to_u32_label(), ENGLISH_ID as u32);
    }
}
