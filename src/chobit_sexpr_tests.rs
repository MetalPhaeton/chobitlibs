use std::prelude::rust_2021::*;

use crate::chobit_sexpr::*;
use core::mem::size_of;

fn gen_test_data() -> Vec<u8> {
    const SIZE: usize = 1024;

    let mut ret = Vec::<u8>::with_capacity(SIZE);

    for i in 0..SIZE {
        ret.push(i as u8);
    }

    ret
}

#[test]
fn chobit_sexpr_test_1() {
    let data = gen_test_data();

    let buf = ChobitSexprBuf::new().push_atom(data.as_slice());
    let header = buf.as_sexpr().header().unwrap();

    assert!(header.is_atom());
    assert_eq!(header.size(), data.len());
    assert_eq!(buf.as_sexpr().atom().unwrap(), data.as_slice());
}

#[test]
fn chobit_sexpr_test_2() {
    const INDEX_1: usize = 10;
    const INDEX_2: usize = 30;

    let data = gen_test_data();

    let buf = ChobitSexprBuf::new()
        .build_cons()
        .push_car(
            ChobitSexprBuf::new().push_atom(&data[..INDEX_1]).as_sexpr()
        ).push_cdr(
            ChobitSexprBuf::new().push_atom(&data[INDEX_1..INDEX_2]).as_sexpr()
        );

    let header = buf.as_sexpr().header().unwrap();
    assert!(header.is_cons());
    assert_eq!(header.size(), INDEX_1 + size_of::<u32>());

    let sexpr = buf.as_sexpr().car().unwrap();
    let header = sexpr.header().unwrap();

    assert!(header.is_atom());
    assert_eq!(header.size(), INDEX_1);
    assert_eq!(sexpr.len(), INDEX_1 + size_of::<u32>());
    assert_eq!(sexpr.atom().unwrap(), &data[..INDEX_1]);

    let sexpr = buf.as_sexpr().cdr().unwrap();
    let header = sexpr.header().unwrap();

    assert!(header.is_atom());
    assert_eq!(header.size(), INDEX_2 - INDEX_1);
    assert_eq!(sexpr.len(), INDEX_2 - INDEX_1 + size_of::<u32>());
    assert_eq!(sexpr.atom().unwrap(), &data[INDEX_1..INDEX_2]);
}

#[test]
fn chobit_sexpr_test_3() {
    const INDEX_1: usize = 10;
    const INDEX_2: usize = 30;

    let data = gen_test_data();

    let buf = ChobitSexprBuf::new()
        .build_list()
        .push_item(
            ChobitSexprBuf::new().push_atom(&data[..INDEX_1]).as_sexpr()
        ).push_item(
            ChobitSexprBuf::new().push_atom(&data[INDEX_1..INDEX_2]).as_sexpr()
        ).finish();

    let header = buf.as_sexpr().header().unwrap();
    assert!(header.is_cons());
    assert_eq!(header.size(), INDEX_1 + size_of::<u32>());

    let sexpr = buf.as_sexpr().car().unwrap();
    let header = sexpr.header().unwrap();
    assert!(header.is_atom());
    assert_eq!(header.size(), INDEX_1);
    assert_eq!(sexpr.len(), INDEX_1 + size_of::<u32>());
    assert_eq!(sexpr.atom().unwrap(), &data[..INDEX_1]);

    let sexpr = buf.as_sexpr().cdr().unwrap();
    let header = sexpr.header().unwrap();
    assert!(header.is_cons());
    assert_eq!(header.size(), (INDEX_2 - INDEX_1) + size_of::<u32>());

    let sexpr = buf.as_sexpr().cdr().unwrap().car().unwrap();
    let header = sexpr.header().unwrap();
    assert!(header.is_atom());
    assert_eq!(header.size(), (INDEX_2 - INDEX_1));
    assert_eq!(sexpr.len(), (INDEX_2 - INDEX_1) + size_of::<u32>());
    assert_eq!(sexpr.atom().unwrap(), &data[INDEX_1..INDEX_2]);

    let sexpr = buf.as_sexpr().cdr().unwrap().cdr().unwrap();
    let header = sexpr.header().unwrap();
    assert!(header.is_atom());
    assert_eq!(header.size(), 0);
    assert_eq!(sexpr.len(), size_of::<u32>());
    assert_eq!(sexpr.atom().unwrap(), &[]);
}

