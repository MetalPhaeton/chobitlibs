extern crate chobitlibs;

use std::prelude::rust_2021::*;

use chobitlibs::chobit_ani_value::*;

fn gen_chobit_ani_value() -> ChobitAniValue {
    ChobitAniValue::new(5, &[3usize, 5, 7, 0], 10.0).unwrap()
}

#[test]
fn new_test_1() {
    let value = gen_chobit_ani_value();

    assert_eq!(value.columns(), 5);
    assert_eq!(value.rows(), 4);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 0);
    assert_eq!(value.saved_time(), 0.0);
    assert_eq!(value.seconds_per_frame(), 10.0f32.recip());
    assert_eq!(value.frames_per_second(), 10.0);
    assert_eq!(value.uv_frame_width(), (value.columns() as f32).recip());
    assert_eq!(value.uv_frame_height(), (value.rows() as f32).recip());
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (0.0f32, 0.0f32, value.uv_frame_width(), value.uv_frame_height())
    );
}

#[test]
fn new_test_2() {
    assert_eq!(
        ChobitAniValue::new(0, &[3usize, 5, 7, 0], 10.0),
        Err(ChobitAniValueError::from(GenerationError::InvalidColumns))
    );

    assert_eq!(
        ChobitAniValue::new(5, &[], 10.0),
        Err(ChobitAniValueError::from(GenerationError::InvalidRows))
    );

    assert_eq!(
        ChobitAniValue::new(5, &[3usize, 5, 7, 0], 0.0),
        Err(ChobitAniValueError::from(GenerationError::InvalidFramesPerSecond))
    );
}

#[test]
fn next_frame_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), Some(2));

    value.next_frame();
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    value.next_frame();
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    value.next_frame();
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );

    value.next_frame();
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
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

    assert!(value.set_row(value.current_row() + 1).is_ok());

    assert_eq!(value.last_frame(), Some(4));

    value.next_frame();
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            value.uv_frame_height(),
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.next_frame();
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.next_frame();
    assert_eq!(value.current_frame(), Some(3));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.next_frame();
    assert_eq!(value.current_frame(), Some(4));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.next_frame();
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0,
            value.uv_frame_height(),
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0
        )
    );

    value.next_frame();
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
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

    assert!(value.set_row(value.current_row() + 2).is_ok());

    assert_eq!(value.last_frame(), Some(4));

    value.next_frame();
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.next_frame();
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.next_frame();
    assert_eq!(value.current_frame(), Some(3));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.next_frame();
    assert_eq!(value.current_frame(), Some(4));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.next_frame();
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width(),
            value.uv_frame_height() * 3.0
        )
    );

    value.next_frame();
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 3.0
        )
    );
}

#[test]
fn next_frame_test_4() {
    let mut value = gen_chobit_ani_value();

    assert!(value.set_row(value.current_row() + 3).is_ok());

    assert_eq!(value.last_frame(), None);

    value.next_frame();
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

    value.next_frame();
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

}

#[test]
fn prev_frame_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), Some(2));

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
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

    assert!(value.set_row(value.current_row() + 1).is_ok());

    assert_eq!(value.last_frame(), Some(4));

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(4));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(3));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0
        )
    );


    value.prev_frame();
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            value.uv_frame_height(),
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0,
            value.uv_frame_height(),
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0
        )
    );

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(4));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 2.0
        )
    );
}

#[test]
fn prev_frame_test_3() {
    let mut value = gen_chobit_ani_value();

    assert!(value.set_row(value.current_row() + 2).is_ok());

    assert_eq!(value.last_frame(), Some(4));

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(4));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(3));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width(),
            value.uv_frame_height() * 3.0
        )
    );

    value.prev_frame();
    assert_eq!(value.current_frame(), Some(4));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 3.0
        )
    );
}

#[test]
fn prev_frame_test_4() {
    let mut value = gen_chobit_ani_value();

    assert!(value.set_row(value.current_row() + 3).is_ok());

    assert_eq!(value.last_frame(), None);

    value.prev_frame();
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

    value.prev_frame();
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

}

#[test]
fn elapse_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), Some(2));

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );

    value.elapse(0.05);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );

    value.elapse(0.15);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    value.elapse(0.3);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
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

    assert!(value.set_row(value.current_row() + 1).is_ok());

    assert_eq!(value.last_frame(), Some(4));

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            value.uv_frame_height(),
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(3));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(4));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0,
            value.uv_frame_height(),
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0
        )
    );

    value.elapse(0.3);
    assert_eq!(value.current_frame(), Some(3));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.elapse(0.6);
    assert_eq!(value.current_frame(), Some(4));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 2.0
        )
    );
}

