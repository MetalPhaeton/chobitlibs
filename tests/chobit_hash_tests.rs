extern crate chobitlibs;

use chobitlibs::chobit_hash::*;

#[test]
fn hash_test() {
    const TEXT: &str = "Hello Alice! My name is Bob!";

    assert_eq!(fnv_1a_32(TEXT.as_bytes()), 0x33df93adu32);
    assert_eq!(fnv_1a_64(TEXT.as_bytes()), 0x5a4292cabf34942du64);
    assert_eq!(
        fnv_1a_128(TEXT.as_bytes()),
        0xfcd99fca263c0f52aad9258dfbfb7d3du128
    );
}
