extern crate chobitlibs;
extern crate turbo_json_checker;

use chobitlibs::chobit_map::*;
use turbo_json_checker as tjc;

#[test]
fn display_error_test() {
    assert!(tjc::validate_str(
        &(ChobitMapError::AlreadyExists {key: 100}).to_string()
    ).is_ok());

    assert!(tjc::validate_str(
        &(ChobitMapError::NotFound {key: 200}).to_string()
    ).is_ok());
}

#[test]
fn chobit_map_test() {
    const SIZE: usize = 512;

    let mut map = ChobitMap::<i32>::new(SIZE + 10);
    assert_eq!(map.table_size(), SIZE);

    const MAX: u64 = 10000;

    for i in 0..MAX {
        assert!(map.add(i, i as i32).is_ok());
    }

    for i in 0..MAX {
        assert_eq!(
            map.add(i, i as i32),
            Err(ChobitMapError::AlreadyExists {key: i})
        );
    }

    for i in 0..MAX {
        assert_eq!(*map.get(i).unwrap(), i as i32);
        assert_eq!(*map.get_mut(i).unwrap(), i as i32);
    }

    const ADDITION: i32 = 10;

    for i in 0..MAX {
        *map.get_mut(i).unwrap() = (i as i32) + ADDITION;
    }
    for i in 0..MAX {
        assert_eq!(*map.get(i).unwrap(), (i as i32) + ADDITION);
    }

    for i in 0..MAX {
        assert_eq!(map.remove(i).unwrap(), (i as i32) + ADDITION);
    }

    for i in 0..MAX {
        assert_eq!(
            map.remove(i),
            Err(ChobitMapError::NotFound {key: i})
        );
        assert!(map.get(i).is_none());
        assert!(map.get_mut(i).is_none());
    }
}

#[test]
fn iter_test() {
    const SIZE: usize = 0x100;

    let mut map = ChobitMap::<i32>::new(SIZE);

    let key_1: u64 = 0x111;
    let value_1: i32 = 100;
    assert!(map.add(key_1, value_1).is_ok());

    let key_2: u64 = 0x222;
    let value_2: i32 = 200;
    assert!(map.add(key_2, value_2).is_ok());

    let key_3: u64 = 0x311;
    let value_3: i32 = 300;
    assert!(map.add(key_3, value_3).is_ok());

    let key_4: u64 = 0x444;
    let value_4: i32 = 400;
    assert!(map.add(key_4, value_4).is_ok());

    let mut iter = map.iter();

    assert_eq!(iter.next().unwrap(), (key_1, &value_1));
    assert_eq!(iter.next().unwrap(), (key_3, &value_3));
    assert_eq!(iter.next().unwrap(), (key_2, &value_2));
    assert_eq!(iter.next().unwrap(), (key_4, &value_4));
    assert!(iter.next().is_none());
    assert!(iter.next().is_none());
}
