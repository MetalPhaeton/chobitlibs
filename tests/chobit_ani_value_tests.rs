extern crate chobitlibs;

use std::prelude::rust_2021::*;

use chobitlibs::chobit_ani_value::*;

#[test]
fn display_error_test() {
    println!(
        "{}",
        ChobitAniValueError::InvalidColumns,
    );

    println!(
        "{}",
        ChobitAniValueError::InvalidFramesOfEachRow,
    );

    println!(
        "{}",
        ChobitAniValueError::InvalidRows,
    );

    println!(
        "{}",
        ChobitAniValueError::InvalidFramesPerSecond,
    );
}

fn gen_chobit_ani_value() -> ChobitAniValue {
    assert_eq!(
        ChobitAniValue::new(0, &[3usize, 4, 5], 10.0),
        Err(ChobitAniValueError::InvalidColumns)
    );

    assert_eq!(
        ChobitAniValue::new(5, &[], 10.0),
        Err(ChobitAniValueError::InvalidRows)
    );

    assert_eq!(
        ChobitAniValue::new(5, &[0usize, 4, 5], 10.0),
        Err(ChobitAniValueError::InvalidFramesOfEachRow)
    );

    assert_eq!(
        ChobitAniValue::new(5, &[7usize, 4, 5], 10.0),
        Err(ChobitAniValueError::InvalidFramesOfEachRow)
    );

    assert_eq!(
        ChobitAniValue::new(5, &[3usize, 4, 5], 0.0),
        Err(ChobitAniValueError::InvalidFramesPerSecond)
    );

    ChobitAniValue::new(5, &[3usize, 4, 5], 10.0).unwrap()
}

#[test]
fn new_test_1() {
    let value = gen_chobit_ani_value();

    assert_eq!(value.columns(), 5);
    assert_eq!(value.rows(), 3);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 0);
    assert_eq!(value.saved_time(), 0.0);
    assert_eq!(value.seconds_per_frame(), 10.0f32.recip());
    assert_eq!(value.frames_per_second(), 10.0);
    assert_eq!(value.uv_frame_width(), (value.columns() as f32).recip());
    assert_eq!(value.uv_frame_height(), (value.rows() as f32).recip());
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (0.0f32, 0.0f32, value.uv_frame_width(), value.uv_frame_height())
    );
}

#[test]
fn next_frame_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), 2);

    assert_eq!(value.next_frame(), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.next_frame(), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.next_frame(), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );

    assert_eq!(value.next_frame(), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );
}

#[test]
fn next_frame_test_2() {
    let mut value = gen_chobit_ani_value();

    value.set_row(value.current_row() + 1);

    assert_eq!(value.last_frame(), 3);

    assert_eq!(value.next_frame(), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            value.uv_frame_height(),
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.next_frame(), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.next_frame(), 3);
    assert_eq!(value.current_frame(), 3);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.next_frame(), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0,
            value.uv_frame_height(),
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.next_frame(), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            value.uv_frame_height(),
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0
        )
    );
}

#[test]
fn next_frame_test_3() {
    let mut value = gen_chobit_ani_value();

    value.set_row(value.current_row() + 2);

    assert_eq!(value.last_frame(), 4);

    assert_eq!(value.next_frame(), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.next_frame(), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.next_frame(), 3);
    assert_eq!(value.current_frame(), 3);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.next_frame(), 4);
    assert_eq!(value.current_frame(), 4);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.next_frame(), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width(),
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.next_frame(), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 3.0
        )
    );
}

#[test]
fn prev_frame_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), 2);

    assert_eq!(value.prev_frame(), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.prev_frame(), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.prev_frame(), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );

    assert_eq!(value.prev_frame(), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );
}

#[test]
fn prev_frame_test_2() {
    let mut value = gen_chobit_ani_value();

    value.set_row(value.current_row() + 1);

    assert_eq!(value.last_frame(), 3);

    assert_eq!(value.prev_frame(), 3);
    assert_eq!(value.current_frame(), 3);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.prev_frame(), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0
        )
    );


    assert_eq!(value.prev_frame(), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            value.uv_frame_height(),
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.prev_frame(), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0,
            value.uv_frame_height(),
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.prev_frame(), 3);
    assert_eq!(value.current_frame(), 3);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0
        )
    );
}

