use crate::chobit_map::*;

#[test]
fn chobit_map_test() {
    const SIZE: usize = 512;

    let mut map = ChobitMap::<i32>::new(SIZE + 10);
    assert_eq!(map.table_size(), SIZE);

    const MAX: u64 = 10000;

    for i in 0..MAX {
        assert!(map.add(i, i as i32).is_some());
    }

    for i in 0..MAX {
        assert!(map.add(i, i as i32).is_none());
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
        assert!(map.remove(i).is_none());
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
    assert!(map.add(key_1, value_1).is_some());

    let key_2: u64 = 0x222;
    let value_2: i32 = 200;
    assert!(map.add(key_2, value_2).is_some());

    let key_3: u64 = 0x311;
    let value_3: i32 = 300;
    assert!(map.add(key_3, value_3).is_some());

    let key_4: u64 = 0x444;
    let value_4: i32 = 400;
    assert!(map.add(key_4, value_4).is_some());

    let mut iter = map.iter();

    assert_eq!(iter.next().unwrap(), &value_1);
    assert_eq!(iter.next().unwrap(), &value_3);
    assert_eq!(iter.next().unwrap(), &value_2);
    assert_eq!(iter.next().unwrap(), &value_4);
    assert!(iter.next().is_none());
    assert!(iter.next().is_none());
}
