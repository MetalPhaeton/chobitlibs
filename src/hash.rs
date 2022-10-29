//        DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004 
//
// Copyright (C) 2022 Hironori Ishibashi
//
// Everyone is permitted to copy and distribute verbatim or modified 
// copies of this license document, and changing it is allowed as long 
// as the name is changed. 
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE 
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION 
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

#![allow(dead_code)]

//! Hash function library.

macro_rules! fnv_1a {
    ($type:ty, $bytes:expr, $OFFSET:expr, $PRIME:expr) => {{
        let mut hash: $type = $OFFSET;

        let mut ptr = $bytes.as_ptr();
        let mut len = $bytes.len();

        while len != 0 {
            hash ^= unsafe {*ptr} as $type;
            ptr = unsafe {ptr.add(1)};

            hash = $PRIME.wrapping_mul(hash);

            len -= 1;
        }

        hash
    }};
}

/// 32 bits FNV-1a function.
///
/// * `bytes` : Bytes that you want to calculate hash value.
/// * _Return_ : Hash value.
///
/// ```
/// use chobit::hash::fnv_1a_32;
///
/// const TEXT: &str = "Hello Alice! My name is Bob!";
///
/// assert_eq!(fnv_1a_32(TEXT.as_bytes()), 0x33df93adu32);
/// ```
#[inline]
pub const fn fnv_1a_32(bytes: &[u8]) -> u32 {
    fnv_1a!(u32, bytes, 0x811c9dc5u32, 0x01000193u32)
}

/// 64 bits FNV-1a function.
///
/// * `bytes` : Bytes that you want to calculate hash value.
/// * _Return_ : Hash value.
///
/// ```
/// use chobit::hash::fnv_1a_64;
///
/// const TEXT: &str = "Hello Alice! My name is Bob!";
///
/// assert_eq!(fnv_1a_64(TEXT.as_bytes()), 0x5a4292cabf34942du64);
/// ```
#[inline]
pub const fn fnv_1a_64(bytes: &[u8]) -> u64 {
    fnv_1a!(u64, bytes, 0xcbf29ce484222325u64, 0x00000100000001b3u64)
}

/// 128 bits FNV-1a function.
///
/// * `bytes` : Bytes that you want to calculate hash value.
/// * _Return_ : Hash value.
///
/// ```
/// use chobit::hash::fnv_1a_128;
///
/// const TEXT: &str = "Hello Alice! My name is Bob!";
///
/// assert_eq!(
///     fnv_1a_128(TEXT.as_bytes()),
///     0xfcd99fca263c0f52aad9258dfbfb7d3du128
/// );
/// ```
#[inline]
pub const fn fnv_1a_128(bytes: &[u8]) -> u128 {
    fnv_1a!(
        u128,
        bytes,
        0x6c62272e07bb014262b821756295c58du128,
        0x0000000001000000000000000000013bu128
    )
}
