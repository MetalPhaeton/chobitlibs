extern crate chobitlibs;

use std::prelude::rust_2021::*;

use chobitlibs::chobit_ani_value::*;

fn gen_chobit_ani_value() -> ChobitAniValue {
    ChobitAniValue::new(5, &[3usize, 5, 7, 0], 10, 4).unwrap()
}

#[test]
fn new_test_1() {
    let value = gen_chobit_ani_value();

    assert_eq!(value.columns(), 5);
    assert_eq!(value.rows(), 4);
    assert_eq!(value.current_frame(), Some(0));
    assert_eq!(value.current_row(), 0);
    assert_eq!(value.ticks_per_second(), 10);
    assert_eq!(value.ticks_per_frame(), 4);
    assert_eq!(value.uv_frame_width(), (value.columns() as f32).recip());
    assert_eq!(value.uv_frame_height(), (value.rows() as f32).recip());
    assert_eq!(
        *value.uv_frame_left_top_right_bottom().unwrap(),
        (0.0f32, 0.0f32, value.uv_frame_width(), value.uv_frame_height())
    );
    assert_eq!(value.ticks_to_seconds(20), 2.0);
    assert_eq!(value.seconds_to_ticks(3.0), value.ticks_per_second() * 3);
    assert_eq!(value.fps_to_tpf(2.0), 5);
    assert_eq!(value.tpf_to_fps(5), 2.0);
}

#[test]
fn new_test_2() {
    assert_eq!(
        ChobitAniValue::new(0, &[3usize, 5, 7, 0], 10, 4),
        Err(ChobitAniValueError::from(GenerationError::InvalidColumns))
    );

    assert_eq!(
        ChobitAniValue::new(5, &[], 10, 4),
        Err(ChobitAniValueError::from(GenerationError::InvalidRows))
    );

    assert_eq!(
        ChobitAniValue::new(5, &[3usize, 5, 7, 0], 0, 4),
        Err(ChobitAniValueError::from(GenerationError::InvalidTicksPerSecond))
    );

    assert_eq!(
        ChobitAniValue::new(5, &[3usize, 5, 7, 0], 10, 0),
        Err(ChobitAniValueError::from(GenerationError::InvalidTicksPerFrame))
    );
}

#[test]
fn next_frame_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), Some(2));

    assert!(!value.next_frame());
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

    assert!(!value.next_frame());
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

    assert!(value.next_frame());
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

    assert!(!value.next_frame());
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

    assert!(!value.next_frame());
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

    assert!(!value.next_frame());
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

    assert!(!value.next_frame());
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

    assert!(!value.next_frame());
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

    assert!(value.next_frame());
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

    assert!(!value.next_frame());
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

    assert!(!value.next_frame());
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

    assert!(!value.next_frame());
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

    assert!(!value.next_frame());
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

    assert!(!value.next_frame());
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

    assert!(value.next_frame());
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

    assert!(!value.next_frame());
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

    assert!(!value.next_frame());
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

    assert!(!value.next_frame());
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

}

#[test]
fn prev_frame_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), Some(2));

    assert!(value.prev_frame());
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

    assert!(!value.prev_frame());
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

    assert!(!value.prev_frame());
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

    assert!(value.prev_frame());
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

    assert!(value.prev_frame());
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

    assert!(!value.prev_frame());
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

    assert!(!value.prev_frame());
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


    assert!(!value.prev_frame());
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

    assert!(!value.prev_frame());
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

    assert!(value.prev_frame());
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

    assert!(value.prev_frame());
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

    assert!(!value.prev_frame());
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

    assert!(!value.prev_frame());
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

    assert!(!value.prev_frame());
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

    assert!(!value.prev_frame());
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

    assert!(value.prev_frame());
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

    assert!(!value.prev_frame());
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

    assert!(!value.prev_frame());
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

}

#[test]
fn elapse_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), Some(2));

    assert!(!value.elapse_ticks(4));
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

    assert!(!value.elapse_ticks(4));
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

    assert!(value.elapse_ticks(4));
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

    assert!(!value.elapse_ticks(2));
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

    assert!(!value.elapse_ticks(6));
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

    assert!(value.elapse_ticks(12));
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

    assert!(!value.elapse_ticks(4));
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

    assert!(!value.elapse_ticks(4));
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

    assert!(!value.elapse_ticks(4));
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

    assert!(!value.elapse_ticks(4));
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

    assert!(value.elapse_ticks(4));
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

    assert!(!value.elapse_ticks(12));
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

    assert!(value.elapse_ticks(24));
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

    assert!(!value.elapse_ticks(4));
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

    assert!(!value.elapse_ticks(4));
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

    assert!(!value.elapse_ticks(4));
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

    assert!(!value.elapse_ticks(4));
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

    assert!(value.elapse_ticks(4));
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

    assert!(!value.elapse_ticks(2));
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

    assert!(!value.elapse_ticks(6));
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

    assert!(value.elapse_ticks(24));
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

    assert!(!value.elapse_ticks(4));
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

    assert!(!value.elapse_ticks(10));
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

}

#[test]
fn elapse_test_5() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), Some(2));

    assert!(!value.elapse_seconds(0.4));
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

    assert!(!value.elapse_seconds(0.4));
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

    assert!(value.elapse_seconds(0.4));
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

    assert!(!value.elapse_seconds(0.2));
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

    assert!(!value.elapse_seconds(0.6));
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

    assert!(value.elapse_seconds(1.2));
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
fn elapse_inv_test_1() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), Some(2));

    assert!(value.elapse_ticks_inv(4));
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

    assert!(!value.elapse_ticks_inv(4));
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

    assert!(!value.elapse_ticks_inv(4));
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

    assert!(!value.elapse_ticks_inv(2));
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

    assert!(value.elapse_ticks_inv(6));
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

    assert!(value.elapse_ticks_inv(12));
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

    assert!(value.elapse_ticks_inv(4));
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

    assert!(!value.elapse_ticks_inv(4));
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

    assert!(!value.elapse_ticks_inv(4));
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

    assert!(!value.elapse_ticks_inv(4));
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

    assert!(!value.elapse_ticks_inv(4));
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

    assert!(value.elapse_ticks_inv(12));
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

    assert!(value.elapse_ticks_inv(24));
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

    assert!(value.elapse_ticks_inv(4));
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

    assert!(!value.elapse_ticks_inv(4));
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

    assert!(!value.elapse_ticks_inv(4));
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

    assert!(!value.elapse_ticks_inv(4));
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

    assert!(!value.elapse_ticks_inv(4));
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

    assert!(!value.elapse_ticks_inv(2));
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

    assert!(value.elapse_ticks_inv(6));
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

    assert!(value.elapse_ticks_inv(24));
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

    assert!(!value.elapse_ticks_inv(4));
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

    assert!(!value.elapse_ticks_inv(10));
    assert_eq!(value.current_frame(), None);
    assert_eq!(value.current_row(), 3);
    assert_eq!(value.uv_frame_left_top_right_bottom(), None);

}

#[test]
fn elapse_inv_test_5() {
    let mut value = gen_chobit_ani_value();

    assert_eq!(value.last_frame(), Some(2));

    assert!(value.elapse_seconds_inv(0.4));
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

    assert!(!value.elapse_seconds_inv(0.4));
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

    assert!(!value.elapse_seconds_inv(0.4));
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

    assert!(!value.elapse_seconds_inv(0.2));
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

    assert!(value.elapse_seconds_inv(0.6));
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

    assert!(value.elapse_seconds_inv(1.2));
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