#[test]
fn prev_frame_test_3() {
    let mut value = gen_chobit_ani_value();

    value.set_row(value.current_row() + 2);

    assert_eq!(value.last_frame(), 4);

    assert_eq!(value.prev_frame(), 4);
    assert_eq!(value.current_frame(), 4);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.prev_frame(), 3);
    assert_eq!(value.current_frame(), 3);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.prev_frame(), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.prev_frame(), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.prev_frame(), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width(),
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.prev_frame(), 4);
    assert_eq!(value.current_frame(), 4);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 3.0
        )
    );
}

#[test]
fn elapse_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), 2);

    assert_eq!(value.elapse(0.1), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse(0.1), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse(0.1), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse(0.05), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse(0.15), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse(0.3), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );
}

#[test]
fn elapse_test_2() {
    let mut value = gen_chobit_ani_value();

    value.set_row(value.current_row() + 1);

    assert_eq!(value.last_frame(), 3);

    assert_eq!(value.elapse(0.1), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            value.uv_frame_height(),
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.elapse(0.1), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.elapse(0.1), 3);
    assert_eq!(value.current_frame(), 3);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.elapse(0.1), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0,
            value.uv_frame_height(),
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.elapse(0.3), 3);
    assert_eq!(value.current_frame(), 3);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0
        )
    );
}

#[test]
fn elapse_test_3() {
    let mut value = gen_chobit_ani_value();

    value.set_row(value.current_row() + 2);

    assert_eq!(value.last_frame(), 4);

    assert_eq!(value.elapse(0.1), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse(0.1), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse(0.1), 3);
    assert_eq!(value.current_frame(), 3);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse(0.1), 4);
    assert_eq!(value.current_frame(), 4);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse(0.1), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width(),
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse(0.05), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width(),
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse(0.15), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse(0.6), 3);
    assert_eq!(value.current_frame(), 3);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 3.0
        )
    );
}

#[test]
fn elapse_inv_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), 2);

    assert_eq!(value.elapse_inv(0.1), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse_inv(0.1), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse_inv(0.1), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse_inv(0.05), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse_inv(0.15), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse_inv(0.3), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );
}

#[test]
fn elapse_inv_test_2() {
    let mut value = gen_chobit_ani_value();

    value.set_row(value.current_row() + 1);

    assert_eq!(value.last_frame(), 3);

    assert_eq!(value.elapse_inv(0.1), 3);
    assert_eq!(value.current_frame(), 3);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.elapse_inv(0.1), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.elapse_inv(0.1), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            value.uv_frame_height(),
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.elapse_inv(0.1), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0,
            value.uv_frame_height(),
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0
        )
    );

    assert_eq!(value.elapse_inv(0.1), 3);
    assert_eq!(value.current_frame(), 3);
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0
        )
    );
}

#[test]
fn elapse_inv_test_3() {
    let mut value = gen_chobit_ani_value();

    value.set_row(value.current_row() + 2);

    assert_eq!(value.last_frame(), 4);

    assert_eq!(value.elapse_inv(0.1), 4);
    assert_eq!(value.current_frame(), 4);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse_inv(0.1), 3);
    assert_eq!(value.current_frame(), 3);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse_inv(0.1), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse_inv(0.1), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse_inv(0.1), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width(),
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse_inv(0.05), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width(),
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse_inv(0.15), 3);
    assert_eq!(value.current_frame(), 3);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 3.0
        )
    );

    assert_eq!(value.elapse_inv(0.6), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 3.0
        )
    );
}

#[test]
fn elapse_other_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), 2);

    assert_eq!(value.elapse(0.1), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    value.set_frames_per_second(5.0);

    assert_eq!(value.elapse(0.1), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse(0.1), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse(0.3), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );
}

#[test]
fn elapse_other_test_2() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), 2);

    assert_eq!(value.elapse_inv(0.1), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    value.set_frames_per_second(5.0);

    assert_eq!(value.elapse_inv(0.1), 2);
    assert_eq!(value.current_frame(), 2);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse_inv(0.1), 1);
    assert_eq!(value.current_frame(), 1);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    assert_eq!(value.elapse_inv(0.3), 0);
    assert_eq!(value.current_frame(), 0);
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );
}
