extern crate chobitlibs;

use chobitlibs::chobit_ai::*;
use chobitlibs::chobit_rand::*;

use std::mem::size_of;

//==================//
// ChobitAI Example //
//==================//

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
    let mut ret = String::with_capacity(WORD_SIZE);

    ret.push(gen_letter(rng));
    ret.push(gen_letter(rng));
    ret.push(gen_letter(rng));
    ret.push(gen_letter(rng));

    ret
}

fn word_to_data(word: &String) -> MathVec<IN> {
    let mut ret = MathVec::<IN>::new();

    let mut iter = word.chars();

    if let Some(letter) = iter.next() {
        let vec = letter_to_vec(letter);
        ret[..NUM_CHAR_BITS].copy_from_slice(&vec);
    }

    if let Some(letter) = iter.next() {
        let vec = letter_to_vec(letter);
        ret[NUM_CHAR_BITS..(NUM_CHAR_BITS * 2)].copy_from_slice(&vec);
    }

    if let Some(letter) = iter.next() {
        let vec = letter_to_vec(letter);
        ret[(NUM_CHAR_BITS * 2)..(NUM_CHAR_BITS * 3)].copy_from_slice(&vec);
    }

    if let Some(letter) = iter.next() {
        let vec = letter_to_vec(letter);
        ret[(NUM_CHAR_BITS * 3)..(NUM_CHAR_BITS * 4)].copy_from_slice(&vec);
    }

    ret
}

const JAPANESE_FLAG: char = '日';
const ENGLISH_FLAG: char = 'E';
const RUNE_FLAG: char = 'ᚱ';

fn main() {
    //=======//
    // Ready //
    //=======//
    // Generates random number generator.
    let mut rng = ChobitRand::new("This is ChobitAI Example".as_bytes());

    // Generates AI.
    let mut ai = ChobitAI::<OUT, MIDDLE, IN>::new(Activation::SoftSign);

    // Initializes weights with random number.
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
    let mut output = MathVec::<OUT>::new();

    // Temporary buffer for AI to work.
    let mut tmpbuf = MathVec::<MIDDLE>::new();

    // japanese_word -> JAPANESE_FLAG? ... No yet!
    for _ in 0..10 {
        let japanese_word = gen_word(&mut rng, gen_japanese_letter);

        let data = word_to_data(&japanese_word);

        ai.calc(&data, &mut output, &mut tmpbuf);

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

        ai.calc(&data, &mut output, &mut tmpbuf);

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

        ai.calc(&data, &mut output, &mut tmpbuf);

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
    let mut ml_ai = ChobitMLAI::<OUT, MIDDLE, IN>::new(ai);

    const EPOCH: usize = 1000;
    const BATCH_SIZE: usize = 20;
    const RATE: f32 = 0.01;

    // Flags to vectors.
    let japanese_flag = letter_to_vec(JAPANESE_FLAG);
    let english_flag = letter_to_vec(ENGLISH_FLAG);
    let rune_flag = letter_to_vec(RUNE_FLAG);

    // Machine learning.
    for _ in 0..EPOCH {
        for _ in 0..BATCH_SIZE {
            // Studies japanese word.
            let japanese_word = gen_word(&mut rng, gen_japanese_letter);

            let data = word_to_data(&japanese_word);

            ml_ai.study(&data, &japanese_flag);

            // Studies english word.
            let english_word = gen_word(&mut rng, gen_english_letter);

            let data = word_to_data(&english_word);

            ml_ai.study(&data, &english_flag);

            // Studies rune word.
            let rune_word = gen_word(&mut rng, gen_rune_letter);

            let data = word_to_data(&rune_word);

            ml_ai.study(&data, &rune_flag);
        }

        // Updates weights.
        ml_ai.update(RATE);
    }

    //========================//
    // After machine learning //
    //========================//
    println!("//========================//");
    println!("// After machine learning //");
    println!("//========================//");

    // Unwraps AI from ChobitMLAI.
    let ai = ml_ai.drop();

    // japanese_word -> JAPANESE_FLAG? ... Yes!
    for _ in 0..10 {
        let japanese_word = gen_word(&mut rng, gen_japanese_letter);

        let data = word_to_data(&japanese_word);

        ai.calc(&data, &mut output, &mut tmpbuf);

        assert_eq!(vec_to_letter(&output), Some(JAPANESE_FLAG));

        println!(
            "Generated japanese word: {}, flag: {:?}",
            japanese_word,
            vec_to_letter(&output)
        );
    }

    // english_word -> ENGLISH_FLAG? ... Yes!
    for _ in 0..10 {
        let english_word = gen_word(&mut rng, gen_english_letter);

        let data = word_to_data(&english_word);

        ai.calc(&data, &mut output, &mut tmpbuf);

        assert_eq!(vec_to_letter(&output), Some(ENGLISH_FLAG));

        println!(
            "Generated english word: {}, flag: {:?}",
            english_word,
            vec_to_letter(&output)
        );
    }

    // rune_word -> RUNE_FLAG? ... Yes!
    for _ in 0..10 {
        let rune_word = gen_word(&mut rng, gen_rune_letter);

        let data = word_to_data(&rune_word);

        ai.calc(&data, &mut output, &mut tmpbuf);

        assert_eq!(vec_to_letter(&output), Some(RUNE_FLAG));

        println!(
            "Generated rune word: {}, flag: {:?}",
            rune_word,
            vec_to_letter(&output)
        );
    }
}
