extern crate chobitlibs;

use chobitlibs::chobit_ai::*;
use chobitlibs::chobit_rand::*;

use std::mem::size_of;

//==================//
// ChobitAI Example //
//==================//

const NUM_CHAR_BITS: usize = size_of::<u32>() * 8;
const MAX_WORD_SIZE: usize = 10;

const IN: usize = NUM_CHAR_BITS;
const MIDDLE: usize = 64;
const OUT: usize = NUM_CHAR_BITS;

#[inline]
fn gen_japanese_letter(rng: &mut ChobitRand) -> char {
    static LETTERS: [char; 25] = [
        'あ', 'い', 'う', 'え', 'お',
        'か', 'き', 'く', 'け', 'こ',
        'さ', 'し', 'す', 'せ', 'そ',
        'た', 'ち', 'つ', 'て', 'と',
        'な', 'に', 'ぬ', 'ね', 'の'
    ];

    LETTERS[(rng.next_u64() as usize) % LETTERS.len()]
}

#[inline]
fn gen_english_letter(rng: &mut ChobitRand) -> char {
    static LETTERS: [char; 26] = [
        'A', 'B', 'C', 'D', 'E',
        'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O',
        'P', 'Q', 'R', 'S', 'T',
        'U', 'V', 'W', 'X', 'Y', 'Z'
    ];

    LETTERS[(rng.next_u64() as usize) % LETTERS.len()]
}

#[inline]
fn gen_rune_letter(rng: &mut ChobitRand) -> char {
    static LETTERS: [char; 24] = [
        'ᚠ', 'ᚢ', 'ᚦ', 'ᚨ', 'ᚱ',
        'ᚲ', 'ᚷ', 'ᚹ', 'ᚺ', 'ᚾ',
        'ᛁ', 'ᛃ', 'ᛇ', 'ᛈ', 'ᛉ',
        'ᛊ', 'ᛏ', 'ᛒ', 'ᛖ', 'ᛗ',
        'ᛚ', 'ᛜ', 'ᛟ', 'ᛞ'
    ];

    LETTERS[(rng.next_u64() as usize) % LETTERS.len()]
}

#[inline]
fn gen_dummy_letter(rng: &mut ChobitRand) -> char {
    static LETTERS: [char; 10] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
    ];

    LETTERS[(rng.next_u64() as usize) % LETTERS.len()]
}

#[inline]
fn letter_to_vec(letter: char) -> MathVec<NUM_CHAR_BITS> {
    let mut ret = MathVec::<NUM_CHAR_BITS>::new();

    ret.load_u32_label(letter as u32);

    ret
}

#[inline]
fn vec_to_letter(vec: &MathVec<NUM_CHAR_BITS>) -> Option<char> {
    char::from_u32(vec.to_u32_label())
}

fn gen_word(
    rng: &mut ChobitRand,
    gen_letter: fn(&mut ChobitRand) -> char
) -> String {
    let num_letters: usize = ((rng.next_u64() as usize) % MAX_WORD_SIZE) + 1;
    let num_dummy_letters = num_letters / 2;
    let num_correct_letters = num_letters - num_dummy_letters;

    let mut letters = Vec::<char>::with_capacity(num_letters);

    for _ in 0..num_correct_letters {
        letters.push(gen_letter(rng));
    }

    for _ in 0..num_dummy_letters {
        letters.push(gen_dummy_letter(rng));
    }

    rng.shuffle(&mut letters);

    letters.iter().map(|c| c).collect()
}

#[inline]
fn word_to_data(word: &String) -> Vec<MathVec<IN>> {
    word.chars().map(|c| letter_to_vec(c)).collect()
}

const JAPANESE_FLAG: char = '日';
const ENGLISH_FLAG: char = 'E';
const RUNE_FLAG: char = 'ᚱ';