#[test]
fn chobit_sexpr_test_4() {
    const INDEX_1: usize = 10;
    const INDEX_2: usize = 30;
    const INDEX_3: usize = 100;

    let data = gen_test_data();

    let buf = ChobitSexprBuf::new()
        .build_list()
        .push_item(
            ChobitSexprBuf::new().push_atom(&data[..INDEX_1]).as_sexpr()
        ).push_item(
            ChobitSexprBuf::new().push_atom(&data[INDEX_1..INDEX_2]).as_sexpr()
        ).finish_with(
            ChobitSexprBuf::new().push_atom(&data[INDEX_2..INDEX_3]).as_sexpr()
        );

    let header = buf.as_sexpr().header().unwrap();
    assert!(header.is_cons());
    assert_eq!(header.size(), INDEX_1 + size_of::<u32>());

    let sexpr = buf.as_sexpr().car().unwrap();
    let header = sexpr.header().unwrap();
    assert!(header.is_atom());
    assert_eq!(header.size(), INDEX_1);
    assert_eq!(sexpr.len(), INDEX_1 + size_of::<u32>());
    assert_eq!(sexpr.atom().unwrap(), &data[..INDEX_1]);

    let sexpr = buf.as_sexpr().cdr().unwrap();
    let header = sexpr.header().unwrap();
    assert!(header.is_cons());
    assert_eq!(header.size(), (INDEX_2 - INDEX_1) + size_of::<u32>());

    let sexpr = buf.as_sexpr().cdr().unwrap().car().unwrap();
    let header = sexpr.header().unwrap();
    assert!(header.is_atom());
    assert_eq!(header.size(), (INDEX_2 - INDEX_1));
    assert_eq!(sexpr.len(), (INDEX_2 - INDEX_1) + size_of::<u32>());
    assert_eq!(sexpr.atom().unwrap(), &data[INDEX_1..INDEX_2]);

    let sexpr = buf.as_sexpr().cdr().unwrap().cdr().unwrap();
    let header = sexpr.header().unwrap();
    assert!(header.is_atom());
    assert_eq!(header.size(), (INDEX_3 - INDEX_2));
    assert_eq!(sexpr.len(), (INDEX_3 - INDEX_2) + size_of::<u32>());
    assert_eq!(sexpr.atom().unwrap(), &data[INDEX_2..INDEX_3]);
}

#[test]
fn chobit_sexpr_test_5() {
    const INDEX_1: usize = 10;
    const INDEX_2: usize = 30;
    const INDEX_3: usize = 100;

    let data = gen_test_data();

    let mut buf = ChobitSexprBuf::new()
        .build_list()
        .push_item(
            ChobitSexprBuf::new().push_atom(&data[..INDEX_1]).as_sexpr()
        ).push_item(
            ChobitSexprBuf::new().push_atom(&data[INDEX_1..INDEX_2]).as_sexpr()
        ).finish_with(
            ChobitSexprBuf::new().push_atom(&data[INDEX_2..INDEX_3]).as_sexpr()
        );

    let new_data: Vec<u8> =
        data[INDEX_1..INDEX_2].iter().rev().map(|x| *x).collect();

    buf.cdr_mut().unwrap()
        .car_mut().unwrap()
        .atom_mut().unwrap()
        .copy_from_slice(&new_data);

    let header = buf.header().unwrap();
    assert!(header.is_cons());
    assert_eq!(header.size(), INDEX_1 + size_of::<u32>());

    let sexpr = buf.car().unwrap();
    let header = sexpr.header().unwrap();
    assert!(header.is_atom());
    assert_eq!(header.size(), INDEX_1);
    assert_eq!(sexpr.len(), INDEX_1 + size_of::<u32>());
    assert_eq!(sexpr.atom().unwrap(), &data[..INDEX_1]);

    let sexpr = buf.cdr().unwrap();
    let header = sexpr.header().unwrap();
    assert!(header.is_cons());
    assert_eq!(header.size(), (INDEX_2 - INDEX_1) + size_of::<u32>());

    let sexpr = buf.cdr().unwrap().car().unwrap();
    let header = sexpr.header().unwrap();
    assert!(header.is_atom());
    assert_eq!(header.size(), (INDEX_2 - INDEX_1));
    assert_eq!(sexpr.len(), (INDEX_2 - INDEX_1) + size_of::<u32>());
    assert_ne!(sexpr.atom().unwrap(), &data[INDEX_1..INDEX_2]);
    assert_eq!(sexpr.atom().unwrap(), &new_data);

    let sexpr = buf.cdr().unwrap().cdr().unwrap();
    let header = sexpr.header().unwrap();
    assert!(header.is_atom());
    assert_eq!(header.size(), (INDEX_3 - INDEX_2));
    assert_eq!(sexpr.len(), (INDEX_3 - INDEX_2) + size_of::<u32>());
    assert_eq!(sexpr.atom().unwrap(), &data[INDEX_2..INDEX_3]);
}