#[test]
fn elapse_test_3() {
    let mut value = gen_chobit_ani_value();

    assert!(value.set_row(value.current_row() + 2).is_ok());

    assert_eq!(value.last_frame(), Some(4));

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(3));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(4));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width(),
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse(0.05);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width(),
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse(0.15);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse(0.6);
    assert_eq!(value.current_frame(), Some(3));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 3.0
        )
    );
}

#[test]
fn elapse_test_4() {
    let mut value = gen_chobit_ani_value();

    assert!(value.set_row(value.current_row() + 3).is_ok());

    assert_eq!(value.last_frame(), None);

    value.elapse(0.1);
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

    value.elapse(0.1);
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

}

#[test]
fn elapse_inv_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), Some(2));

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );

    value.elapse_inv(0.05);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );

    value.elapse_inv(0.15);
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    value.elapse_inv(0.3);
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
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

    assert!(value.set_row(value.current_row() + 1).is_ok());

    assert_eq!(value.last_frame(), Some(4));

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(4));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(3));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            value.uv_frame_height(),
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0,
            value.uv_frame_height(),
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0
        )
    );

    value.elapse_inv(0.3);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height(),
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0
        )
    );

    value.elapse_inv(0.6);
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 1);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            value.uv_frame_height(),
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0
        )
    );
}

#[test]
fn elapse_inv_test_3() {
    let mut value = gen_chobit_ani_value();

    assert!(value.set_row(value.current_row() + 2).is_ok());

    assert_eq!(value.last_frame(), Some(4));

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(4));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 5.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(3));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width(),
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse_inv(0.05);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width(),
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse_inv(0.15);
    assert_eq!(value.current_frame(), Some(3));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 4.0,
            value.uv_frame_height() * 3.0
        )
    );

    value.elapse_inv(0.6);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 2);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            value.uv_frame_height() * 2.0,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height() * 3.0
        )
    );
}

#[test]
fn elapse_inv_test_4() {
    let mut value = gen_chobit_ani_value();

    assert!(value.set_row(value.current_row() + 3).is_ok());

    assert_eq!(value.last_frame(), None);

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

}

#[test]
fn elapse_other_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), Some(2));

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    value.set_frames_per_second(5.0);

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    value.elapse(0.1);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    value.elapse(0.3);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );
}

#[test]
fn elapse_other_test_2() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), Some(2));

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    value.set_frames_per_second(5.0);

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(2));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width() * 2.0,
            0.0f32,
            value.uv_frame_width() * 3.0,
            value.uv_frame_height()
        )
    );

    value.elapse_inv(0.1);
    assert_eq!(value.current_frame(), Some(1));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            value.uv_frame_width(),
            0.0f32,
            value.uv_frame_width() * 2.0,
            value.uv_frame_height()
        )
    );

    value.elapse_inv(0.3);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 0);
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (
            0.0f32,
            0.0f32,
            value.uv_frame_width(),
            value.uv_frame_height()
        )
    );
}

#[test]
fn other_test() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.current_row(), 0);
    assert!(value.set_frame(0).is_ok());
    assert!(value.set_frame(1).is_ok());
    assert!(value.set_frame(2).is_ok());
    assert_eq!(value.set_frame(3), Err(ChobitAniValueError::InvalidFrame));
    assert_eq!(value.set_frame(4), Err(ChobitAniValueError::InvalidFrame));

    assert!(value.set_row(value.current_row() + 1).is_ok());
    assert_eq!(value.current_row(), 1);
    assert!(value.set_frame(0).is_ok());
    assert!(value.set_frame(1).is_ok());
    assert!(value.set_frame(2).is_ok());
    assert!(value.set_frame(3).is_ok());
    assert!(value.set_frame(4).is_ok());
    assert_eq!(value.set_frame(5), Err(ChobitAniValueError::InvalidFrame));
    assert_eq!(value.set_frame(6), Err(ChobitAniValueError::InvalidFrame));

    assert!(value.set_row(value.current_row() + 1).is_ok());
    assert_eq!(value.current_row(), 2);
    assert!(value.set_frame(0).is_ok());
    assert!(value.set_frame(1).is_ok());
    assert!(value.set_frame(2).is_ok());
    assert!(value.set_frame(3).is_ok());
    assert!(value.set_frame(4).is_ok());
    assert_eq!(value.set_frame(5), Err(ChobitAniValueError::InvalidFrame));
    assert_eq!(value.set_frame(6), Err(ChobitAniValueError::InvalidFrame));

    assert!(value.set_row(value.current_row() + 1).is_ok());
    assert_eq!(value.current_row(), 3);
    assert_eq!(
        value.set_frame(0),
        Err(ChobitAniValueError::NoFrameInCurrentRow)
    );
    assert_eq!(
        value.set_frame(1),
        Err(ChobitAniValueError::NoFrameInCurrentRow)
    );

    assert_eq!(
        value.set_row(value.current_row() + 1),
        Err(ChobitAniValueError::InvalidRow)
    );
    assert_eq!(value.current_row(), 3);
}
