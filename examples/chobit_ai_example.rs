extern crate chobitlibs;

use chobitlibs::chobit_ai::*;
use chobitlibs::chobit_rand::*;

use std::mem::size_of;

//==================//
// ChobitAI Example //
//==================//

// AI to judge whether a word is Japanese, English, or Rune.

const NUM_CHAR_BITS: usize = size_of::<u32>() * 8;
const WORD_SIZE: usize = 4;

const IN: usize = NUM_CHAR_BITS * WORD_SIZE;
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
fn letter_to_vec(letter: char, vec: &mut MathVec<NUM_CHAR_BITS>) {
    vec.load_u32_label(letter as u32);
}

#[inline]
fn vec_to_letter(vec: &MathVec<NUM_CHAR_BITS>) -> Option<char> {
    char::from_u32(vec.to_u32_label())
}

fn gen_word(
    rng: &mut ChobitRand,
    gen_letter: fn(&mut ChobitRand) -> char,
    word: &mut String
) {
    word.clear();

    for _ in 0..WORD_SIZE {
        word.push(gen_letter(rng));
    }
}

fn word_to_data(
    word: &String,
    data: &mut MathVec<IN>,
    tmpbuf: &mut MathVec<NUM_CHAR_BITS>
) {
    data.chunks_mut(NUM_CHAR_BITS).zip(
        word.chars()
    ).for_each(
        |(chunk, letter)| {
            tmpbuf.load_u32_label(letter as u32);
            chunk.copy_from_slice(&tmpbuf);
        }
    );
}

const JAPANESE_FLAG: char = '日';
const ENGLISH_FLAG: char = 'E';
const RUNE_FLAG: char = 'ᚱ';

fn main() {
    //=======//
    // Ready //
    //=======//
    // Generates random number generator.
    let mut rng = ChobitRand::new("ChobitAI Example".as_bytes());

    // Generates AI.
    let mut ai = ChobitAI::<OUT, MIDDLE, IN >::new(Activation::SoftSign);

    // Randomise weights.
    ai.middle_layer_mut().mut_weights().iter_mut().for_each(|val| {
        *val = ((rng.next_f64() as f32) * 2.0) - 1.0;  // (-1.0, 1.0)
    });
    ai.output_layer_mut().mut_weights().iter_mut().for_each(|val| {
        *val = ((rng.next_f64() as f32) * 2.0) - 1.0;  // (-1.0, 1.0)
    });

    //========================//
    // Befor machine learning //
    //========================//
    println!("//========================//");
    println!("// Befor machine learning //");
    println!("//========================//");

    // Buffer for output.
    let mut word = String::with_capacity(WORD_SIZE);
    let mut input = MathVec::<IN>::new();
    let mut output = MathVec::<OUT>::new();

    let mut tmpbuf_1 = MathVec::<NUM_CHAR_BITS>::new();
    let mut tmpbuf_2 = MathVec::<MIDDLE>::new();

    // Japanese word -> JAPANESE_FLAG? ... No yet!
    for _ in 0..10 {
        gen_word(&mut rng, gen_japanese_letter, &mut word);

        word_to_data(&word, &mut input, &mut tmpbuf_1);

        // Outputs result.
        ai.calc(&input, &mut output, &mut tmpbuf_2);

        assert_ne!(vec_to_letter(&output), Some(JAPANESE_FLAG));

        println!(
            "Generated japanese word: {}, flag: {:?}",
            word,
            vec_to_letter(&output)
        );
    }

    // English word -> ENGLISH_FLAG? ... No yet!
    for _ in 0..10 {
        gen_word(&mut rng, gen_english_letter, &mut word);

        word_to_data(&word, &mut input, &mut tmpbuf_1);

        // Outputs result.
        ai.calc(&input, &mut output, &mut tmpbuf_2);

        assert_ne!(vec_to_letter(&output), Some(ENGLISH_FLAG));

        println!(
            "Generated english word: {}, flag: {:?}",
            word,
            vec_to_letter(&output)
        );
    }

    // Rune word -> RUNE_FLAG? ... No yet!
    for _ in 0..10 {
        gen_word(&mut rng, gen_rune_letter, &mut word);

        word_to_data(&word, &mut input, &mut tmpbuf_1);

        // Outputs result.
        ai.calc(&input, &mut output, &mut tmpbuf_2);

        assert_ne!(vec_to_letter(&output), Some(RUNE_FLAG));

        println!(
            "Generated rune word: {}, flag: {:?}",
            word,
            vec_to_letter(&output)
        );
    }

    //==================//
    // Machine learning //
    //==================//
    // Wraps with ChobitMLAI for machine learning.
    let mut ml_ai = ChobitMLAI::<OUT, MIDDLE, IN>::new(ai);
    let mut train_out = MathVec::<OUT>::new();

    const EPOCH: usize = 1000;
    const BATCH_SIZE: usize = 20;
    const RATE: f32 = 0.01;

    for _ in 0..EPOCH {
        for _ in 0..BATCH_SIZE {
            // Studies Japanese.
            gen_word(&mut rng, gen_japanese_letter, &mut word);
            word_to_data(&word, &mut input, &mut tmpbuf_1);

            letter_to_vec(JAPANESE_FLAG, &mut train_out);

            ml_ai.study(&input, &train_out);

            // Studies English.
            gen_word(&mut rng, gen_english_letter, &mut word);
            word_to_data(&word, &mut input, &mut tmpbuf_1);

            letter_to_vec(ENGLISH_FLAG, &mut train_out);

            ml_ai.study(&input, &train_out);

            // Studies Rune.
            gen_word(&mut rng, gen_rune_letter, &mut word);
            word_to_data(&word, &mut input, &mut tmpbuf_1);

            letter_to_vec(RUNE_FLAG, &mut train_out);

            ml_ai.study(&input, &train_out);
        }

        ml_ai.update(RATE);
    }

    //========================//
    // After machine learning //
    //========================//
    println!("//========================//");
    println!("// After machine learning //");
    println!("//========================//");

    let ai = ml_ai.drop();

    // Japanese word -> JAPANESE_FLAG? ... Yes!
    for _ in 0..10 {
        gen_word(&mut rng, gen_japanese_letter, &mut word);

        word_to_data(&word, &mut input, &mut tmpbuf_1);

        // Outputs result.
        ai.calc(&input, &mut output, &mut tmpbuf_2);

        assert_eq!(vec_to_letter(&output), Some(JAPANESE_FLAG));

        println!(
            "Generated japanese word: {}, flag: {:?}",
            word,
            vec_to_letter(&output)
        );
    }

    // English word -> ENGLISH_FLAG? ... Yes!
    for _ in 0..10 {
        gen_word(&mut rng, gen_english_letter, &mut word);

        word_to_data(&word, &mut input, &mut tmpbuf_1);

        // Outputs result.
        ai.calc(&input, &mut output, &mut tmpbuf_2);

        assert_eq!(vec_to_letter(&output), Some(ENGLISH_FLAG));

        println!(
            "Generated english word: {}, flag: {:?}",
            word,
            vec_to_letter(&output)
        );
    }

    // Rune word -> RUNE_FLAG? ... Yes!
    for _ in 0..10 {
        gen_word(&mut rng, gen_rune_letter, &mut word);

        word_to_data(&word, &mut input, &mut tmpbuf_1);

        // Outputs result.
        ai.calc(&input, &mut output, &mut tmpbuf_2);

        assert_eq!(vec_to_letter(&output), Some(RUNE_FLAG));

        println!(
            "Generated rune word: {}, flag: {:?}",
            word,
            vec_to_letter(&output)
        );
    }
}
