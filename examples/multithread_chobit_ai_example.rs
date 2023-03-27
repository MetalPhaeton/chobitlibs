extern crate chobitlibs;

use std::{
    thread,
    sync::{
        mpsc,
        Arc,
        Mutex
    }
};

use chobitlibs::chobit_ai::{
    MathVec,
    Activation,
    ChobitAI,
    ChobitMLAI,
    MLAICache
};

use chobitlibs::chobit_rand::ChobitRand;

const OUT: usize = 32;
const MIDDLE: usize = 64;
const IN: usize = 32;

const EPOCH: usize = 10000;
const BATCH_SIZE: usize = 30;
const RATE: f32 = 0.01;

// Command from parent to child.
enum P2C {
    StartEpoch,
    Break
}

// Command from child to parent.
enum C2P {
    EndedOneEpoch,
}

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

// Initializes shared gradient.
fn init_grad(
    ai: &ChobitMLAI<OUT, MIDDLE, IN>,
    grads: &Arc<Mutex<(Vec<f32>, Vec<f32>)>>
) {
    let mut lock = grads.lock().unwrap();

    ai.for_each_total_grad(|_| {
        lock.0.push(0.0);
        lock.1.push(0.0);
    });
}

// Loads from shared gradient to AI's gradient.
fn load_grad(
    ai: &mut ChobitMLAI<OUT, MIDDLE, IN>,
    grads: &Arc<Mutex<(Vec<f32>, Vec<f32>)>>
) {
    let lock = grads.lock().unwrap();

    let mut iter = lock.0.iter();

    ai.for_each_total_grad_mut(|grad| {
        if let Some(shared_grad) = iter.next() {
            *grad = *shared_grad;
        }
    });
}

// Adds AI's gradient to shared gradient.
fn save_grad(
    ai: &ChobitMLAI<OUT, MIDDLE, IN>,
    grads: &Arc<Mutex<(Vec<f32>, Vec<f32>)>>
) {
    let mut lock = grads.lock().unwrap();

    let mut iter = lock.1.iter_mut();

    ai.for_each_total_grad(|grad| {
        if let Some(shared_grad) = iter.next() {
            *shared_grad += *grad;
        }
    });
}

// Prepares to load_grad().
fn move_grad(
    grads: &Arc<Mutex<(Vec<f32>, Vec<f32>)>>,
    tmpbuf: &mut Vec<f32>
) {
    let mut lock = grads.lock().unwrap();

    lock.0.clear();
    tmpbuf.clear();

    tmpbuf.extend_from_slice(&lock.1);

    lock.0.extend_from_slice(&tmpbuf);

    lock.1.fill(0.0);
}

fn run_thread(
    rng_seed: &[u8],
    p2c_rx: mpsc::Receiver<P2C>,
    c2p_tx: mpsc::Sender<C2P>,
    ai: ChobitAI<OUT, MIDDLE, IN>,
    grads: Arc<Mutex<(Vec<f32>, Vec<f32>)>>
) -> thread::JoinHandle<()> {
    let rng_seed = rng_seed.to_vec();

    thread::spawn(move || {
        // Wraps AI for machine learning.
        let mut ai = ChobitMLAI::<OUT, MIDDLE, IN>::new(ai);

        let mut rng = ChobitRand::new(&rng_seed);

        let mut cache = MLAICache::<OUT, MIDDLE, IN>::new();
        let mut input = MathVec::<IN>::new();
        let mut output = MathVec::<OUT>::new();

        let mut input_error = MathVec::<IN>::new();
        let mut output_error = MathVec::<OUT>::new();

        // Machine learning.
        for cmd in p2c_rx {  // Waits for command from parent.
            match cmd {
                P2C::StartEpoch => {
                    // To load shared gradient and update itself,
                    // all children and parent become same state.
                    load_grad(&mut ai, &grads);
                    ai.update(RATE);

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

                    // Adds its own gradient to shared gradient.
                    save_grad(&ai, &grads);

                    // Notifies ended one epoch to parent.
                    c2p_tx.send(C2P::EndedOneEpoch).unwrap();
                },

                P2C::Break => {
                    break;
                }
            }
        }
    })
}

fn test_ai(ai: &ChobitAI<OUT, MIDDLE, IN>, rng: &mut ChobitRand) {
    let mut input = MathVec::<IN>::new();
    let mut output = MathVec::<OUT>::new();
    let mut tmpbuf = MathVec::<MIDDLE>::new();

    // Tests Japanese.
    for _ in 0..10 {
        input.load_u32_label(japanese_letter(rng) as u32);

        ai.calc(&input, &mut output, &mut tmpbuf);

        assert_eq!(output.to_u32_label(), JAPANESE_ID as u32);
    }

    // Tests English.
    for _ in 0..10 {
        input.load_u32_label(english_letter(rng) as u32);

        ai.calc(&input, &mut output, &mut tmpbuf);

        assert_eq!(output.to_u32_label(), ENGLISH_ID as u32);
    }
}

fn main() {
    let mut rng = ChobitRand::new(b"ChobitAI Example");

    let mut ai = ChobitAI::<OUT, MIDDLE, IN>::new(Activation::SoftSign);

    // Randomises weights.
    ai.for_each_weight_mut(|weight| {
        *weight = ((rng.next_f64() as f32) * 2.0) - 1.0;
    });

    // Shared gradient.
    let grads = Arc::new(Mutex::new(
        (Vec::<f32>::new(), Vec::<f32>::new())
    ));

    // Channel from parent to child.
    let (p2c_1_tx, p2c_1_rx) = mpsc::channel::<P2C>();
    let (p2c_2_tx, p2c_2_rx) = mpsc::channel::<P2C>();
    let p2c_tx = [p2c_1_tx, p2c_2_tx];

    // Channel from child to parent.
    let (c2p_1_tx, c2p_1_rx) = mpsc::channel::<C2P>();
    let (c2p_2_tx, c2p_2_rx) = mpsc::channel::<C2P>();
    let c2p_rx = [c2p_1_rx, c2p_2_rx];

    let handle_1 =
        run_thread(b"child_1", p2c_1_rx, c2p_1_tx, ai.clone(), grads.clone());

    let handle_2 =
        run_thread(b"child_2", p2c_2_rx, c2p_2_tx, ai.clone(), grads.clone());

    // Wraps AI for machine learning.
    let mut ai = ChobitMLAI::<OUT, MIDDLE, IN>::new(ai);
    let mut tmpbuf = Vec::<f32>::new();

    // Initialize shared gradient.
    init_grad(&ai, &grads);

    // Loop each epoch.
    for _ in 0..EPOCH {
        load_grad(&mut ai, &grads);
        ai.update(RATE);

        p2c_tx.iter().for_each(|tx| {tx.send(P2C::StartEpoch).unwrap();});

        c2p_rx.iter().for_each(|rx| {
            let _ = rx.recv().unwrap();
        });

        move_grad(&grads, &mut tmpbuf);
    }

    p2c_tx.iter().for_each(|tx| {tx.send(P2C::Break).unwrap();});

    // Updates by gradient of last epoch.
    move_grad(&grads, &mut tmpbuf);
    load_grad(&mut ai, &grads);
    ai.update(RATE);

    handle_1.join().unwrap();
    handle_2.join().unwrap();

    let ai = ai.drop();

    test_ai(&ai, &mut rng);
}
