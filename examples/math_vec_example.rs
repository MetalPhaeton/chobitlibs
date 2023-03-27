extern crate chobitlibs;

use chobitlibs::chobit_ai::MathVec;

fn main() {
    const DIM: usize = 3;

    let mut v_1 = MathVec::<DIM>::new();

    v_1[0] = 1.0;
    v_1[1] = 2.0;
    v_1[2] = 3.0;

    let mut v_2 = MathVec::<DIM>::new();

    v_2[0] = 4.0;
    v_2[1] = 5.0;
    v_2[2] = 6.0;

    let x: f32 = 10.0;

    // Add
    let mut v_3 = &v_1 + &v_2;
    assert_eq!(v_3[0], 1.0 + 4.0);
    assert_eq!(v_3[1], 2.0 + 5.0);
    assert_eq!(v_3[2], 3.0 + 6.0);

    // Add assign
    v_3 += &v_1;
    assert_eq!(v_3[0], 1.0 + 4.0 + 1.0);
    assert_eq!(v_3[1], 2.0 + 5.0 + 2.0);
    assert_eq!(v_3[2], 3.0 + 6.0 + 3.0);

    // Sub
    let mut v_3 = &v_1 - &v_2;
    assert_eq!(v_3[0], 1.0 - 4.0);
    assert_eq!(v_3[1], 2.0 - 5.0);
    assert_eq!(v_3[2], 3.0 - 6.0);

    // Sub assign
    v_3 -= &v_1;
    assert_eq!(v_3[0], 1.0 - 4.0 - 1.0);
    assert_eq!(v_3[1], 2.0 - 5.0 - 2.0);
    assert_eq!(v_3[2], 3.0 - 6.0 - 3.0);

    // Multiply.
    let mut v_3 = &v_1 * x;
    assert_eq!(v_3[0], 1.0 * 10.0);
    assert_eq!(v_3[1], 2.0 * 10.0);
    assert_eq!(v_3[2], 3.0 * 10.0);

    // Multiply assign.
    v_3 *= x;
    assert_eq!(v_3[0], 1.0 * 10.0 * 10.0);
    assert_eq!(v_3[1], 2.0 * 10.0 * 10.0);
    assert_eq!(v_3[2], 3.0 * 10.0 * 10.0);

    // Division.
    let mut v_3 = &v_1 / x;
    assert_eq!(v_3[0], 1.0 / 10.0);
    assert_eq!(v_3[1], 2.0 / 10.0);
    assert_eq!(v_3[2], 3.0 / 10.0);

    // Division assign.
    v_3 /= x;
    assert_eq!(v_3[0], 1.0 / 10.0 / 10.0);
    assert_eq!(v_3[1], 2.0 / 10.0 / 10.0);
    assert_eq!(v_3[2], 3.0 / 10.0 / 10.0);

    // Division remainder.
    let mut v_3 = &v_1 % x;
    assert_eq!(v_3[0], 1.0 % 10.0);
    assert_eq!(v_3[1], 2.0 % 10.0);
    assert_eq!(v_3[2], 3.0 % 10.0);

    // Division remainder assign.
    v_3 %= x;
    assert_eq!(v_3[0], 1.0 % 10.0 % 10.0);
    assert_eq!(v_3[1], 2.0 % 10.0 % 10.0);
    assert_eq!(v_3[2], 3.0 % 10.0 % 10.0);

    // Inner product.
    let y = &v_1 * &v_2;
    assert_eq!(y, (1.0 * 4.0) + (2.0 * 5.0) + (3.0 * 6.0));
}