fn main() {
    //=======//
    // Ready //
    //=======//
    // Generates random number generator.
    let mut rng = ChobitRand::new("ChobitEncoder Example".as_bytes());

    // Generates AI.
    let mut encoder =
        ChobitEncoder::<OUT, MIDDLE, IN>::new(Activation::SoftSign);

    // Initializes weights with random number.
    encoder.lstm_mut().main_layer_mut().mut_weights().iter_mut().for_each(
        |val| {
            *val = ((rng.next_f64() as f32) * 2.0) - 1.0;  // (-1.0, 1.0)
        }
    );
    encoder.lstm_mut().f_gate_mut().mut_weights().iter_mut().for_each(
        |val| {
            *val = ((rng.next_f64() as f32) * 2.0) - 1.0;  // (-1.0, 1.0)
        }
    );
    encoder.lstm_mut().i_gate_mut().mut_weights().iter_mut().for_each(
        |val| {
            *val = ((rng.next_f64() as f32) * 2.0) - 1.0;  // (-1.0, 1.0)
        }
    );
    encoder.lstm_mut().o_gate_mut().mut_weights().iter_mut().for_each(
        |val| {
            *val = ((rng.next_f64() as f32) * 2.0) - 1.0;  // (-1.0, 1.0)
        }
    );
    encoder.output_layer_mut().mut_weights().iter_mut().for_each(
        |val| {
            *val = ((rng.next_f64() as f32) * 2.0) - 1.0;  // (-1.0, 1.0)
        }
    );

    //========================//
    // Befor machine learning //
    //========================//
    println!("//========================//");
    println!("// Befor machine learning //");
    println!("//========================//");

    // Buffer for output.
    let mut output = MathVec::<OUT>::new();

    // Previous state. (All are zero)
    let prev_state = MathVec::<MIDDLE>::new();

    // japanese_word -> JAPANESE_FLAG? ... No yet!
    for _ in 0..10 {
        let japanese_word = gen_word(&mut rng, gen_japanese_letter);

        let data = word_to_data(&japanese_word);

        // Initializes state.
        encoder.state_mut().copy_from(&prev_state);

        // Inputs data.
        data.iter().for_each(|vec| {
            encoder.input_next(vec);
        });

        // Outputs result.
        encoder.output(&mut output);

        assert_ne!(vec_to_letter(&output), Some(JAPANESE_FLAG));

        println!(
            "Generated japanese word: {}, flag: {:?}",
            japanese_word,
            vec_to_letter(&output)
        );
    }

    // english_word -> ENGLISH_FLAG? ... No yet!
    for _ in 0..10 {
        let english_word = gen_word(&mut rng, gen_english_letter);

        let data = word_to_data(&english_word);

        // Initializes state.
        encoder.state_mut().copy_from(&prev_state);

        // Inputs data.
        data.iter().for_each(|vec| {
            encoder.input_next(vec);
        });

        // Outputs result.
        encoder.output(&mut output);

        assert_ne!(vec_to_letter(&output), Some(ENGLISH_FLAG));

        println!(
            "Generated english word: {}, flag: {:?}",
            english_word,
            vec_to_letter(&output)
        );
    }

    // rune_word -> RUNE_FLAG? ... No yet!
    for _ in 0..10 {
        let rune_word = gen_word(&mut rng, gen_rune_letter);

        let data = word_to_data(&rune_word);

        // Initializes state.
        encoder.state_mut().copy_from(&prev_state);

        // Inputs data.
        data.iter().for_each(|vec| {
            encoder.input_next(vec);
        });

        // Outputs result.
        encoder.output(&mut output);

        assert_ne!(vec_to_letter(&output), Some(RUNE_FLAG));

        println!(
            "Generated rune word: {}, flag: {:?}",
            rune_word,
            vec_to_letter(&output)
        );
    }

    //==================//
    // Machine learning //
    //==================//
    // Wraps with ChobitMLAI for machine learning.
    let mut ml_encoder = ChobitMLEncoder::<OUT, MIDDLE, IN>::new(encoder);

    const EPOCH: usize = 1000;
    const BATCH_SIZE: usize = 20;
    const RATE: f32 = 0.01;

    // Flags to vectors.
    let japanese_flag = letter_to_vec(JAPANESE_FLAG);
    let english_flag = letter_to_vec(ENGLISH_FLAG);
    let rune_flag = letter_to_vec(RUNE_FLAG);

    // Machine learning.
    let mut input_error =
        MathVec::<IN>::new();  // Not use in this example.
    let mut state_error =
        MathVec::<MIDDLE>::new();  // Not use in this example.

    for _ in 0..EPOCH {
        for _ in 0..BATCH_SIZE {

            // Studies japanese word.
            let japanese_word = gen_word(&mut rng, gen_japanese_letter);

            let data = word_to_data(&japanese_word);

            ml_encoder.study(
                &data,
                &prev_state,
                &japanese_flag,
                &mut input_error,
                &mut state_error
            );

            // Studies english word.
            let english_word = gen_word(&mut rng, gen_english_letter);

            let data = word_to_data(&english_word);

            ml_encoder.study(
                &data,
                &prev_state,
                &english_flag,
                &mut input_error,
                &mut state_error
            );

            // Studies rune word.
            let rune_word = gen_word(&mut rng, gen_rune_letter);

            let data = word_to_data(&rune_word);

            ml_encoder.study(
                &data,
                &prev_state,
                &rune_flag,
                &mut input_error,
                &mut state_error
            );
        }

        // Updates weights.
        ml_encoder.update(RATE);
    }

    //========================//
    // After machine learning //
    //========================//
    println!("//========================//");
    println!("// After machine learning //");
    println!("//========================//");

    // Unwraps AI from ChobitMLAI.
    let mut encoder = ml_encoder.drop();

    // japanese_word -> JAPANESE_FLAG? ... yes!
    for _ in 0..10 {
        let japanese_word = gen_word(&mut rng, gen_japanese_letter);

        let data = word_to_data(&japanese_word);

        // Initializes state.
        encoder.state_mut().copy_from(&prev_state);

        // Inputs data.
        data.iter().for_each(|vec| {
            encoder.input_next(vec);
        });

        // Outputs result.
        encoder.output(&mut output);

        //assert_ne!(vec_to_letter(&output), Some(JAPANESE_FLAG));

        println!(
            "Generated japanese word: {}, flag: {:?}",
            japanese_word,
            vec_to_letter(&output)
        );
    }

    // english_word -> ENGLISH_FLAG? ... yes!
    for _ in 0..10 {
        let english_word = gen_word(&mut rng, gen_english_letter);

        let data = word_to_data(&english_word);

        // Initializes state.
        encoder.state_mut().copy_from(&prev_state);

        // Inputs data.
        data.iter().for_each(|vec| {
            encoder.input_next(vec);
        });

        // Outputs result.
        encoder.output(&mut output);

        //assert_ne!(vec_to_letter(&output), Some(ENGLISH_FLAG));

        println!(
            "Generated english word: {}, flag: {:?}",
            english_word,
            vec_to_letter(&output)
        );
    }

    // rune_word -> RUNE_FLAG? ... yes!
    for _ in 0..10 {
        let rune_word = gen_word(&mut rng, gen_rune_letter);

        let data = word_to_data(&rune_word);

        // Initializes state.
        encoder.state_mut().copy_from(&prev_state);

        // Inputs data.
        data.iter().for_each(|vec| {
            encoder.input_next(vec);
        });

        // Outputs result.
        encoder.output(&mut output);

        //assert_ne!(vec_to_letter(&output), Some(RUNE_FLAG));

        println!(
            "Generated rune word: {}, flag: {:?}",
            rune_word,
            vec_to_letter(&output)
        );
    }
}