#[test]
fn chobit_sexpr_test_6() {
    let data = gen_test_data();

    let buf = ChobitSexprBuf::new().push_atom(data.as_slice());
    let header = buf.as_sexpr().header().unwrap();

    assert!(header.is_atom());
    assert_eq!(header.size(), data.len());
    assert_eq!(buf.as_sexpr().atom().unwrap(), data.as_slice());

    let new_data: Vec<u8> = data.iter().rev().map(|x| *x).collect();
    let buf = buf.clear().push_atom(&new_data);

    assert!(header.is_atom());
    assert_eq!(header.size(), new_data.len());
    assert_eq!(buf.as_sexpr().atom().unwrap(), new_data.as_slice());
    assert_ne!(buf.as_sexpr().atom().unwrap(), data.as_slice());
}

#[test]
fn chobit_sexpr_error_test_1() {
    let data = gen_test_data();

    let buf = ChobitSexprBuf::new().push_atom(data.as_slice());
    assert!(buf.car().is_none());
    assert!(buf.cdr().is_none());

    let mut new_data = buf.to_vec();
    let len = new_data.len();
    new_data.truncate(len - 1);

    let sexpr = ChobitSexpr::new(&new_data);
    assert_eq!(sexpr.as_bytes(), new_data.as_slice());
    assert!(sexpr.car().is_none());
    assert!(sexpr.cdr().is_none());
    assert!(sexpr.atom().is_none());
}

#[test]
fn chobit_sexpr_error_test_2() {
    const INDEX_1: usize = 10;
    const INDEX_2: usize = 30;

    let data = gen_test_data();

    let buf = ChobitSexprBuf::new()
        .build_cons()
        .push_car(
            ChobitSexprBuf::new().push_atom(&data[..INDEX_1]).as_sexpr()
        ).push_cdr(
            ChobitSexprBuf::new().push_atom(&data[INDEX_1..INDEX_2]).as_sexpr()
        );

    let mut new_data = buf.to_vec();
    new_data.truncate(data[..INDEX_1].len() + (size_of::<u32>() * 2) - 1);

    let sexpr = ChobitSexpr::new(&new_data);
    assert!(sexpr.car().is_none());
    assert!(sexpr.cdr().is_none());
    assert!(sexpr.atom().is_none());

    let header = sexpr.header().unwrap();
    assert!(header.is_cons());
    assert_eq!(header.size(), new_data.len() - size_of::<u32>() + 1);
}

#[test]
fn chobit_sexpr_convert_test() {
    const COUNT: usize = 120;

    macro_rules! convert_test_core {
        ($type:ty, $num:expr) => {{
            let val: $type = $num as $type;

            let buf = ChobitSexprBuf::from(val);
            let result = <$type>::try_from(buf.as_sexpr()).unwrap();
            assert_eq!(result, val);

            let buf: ChobitSexprBuf<Completed> = val.into();
            let result: $type = buf.as_sexpr().try_into().unwrap();
            assert_eq!(result, val);
        }}
    }

    for i in 0..COUNT {
        convert_test_core!(i8, i);
        convert_test_core!(u8, i);
        convert_test_core!(i16, i);
        convert_test_core!(u16, i);
        convert_test_core!(i32, i);
        convert_test_core!(u32, i);
        convert_test_core!(i64, i);
        convert_test_core!(u64, i);
        convert_test_core!(i128, i);
        convert_test_core!(u128, i);
        convert_test_core!(f32, i);
        convert_test_core!(f64, i);
    }

    let val = "hello world";

    let buf = ChobitSexprBuf::from(val);
    let result = <&str>::try_from(buf.as_sexpr()).unwrap();
    assert_eq!(result, val);

    let buf: ChobitSexprBuf<Completed> = val.into();
    let result: &str = buf.as_sexpr().try_into().unwrap();
    assert_eq!(result, val);
}
