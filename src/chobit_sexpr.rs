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

//! Structured byte string library.
//!
//! This library needs `alloc` crate.
//!
//! ```ignore
//! extern crate alloc;
//! ```
//!
//! # Structure of ChobitSexpr
//!
//! There are 2 types of [ChobitSexpr].  
//! One type is __Atom__ , another is __Cons__ .
//!
//! ## Atom
//!
//! __Atom__ consists of [SexprHeader] and payload.
//!
//! | Position | Contents |
//! |-|-|
//! | The first of 4 bytes | [SexprHeader]. That contains atom flag and a size of payload in __little endian__ . |
//! | The rest of bytes | Payload. That contains byte data. |
//!
//! ## Cons
//!
//! __Cons__ consists of [SexprHeader] and __car__ and __cdr__.  
//! __Car__ and __cdr__ are [ChobitSexpr].
//!
//! | Position | Contents |
//! |-|-|
//! | The first of 4 bytes | [SexprHeader]. That contains cons flag and a size of car in __little endian__ . |
//! | Next bytes that size is written in header | [ChobitSexpr]. That is called __car__. |
//! | The rest of bytes | [ChobitSexpr]. That is called __cdr__. |

use alloc::{
    vec::Vec,
    borrow::{Borrow, ToOwned}
};

use core::{
    mem::size_of,
    slice::{from_raw_parts, from_raw_parts_mut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    fmt
};

/// Error for [ChobitSexpr].
#[derive(Debug, Clone, PartialEq)]
pub enum ChobitSexprError {
    /// Error for [SexprHeader].
    HeaderError(SexprHeaderError),

    /// Error in reading value.
    CouldNotRead(ValueType),

    /// Error in writing value.
    CouldNotWrite(ValueType),

    /// It is not [ChobitSexpr].
    NotSexpr,

    /// It is not atom of [ChobitSexpr].
    NotAtom,

    /// It is not cons of [ChobitSexpr].
    NotCons
}

/// Error for [SexprHeader].
#[derive(Debug, Clone, PartialEq)]
pub enum SexprHeaderError {
    /// Couldn't convert from `&[u8]` to [SexprHeader].
    CouldNotConvertFromSlice
}

/// Value type for [ChobitSexprError].
#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    U128,
    I128,
    F32,
    F64,
    Str
}

/// Header size on byte string.
pub const HEADER_SIZE: usize = size_of::<u32>();

/// Mask for flag.
///
/// ```ignore
/// use chobitlibs::chobit_sexpr::*;
///
/// let header = SexprHeader::new_atom(10);
/// assert_eq!(header.to_u32() & FLAG_MASK, ATOM_FLAG);
///
/// let header = SexprHeader::new_cons(10);
/// assert_eq!(header.to_u32() & FLAG_MASK, CONS_FLAG);
/// ```
pub const FLAG_MASK: u32 = 0b10000000_00000000_00000000_00000000;

/// Mask for size.
///
/// ```ignore
/// use chobitlibs::chobit_sexpr::*;
///
/// let header = SexprHeader::new_atom(10);
/// assert_eq!(header.to_u32() & SIZE_MASK, 10);
///
/// let header = SexprHeader::new_cons(10);
/// assert_eq!(header.to_u32() & SIZE_MASK, 10);
/// ```
pub const SIZE_MASK: u32 = !FLAG_MASK;

/// Max size of ChobitSexpr.
pub const SIZE_MAX: usize = SIZE_MASK as usize;

/// Flag of atom.
///
/// See [FLAG_MASK] for details.
pub const ATOM_FLAG: u32 = 0;

/// Flag of cons.
///
/// See [FLAG_MASK] for details.
pub const CONS_FLAG: u32 = FLAG_MASK;

/// Header of ChobitSexpr.
///
/// [SexprHeader] is `u32` value.  
/// This is written on byte string in __little endian__ .
///
/// | Position | Cotents |
/// |-|-|
/// | The hightest of 1 bit | Flag. If sexpr is atom, 0. If sexpr is cons, 1. |
/// | The rest of bits | If the sexpr is atom, a size of payload. If it is cons, a size of car. |
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct SexprHeader {
    body: u32
}

impl SexprHeader {
    /// Creates from slice.
    ///
    /// - `slice` : Header as byte string.
    /// - _Return_ : If slice length is 4 bytes or more, returns instance. Otherwise returns error.
    #[inline]
    pub fn from_slice(slice: &[u8]) -> Result<Self, SexprHeaderError> {
        if slice.len() >= HEADER_SIZE {
            Ok(
                Self {
                    body: u32::from_le(unsafe {*(slice.as_ptr() as *const u32)})
                }
            )
        } else {
            Err(SexprHeaderError::CouldNotConvertFromSlice)
        }
    }

    /// Creates Nil header.
    ///
    /// Nil is 0 size atom.
    ///
    /// - _Return_ : Instance.
    #[inline]
    pub const fn new_nil() -> Self {
        Self {body: 0}
    }

    #[inline]
    const fn new_core(flag: u32, size: usize) -> u32 {
        flag | ((size as u32) & SIZE_MASK)
    }

    /// Creates atom header.
    ///
    /// - `size` : Size of payload.
    /// - _Return_ : Instance.
    #[inline]
    pub const fn new_atom(size: usize) -> Self {
        Self {body: Self::new_core(ATOM_FLAG, size)}
    }

    /// Creates atom header.
    ///
    /// - `car_size` : Size of sexpr on car.
    /// - _Return_ : Instance.
    #[inline]
    pub const fn new_cons(car_size: usize) -> Self {
        Self {body: Self::new_core(CONS_FLAG, car_size)}
    }

    /// Convert into u32.
    ///
    /// - _Return_ : Header as u32.
    #[inline]
    pub const fn to_u32(&self) -> u32 {
        self.body
    }

    /// Judge if atom or not.
    ///
    /// - _Return_ : If atom, true.
    #[inline]
    pub fn is_atom(&self) -> bool {
        self.body < FLAG_MASK
    }

    /// Judge if cons or not.
    ///
    /// - _Return_ : If cons, true.
    #[inline]
    pub fn is_cons(&self) -> bool {
        self.body >= FLAG_MASK
    }

    /// Gets size.
    ///
    /// - _Return_ : If atom, returns size of payload. If cons, returns size of car.
    #[inline]
    pub fn size(&self) -> usize {
        (self.body & SIZE_MASK) as usize
    }
}

impl Deref for SexprHeader {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &u32 {
        &self.body
    }
}

impl From<u32> for SexprHeader {
    #[inline]
    fn from(src: u32) -> Self {
        Self {body: src}
    }
}

impl From<SexprHeader> for u32 {
    #[inline]
    fn from(header: SexprHeader) -> Self {
        header.body
    }
}

impl From<[u8; HEADER_SIZE]> for SexprHeader {
    #[inline]
    fn from(bytes: [u8; HEADER_SIZE]) -> Self {
        Self {
            body: u32::from_le_bytes(bytes)
        }
    }
}

impl From<SexprHeader> for [u8; HEADER_SIZE] {
    #[inline]
    fn from(header: SexprHeader) -> Self {
        header.body.to_le_bytes()
    }
}

/// Structured byte string.
///
/// # Example of atom
///
/// ```ignore
/// extern crate alloc;
/// use alloc::vec::Vec;
///
/// use chobitlibs::chobit_sexpr::{SexprHeader, ChobitSexpr};
///
/// let payload: [u8; 5] = [1, 2, 3, 4, 5];
///
/// let header = SexprHeader::new_atom(payload.len());
///
/// let mut body = Vec::<u8>::new();
///
/// body.extend_from_slice(&header.to_u32().to_le_bytes());
/// body.extend_from_slice(&payload);
///
/// let sexpr = ChobitSexpr::new(&body);
///
/// assert_eq!(sexpr.atom().unwrap(), payload.as_slice());
/// ```
///
/// # Example of cons
///
/// ```ignore
/// extern crate alloc;
/// use alloc::vec::Vec;
///
/// use chobitlibs::chobit_sexpr::{SexprHeader, ChobitSexpr};
///
/// let car_payload: [u8; 5] = [1, 2, 3, 4, 5];
///
/// let car_body = {
///
///     let header = SexprHeader::new_atom(car_payload.len());
///
///     let mut body = Vec::<u8>::new();
///
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&car_payload);
///
///     body
/// };
///
/// let car = ChobitSexpr::new(&car_body);
///
/// let cdr_payload: [u8; 5] = [6, 7, 8, 9, 10];
///
/// let cdr_body = {
///     let header = SexprHeader::new_atom(cdr_payload.len());
///
///     let mut body = Vec::<u8>::new();
///
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&cdr_payload);
///
///     body
/// };
///
/// let cdr = ChobitSexpr::new(&cdr_body);
///
/// let mut body = Vec::<u8>::new();
/// let header = SexprHeader::new_cons(car.as_bytes().len());
///
/// body.extend_from_slice(&header.to_u32().to_le_bytes());
/// body.extend_from_slice(car.as_bytes());
/// body.extend_from_slice(cdr.as_bytes());
///
/// let sexpr = ChobitSexpr::new(&body);
///
/// assert_eq!(sexpr.car().unwrap().as_bytes(), car.as_bytes());
/// assert_eq!(sexpr.cdr().unwrap().as_bytes(), cdr.as_bytes());
///
/// assert_eq!(sexpr.car().unwrap().atom().unwrap(), car_payload.as_slice());
/// assert_eq!(sexpr.cdr().unwrap().atom().unwrap(), cdr_payload.as_slice());
/// ```
///
/// # Example of TryFrom trait
///
/// ```ignore
/// extern crate alloc;
/// use alloc::vec::Vec;
///
/// use chobitlibs::chobit_sexpr::{SexprHeader, ChobitSexpr};
///
/// {
///     let value: i8 = 111;
///     let header = SexprHeader::new_atom(std::mem::size_of::<i8>());
///
///     let mut body = Vec::<u8>::new();
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&value.to_le_bytes());
///
///     let sexpr = ChobitSexpr::new(&body);
///
///     assert_eq!(value, i8::try_from(sexpr).unwrap());
/// }
///
/// {
///     let value: u8 = 111;
///     let header = SexprHeader::new_atom(std::mem::size_of::<u8>());
///
///     let mut body = Vec::<u8>::new();
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&value.to_le_bytes());
///
///     let sexpr = ChobitSexpr::new(&body);
///
///     assert_eq!(value, u8::try_from(sexpr).unwrap());
/// }
///
/// {
///     let value: i16 = 111;
///     let header = SexprHeader::new_atom(std::mem::size_of::<i16>());
///
///     let mut body = Vec::<u8>::new();
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&value.to_le_bytes());
///
///     let sexpr = ChobitSexpr::new(&body);
///
///     assert_eq!(value, i16::try_from(sexpr).unwrap());
/// }
///
/// {
///     let value: u16 = 111;
///     let header = SexprHeader::new_atom(std::mem::size_of::<u16>());
///
///     let mut body = Vec::<u8>::new();
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&value.to_le_bytes());
///
///     let sexpr = ChobitSexpr::new(&body);
///
///     assert_eq!(value, u16::try_from(sexpr).unwrap());
/// }
///
/// {
///     let value: i32 = 111;
///     let header = SexprHeader::new_atom(std::mem::size_of::<i32>());
///
///     let mut body = Vec::<u8>::new();
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&value.to_le_bytes());
///
///     let sexpr = ChobitSexpr::new(&body);
///
///     assert_eq!(value, i32::try_from(sexpr).unwrap());
/// }
///
/// {
///     let value: u32 = 111;
///     let header = SexprHeader::new_atom(std::mem::size_of::<u32>());
///
///     let mut body = Vec::<u8>::new();
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&value.to_le_bytes());
///
///     let sexpr = ChobitSexpr::new(&body);
///
///     assert_eq!(value, u32::try_from(sexpr).unwrap());
/// }
///
/// {
///     let value: i64 = 111;
///     let header = SexprHeader::new_atom(std::mem::size_of::<i64>());
///
///     let mut body = Vec::<u8>::new();
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&value.to_le_bytes());
///
///     let sexpr = ChobitSexpr::new(&body);
///
///     assert_eq!(value, i64::try_from(sexpr).unwrap());
/// }
///
/// {
///     let value: u64 = 111;
///     let header = SexprHeader::new_atom(std::mem::size_of::<u64>());
///
///     let mut body = Vec::<u8>::new();
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&value.to_le_bytes());
///
///     let sexpr = ChobitSexpr::new(&body);
///
///     assert_eq!(value, u64::try_from(sexpr).unwrap());
/// }
///
/// {
///     let value: i128 = 111;
///     let header = SexprHeader::new_atom(std::mem::size_of::<i128>());
///
///     let mut body = Vec::<u8>::new();
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&value.to_le_bytes());
///
///     let sexpr = ChobitSexpr::new(&body);
///
///     assert_eq!(value, i128::try_from(sexpr).unwrap());
/// }
///
/// {
///     let value: u128 = 111;
///     let header = SexprHeader::new_atom(std::mem::size_of::<u128>());
///
///     let mut body = Vec::<u8>::new();
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&value.to_le_bytes());
///
///     let sexpr = ChobitSexpr::new(&body);
///
///     assert_eq!(value, u128::try_from(sexpr).unwrap());
/// }
///
/// {
///     let value: f32 = 111.0;
///     let header = SexprHeader::new_atom(std::mem::size_of::<f32>());
///
///     let mut body = Vec::<u8>::new();
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&value.to_bits().to_le_bytes());
///
///     let sexpr = ChobitSexpr::new(&body);
///
///     assert_eq!(value, f32::try_from(sexpr).unwrap());
/// }
///
/// {
///     let value: f64 = 111.0;
///     let header = SexprHeader::new_atom(std::mem::size_of::<f64>());
///
///     let mut body = Vec::<u8>::new();
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&value.to_bits().to_le_bytes());
///
///     let sexpr = ChobitSexpr::new(&body);
///
///     assert_eq!(value, f64::try_from(sexpr).unwrap());
/// }
///
/// {
///     let value: &str = "Hello World";
///     let header = SexprHeader::new_atom(value.as_bytes().len());
///
///     let mut body = Vec::<u8>::new();
///     body.extend_from_slice(&header.to_u32().to_le_bytes());
///     body.extend_from_slice(&value.as_bytes());
///
///     let sexpr = ChobitSexpr::new(&body);
///
///     assert_eq!(value, <&str>::try_from(sexpr).unwrap());
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct ChobitSexpr {
    body: [u8]
}

macro_rules! gen_read_doc {
    ($type:ty) => {
        concat!(
            "Reads `",
            stringify!($type), 
r#"` value.

* _Return_ : If the sexpr is atom and the size equals `size_of::<"#,
            stringify!($type), 
r#">()` , returns value. Otherwise, error"#
        )
    };
}

macro_rules! gen_write_doc {
    ($type:ty) => {
        concat!(
            "Writes `",
            stringify!($type), 
r#"` value.

* `value` : Value.
* _Return_ : If the sexpr is atom and the size equals `size_of::<"#,
            stringify!($type), 
r#">()` , returns `Some(())` . Otherwise, error"#
        )
    };
}

macro_rules! value_type {
    (u8) => {ValueType::U8};
    (i8) => {ValueType::I8};
    (u16) => {ValueType::U16};
    (i16) => {ValueType::I16};
    (u32) => {ValueType::U32};
    (i32) => {ValueType::I32};
    (u64) => {ValueType::U64};
    (i64) => {ValueType::I64};
    (u128) => {ValueType::U128};
    (i128) => {ValueType::I128};
}

macro_rules! def_read_write {
    (
        $read_func_name: ident,
        $write_func_name:ident,
        $type:tt
    ) => {
        #[doc = gen_read_doc!($type)]
        #[inline]
        pub fn $read_func_name(&self) -> Result<$type, ChobitSexprError> {
            let atom = self.atom()?;

            if atom.len() == size_of::<$type>() {
                unsafe {
                    Ok(<$type>::from_le(*(atom.as_ptr() as *const $type)))
                }
            } else {
                Err(ChobitSexprError::CouldNotRead(value_type!($type)))
            }
        }

        #[doc = gen_write_doc!($type)]
        #[inline]
        pub fn $write_func_name(
            &mut self,
            value: $type
        ) -> Result<(), ChobitSexprError> {
            let atom = self.atom_mut()?;

            if atom.len() == size_of::<$type>() {
                unsafe {
                    *(atom.as_mut_ptr() as *mut $type) = value.to_le();
                }

                Ok(())
            } else {
                Err(ChobitSexprError::CouldNotWrite(value_type!($type)))
            }
        }
    };
}

impl ChobitSexpr {
    /// Creates immutable ChobitSexpr.
    ///
    /// - `value` : Body of the instance.
    /// - _Return_ : Instance.
    #[inline]
    pub fn new<S: AsRef<[u8]> + ?Sized>(value: &S) -> &ChobitSexpr {
        unsafe {&*(value.as_ref() as *const [u8] as *const ChobitSexpr)}
    }

    /// Creates mutable ChobitSexpr.
    ///
    /// - `value` : Body of the instance.
    /// - _Return_ : Instance.
    #[inline]
    pub fn new_mut<S: AsMut<[u8]> + ?Sized>(
        value: &mut S
    ) -> &mut ChobitSexpr {
        unsafe {&mut *(value.as_mut() as *mut [u8] as *mut ChobitSexpr)}
    }

    /// Gets body as slice.
    ///
    /// - _Return_ : Body.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {&self.body}

    /// Gets header.
    ///
    /// - _Return_ : If it has header, returns it. otherwise returns error.
    #[inline]
    pub fn header(&self) -> Result<SexprHeader, ChobitSexprError> {
        SexprHeader::from_slice(&self.body).map_err(
            |error| ChobitSexprError::HeaderError(error)
        )
    }

    #[inline]
    fn get_atom_size(&self) -> Result<usize, ChobitSexprError> {
        let header = self.header()?;

        if header.is_atom() {
            let size = header.size();

            if size <= (self.body.len() - HEADER_SIZE) {
                Ok(size)
            } else {
                Err(ChobitSexprError::NotSexpr)
            }
        } else {
            Err(ChobitSexprError::NotAtom)
        }
    }

    /// Gets immutable payload of atom.
    ///
    /// - _Return_ : If it is correct atom, returns its payload. otherwise returns error.
    #[inline]
    pub fn atom(&self) -> Result<&[u8], ChobitSexprError> {
        let size = self.get_atom_size()?;

        Ok(unsafe {
            from_raw_parts(self.body.as_ptr().add(HEADER_SIZE), size)
        })
    }

    /// Gets mutable payload of atom.
    ///
    /// - _Return_ : If it is correct atom, returns its payload. otherwise returns error.
    #[inline]
    pub fn atom_mut(&mut self) -> Result<&mut [u8], ChobitSexprError> {
        let size = self.get_atom_size()?;

        Ok(unsafe {
            from_raw_parts_mut(self.body.as_mut_ptr().add(HEADER_SIZE), size)
        })
    }

    #[inline]
    fn cons_size(&self) -> Result<usize, ChobitSexprError> {
        let header = self.header()?;

        if header.is_cons() {
            let size = header.size();

            if size <= (self.body.len() - HEADER_SIZE) {
                Ok(size)
            } else {
                Err(ChobitSexprError::NotSexpr)
            }
        } else {
            Err(ChobitSexprError::NotCons)
        }
    }

    /// Gets immutable car of cons.
    ///
    /// - _Return_ : If it is correct cons, returns its car. otherwise returns error.
    #[inline]
    pub fn car(&self) -> Result<&ChobitSexpr, ChobitSexprError> {
        let size = self.cons_size()?;

        Ok(ChobitSexpr::new(unsafe {
            from_raw_parts(
                self.body.as_ptr().add(HEADER_SIZE),
                size
            )
        }))
    }

    /// Gets mutable car of cons.
    ///
    /// - _Return_ : If it is correct cons, returns its car. otherwise returns error.
    #[inline]
    pub fn car_mut(&mut self) -> Result<&mut ChobitSexpr, ChobitSexprError> {
        let size = self.cons_size()?;

        Ok(ChobitSexpr::new_mut(unsafe {
            from_raw_parts_mut(
                self.body.as_mut_ptr().add(HEADER_SIZE),
                size
            )
        }))
    }

    /// Gets immutable cdr of cons.
    ///
    /// - _Return_ : If it is correct cons, returns its cdr. otherwise returns error.
    #[inline]
    pub fn cdr(&self) -> Result<&ChobitSexpr, ChobitSexprError> {
        let cdr_pos = self.cons_size()? + HEADER_SIZE;

        Ok(ChobitSexpr::new(unsafe {
            from_raw_parts(
                self.body.as_ptr().add(cdr_pos),
                self.body.len() - cdr_pos
            )
        }))
    }

    /// Gets mutable cdr of cons.
    ///
    /// - _Return_ : If it is correct cons, returns its cdr. otherwise returns error.
    #[inline]
    pub fn cdr_mut(&mut self) -> Result<&mut ChobitSexpr, ChobitSexprError> {
        let cdr_pos = self.cons_size()? + HEADER_SIZE;

        Ok(ChobitSexpr::new_mut(unsafe {
            from_raw_parts_mut(
                self.body.as_mut_ptr().add(cdr_pos),
                self.body.len() - cdr_pos
            )
        }))
    }

    /// Gets immutable car and cdr of cons.
    ///
    /// - _Return_ : If it is correct cons, returns its car and cdr. otherwise returns error.
    #[inline]
    pub fn car_cdr(
        &self
    ) -> Result<(&ChobitSexpr, &ChobitSexpr), ChobitSexprError> {
        let car_size = self.cons_size()? + HEADER_SIZE;

        Ok((
            ChobitSexpr::new(unsafe{
                from_raw_parts(
                    self.body.as_ptr().add(HEADER_SIZE),
                    car_size
                )
            }),
            ChobitSexpr::new(unsafe{
                from_raw_parts(
                    self.body.as_ptr().add(car_size),
                    self.body.len() - car_size
                )
            }),
        ))
    }

    def_read_write!(read_i8, write_i8, i8);
    def_read_write!(read_u8, write_u8, u8);
    def_read_write!(read_i16, write_i16, i16);
    def_read_write!(read_u16, write_u16, u16);
    def_read_write!(read_i32, write_i32, i32);
    def_read_write!(read_u32, write_u32, u32);
    def_read_write!(read_i64, write_i64, i64);
    def_read_write!(read_u64, write_u64, u64);
    def_read_write!(read_i128, write_i128, i128);
    def_read_write!(read_u128, write_u128, u128);

    #[doc = gen_read_doc!(f32)]
    #[inline]
    pub fn read_f32(&self) -> Result<f32, ChobitSexprError> {
        let atom = self.atom()?;

        if atom.len() == size_of::<f32>() {
            unsafe {
                Ok(f32::from_bits(
                    u32::from_le(*(atom.as_ptr() as *const u32))
                ))
            }
        } else {
            Err(ChobitSexprError::CouldNotRead(ValueType::F32))
        }
    }

    #[doc = gen_write_doc!(f32)]
    #[inline]
    pub fn write_f32(&mut self, value: f32) -> Result<(), ChobitSexprError> {
        let atom = self.atom_mut()?;

        if atom.len() == size_of::<f32>() {
            unsafe {
                *(atom.as_mut_ptr() as *mut u32) = value.to_bits().to_le();
            }

            Ok(())
        } else {
            Err(ChobitSexprError::CouldNotWrite(ValueType::F32))
        }
    }

    #[doc = gen_read_doc!(f64)]
    #[inline]
    pub fn read_f64(&self) -> Result<f64, ChobitSexprError> {
        let atom = self.atom()?;

        if atom.len() == size_of::<f64>() {
            unsafe {
                Ok(f64::from_bits(
                    u64::from_le(*(atom.as_ptr() as *const u64))
                ))
            }
        } else {
            Err(ChobitSexprError::CouldNotRead(ValueType::F64))
        }
    }

    #[doc = gen_write_doc!(f64)]
    #[inline]
    pub fn write_f64(&mut self, value: f64) -> Result<(), ChobitSexprError> {
        let atom = self.atom_mut()?;

        if atom.len() == size_of::<f64>() {
            unsafe {
                *(atom.as_mut_ptr() as *mut u64) = value.to_bits().to_le();
            }

            Ok(())
        } else {
            Err(ChobitSexprError::CouldNotWrite(ValueType::F64))
        }
    }

    /// Generates an iterator.
    ///
    /// If the sexpr is a list, iterates each car.
    ///
    /// - _Return_ : Iterator.
    ///
    /// ```ignore
    /// use chobitlibs::chobit_sexpr::{ChobitSexpr, ChobitSexprBuf};
    ///
    /// let buf = ChobitSexprBuf::new().build_list().push_item(
    ///     &ChobitSexprBuf::from(100i32)
    /// ).push_item(
    ///     &ChobitSexprBuf::from(200i32)
    /// ).push_item(
    ///     &ChobitSexprBuf::from(300i32)
    /// ).finish();
    ///
    /// let result: Vec<i32> = buf.as_sexpr().iter().map(
    ///     |elm| elm.read_i32().unwrap()
    /// ).collect();
    ///
    /// assert_eq!(result.as_slice(), [100i32, 200i32, 300i32].as_slice());
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter {
        Iter {body: self}
    }
}

pub struct Iter<'a> {
    body: &'a ChobitSexpr
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a ChobitSexpr;

    #[inline]
    fn next(&mut self) -> Option<&'a ChobitSexpr> {
        let (car, cdr) = self.body.car_cdr().ok()?;
        self.body = cdr;

        Some(car)
    }
}

impl<'a> IntoIterator for &'a ChobitSexpr {
    type Item = Self;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

impl AsRef<ChobitSexpr> for ChobitSexpr {
    #[inline]
    fn as_ref(&self) -> &ChobitSexpr {
        self
    }
}

impl AsMut<ChobitSexpr> for ChobitSexpr {
    #[inline]
    fn as_mut(&mut self) -> &mut ChobitSexpr {
        self
    }
}

impl AsRef<ChobitSexpr> for [u8] {
    #[inline]
    fn as_ref(&self) -> &ChobitSexpr {
        ChobitSexpr::new(self)
    }
}

impl AsMut<ChobitSexpr> for [u8] {
    #[inline]
    fn as_mut(&mut self) -> &mut ChobitSexpr {
        ChobitSexpr::new_mut(self)
    }
}

impl AsRef<ChobitSexpr> for Vec<u8> {
    #[inline]
    fn as_ref(&self) -> &ChobitSexpr {
        ChobitSexpr::new(self)
    }
}

impl AsMut<ChobitSexpr> for Vec<u8> {
    #[inline]
    fn as_mut(&mut self) -> &mut ChobitSexpr {
        ChobitSexpr::new_mut(self)
    }
}

impl<const N: usize> AsRef<ChobitSexpr> for [u8; N] {
    #[inline]
    fn as_ref(&self) -> &ChobitSexpr {
        ChobitSexpr::new(self)
    }
}

impl<const N: usize> AsMut<ChobitSexpr> for [u8; N] {
    #[inline]
    fn as_mut(&mut self) -> &mut ChobitSexpr {
        ChobitSexpr::new_mut(self)
    }
}

impl ToOwned for ChobitSexpr {
    type Owned = ChobitSexprBuf<Completed>;

    fn to_owned(&self) -> ChobitSexprBuf<Completed> {
        ChobitSexprBuf::<Completed> {
            buffer: self.as_bytes().to_vec(),

            _marker: PhantomData::<Completed>
        }
    }

    fn clone_into(&self, target: &mut ChobitSexprBuf<Completed>) {
        target.buffer = self.as_bytes().to_vec();
    }
}

macro_rules! def_try_from {
    ($type:ty, $read_func_name:ident) => {
        impl TryFrom<&ChobitSexpr> for $type {
            type Error = ChobitSexprError;

            #[inline]
            fn try_from(
                sexpr: &ChobitSexpr
            ) -> Result<$type, ChobitSexprError> {
                sexpr.$read_func_name()
            }
        }
    };
}

def_try_from!(i8, read_i8);
def_try_from!(u8, read_u8);
def_try_from!(i16, read_i16);
def_try_from!(u16, read_u16);
def_try_from!(i32, read_i32);
def_try_from!(u32, read_u32);
def_try_from!(i64, read_i64);
def_try_from!(u64, read_u64);
def_try_from!(i128, read_i128);
def_try_from!(u128, read_u128);

impl TryFrom<&ChobitSexpr> for f32 {
    type Error = ChobitSexprError;

    #[inline]
    fn try_from(sexpr: &ChobitSexpr) -> Result<f32, ChobitSexprError> {
        match u32::try_from(sexpr) {
            Ok(bits) => Ok(f32::from_bits(bits)),

            Err(ChobitSexprError::CouldNotRead(..)) =>
                Err(ChobitSexprError::CouldNotRead(ValueType::F32)),

            Err(error) => Err(error)
        }
    }
}

impl TryFrom<&ChobitSexpr> for f64 {
    type Error = ChobitSexprError;

    #[inline]
    fn try_from(sexpr: &ChobitSexpr) -> Result<f64, ChobitSexprError> {
        match u64::try_from(sexpr) {
            Ok(bits) => Ok(f64::from_bits(bits)),

            Err(ChobitSexprError::CouldNotRead(..)) =>
                Err(ChobitSexprError::CouldNotRead(ValueType::F64)),

            Err(error) => Err(error)
        }
    }
}

impl<'a> TryFrom<&'a ChobitSexpr> for &'a str {
    type Error = ChobitSexprError;

    #[inline]
    fn try_from(sexpr: &'a ChobitSexpr) -> Result<&'a str, ChobitSexprError> {
        core::str::from_utf8(sexpr.atom()?).map_err(
            |_| ChobitSexprError::CouldNotRead(ValueType::Str)
        )
    }
}

impl fmt::Display for ChobitSexpr {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_sexpr(0, formatter)
    }
}

impl ChobitSexpr {
    fn write_indent(
        indent: usize,
        formatter: &mut fmt::Formatter
    ) -> fmt::Result {
        for _ in 0..indent {
            write!(formatter, "|    ")?;
        }

        Ok(())
    }

    #[inline]
    fn fmt_sexpr(
        &self,
        indent: usize,
        formatter: &mut fmt::Formatter
    ) -> fmt::Result {
        match self.atom() {
            Ok(atom) => Self::fmt_atom(indent, atom, formatter),

            Err(..) => self.fmt_cons(indent, formatter)
        }
    }

    fn fmt_atom(
        indent: usize,
        atom: &[u8],
        formatter: &mut fmt::Formatter
    ) -> fmt::Result {
        Self::write_indent(indent, formatter)?;

        write!(formatter, "len: {}, bytes: {:02x?}\n", atom.len(), atom)
    }

    fn fmt_cons(
        &self,
        indent: usize,
        formatter: &mut fmt::Formatter
    ) -> fmt::Result {
        let car = self.car().map_err(|_| fmt::Error)?;

        Self::write_indent(indent, formatter)?;
        write!(formatter, "car:\n")?;
        car.fmt_sexpr(indent + 1, formatter)?;

        let cdr = self.cdr().map_err(|_| fmt::Error)?;

        Self::write_indent(indent, formatter)?;
        write!(formatter, "cdr:\n")?;
        cdr.fmt_sexpr(indent + 1, formatter)
    }
}

/// Typestate of ChobitSexprBuf. Indicates empty and imcomplete sexpr.
#[derive(Debug, PartialEq)]
pub enum Empty {}

/// Typestate of ChobitSexprBuf. Indicates comleted sexpr.
#[derive(Debug, PartialEq)]
pub enum Completed {}

/// Typestate of ChobitSexprBuf. Indicates to be able to push car.
#[derive(Debug, PartialEq)]
pub enum Car {}

/// Typestate of ChobitSexprBuf. Indicates to be able to push cdr.
#[derive(Debug, PartialEq)]
pub enum Cdr {}

/// Typestate of ChobitSexprBuf. Indicates to be able to push list item.
#[derive(Debug, PartialEq)]
pub enum List {}

mod private {
    use super::{Empty, Completed, Car, Cdr, List};

    pub trait Sealed {}

    impl Sealed for Empty {}
    impl Sealed for Completed {}
    impl Sealed for Car {}
    impl Sealed for Cdr {}
    impl Sealed for List {}
}

/// [ChobitSexpr] in heap memory. It is also [ChobitSexpr] builder.
#[derive(Debug, Clone, PartialEq)]
pub struct ChobitSexprBuf<Mode = Completed>
where
    Mode: private::Sealed
{
    buffer: Vec<u8>,

    _marker: PhantomData<Mode>
}

impl<Mode> ChobitSexprBuf<Mode> where Mode: private::Sealed {
    /// Drops self and clear buffer and take back [Empty] state.
    ///
    /// - _Return_ : ChobitSexpr of Empty state.
    #[inline]
    pub fn clear(self) -> ChobitSexprBuf<Empty> {
        let Self {mut buffer, ..} = self;

        buffer.clear();

        ChobitSexprBuf::<Empty> {
            buffer: buffer,

            _marker: PhantomData::<Empty>
        }
    }
}

macro_rules! push_number {
    ($func_name:ident, $type:ty, $doc:expr) => {
        #[doc = $doc]
        #[inline]
        pub fn $func_name(self, value: $type) -> ChobitSexprBuf<Completed> {
            self.push_atom(&value.to_le_bytes())
        }
    };
}

macro_rules! push_number_doc {
    ($type:ty) => {
        concat!(
r#"Drops and pushes `"#,
stringify!($type),
r#"` value and returns completed sexpr.

* `value` : A value.
* _Return_ : Completed sexpr"#
        )
    };
}

impl ChobitSexprBuf<Empty> {
    /// Creates ChobitSexprBuf. Not allocated on heap memory yet.
    ///
    /// - _Return_ : Instance.
    #[inline]
    pub fn new() -> Self {
        Self {
            buffer: Vec::<u8>::new(),

            _marker: PhantomData::<Empty>
        }
    }

    /// Creates ChobitSexprBuf with memory allocation.
    ///
    /// - `capacity` : allocation size.
    /// - _Return_ : Instance.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::<u8>::with_capacity(capacity),

            _marker: PhantomData::<Empty>
        }
    }

    /// Drops and returns completed empty sexpr.
    ///
    /// - _Return_ : Completed sexpr.
    #[inline]
    pub fn empty_sexpr(self) -> ChobitSexprBuf<Completed> {
        let Self {buffer, ..} = self;

        ChobitSexprBuf::<Completed> {
            buffer: buffer,
            _marker: PhantomData::<Completed>
        }
    }

    /// Drops and returns instance to be able to push car.
    ///
    /// - _Return_ : ChobitSexprBuf that can push car.
    #[inline]
    pub fn build_cons(self) -> ChobitSexprBuf<Car> {
        let Self {buffer, ..} = self;

        ChobitSexprBuf::<Car> {
            buffer: buffer,

            _marker: PhantomData::<Car>
        }
    }

    /// Drops and returns instance to be able to push list item.
    ///
    /// - _Return_ : ChobitSexprBuf that can push list item.
    #[inline]
    pub fn build_list(self) -> ChobitSexprBuf<List> {
        let Self {buffer, ..} = self;

        ChobitSexprBuf::<List> {
            buffer: buffer,

            _marker: PhantomData::<List>
        }
    }

    /// Drops and pushes sexpr and completes.
    ///
    /// - `sexpr` : Sexpr.
    /// - _Return_ : Comleted sexpr.
    #[inline]
    pub fn push_sexpr(self, sexpr: &ChobitSexpr) -> ChobitSexprBuf<Completed> {
        let Self {mut buffer, ..} = self;

        buffer.extend_from_slice(sexpr.as_bytes());

        ChobitSexprBuf::<Completed> {
            buffer: buffer,

            _marker: PhantomData::<Completed>
        }
    }

    /// Drops and pushes atom and completes.
    ///
    /// - `value` : Payload of atom.
    /// - _Return_ : Completed sexpr.
    pub fn push_atom(self, value: &[u8]) -> ChobitSexprBuf<Completed> {
        let Self {mut buffer, ..} = self;

        buffer.extend_from_slice(
            &SexprHeader::new_atom(value.len()).to_le_bytes()
        );

        buffer.extend_from_slice(value);

        ChobitSexprBuf::<Completed> {
            buffer: buffer,

            _marker: PhantomData::<Completed>
        }
    }

    /// Drops and returns nil sexpr.
    ///
    /// - _Return_ : Completed sexpr.
    pub fn push_nil(self) -> ChobitSexprBuf<Completed> {
        let Self {mut buffer, ..} = self;

        buffer.extend_from_slice(&SexprHeader::new_nil().to_le_bytes());

        ChobitSexprBuf::<Completed> {
            buffer: buffer,

            _marker: PhantomData::<Completed>
        }
    }

    push_number!(push_i8, i8, push_number_doc!(i8));
    push_number!(push_u8, u8, push_number_doc!(u8));
    push_number!(push_i16, i16, push_number_doc!(i16));
    push_number!(push_u16, u16, push_number_doc!(u16));
    push_number!(push_i32, i32, push_number_doc!(i32));
    push_number!(push_u32, u32, push_number_doc!(u32));
    push_number!(push_i64, i64, push_number_doc!(i64));
    push_number!(push_u64, u64, push_number_doc!(u64));
    push_number!(push_i128, i128, push_number_doc!(i128));
    push_number!(push_u128, u128, push_number_doc!(u128));

    #[doc = push_number_doc!(f32)]
    #[inline]
    pub fn push_f32(self, value: f32) -> ChobitSexprBuf<Completed> {
        self.push_u32(value.to_bits())
    }

    #[doc = push_number_doc!(f64)]
    #[inline]
    pub fn push_f64(self, value: f64) -> ChobitSexprBuf<Completed> {
        self.push_u64(value.to_bits())
    }
}

impl ChobitSexprBuf<Completed> {
    /// Borrows self as immutable sexpr.
    ///
    /// - _Return_ : Self as immutable ChobitSexpr.
    #[inline]
    pub fn as_sexpr(&self) -> &ChobitSexpr {
        ChobitSexpr::new(self.buffer.as_slice())
    }

    /// Borrows self as mutable sexpr.
    ///
    /// - _Return_ : Self as mutable ChobitSexpr.
    #[inline]
    pub fn as_mut_sexpr(&mut self) -> &mut ChobitSexpr {
        ChobitSexpr::new_mut(self.buffer.as_mut_slice())
    }

    /// Drops self and returns buffer.
    ///
    /// - _Return_ : buffer.
    #[inline]
    pub fn drop_buffer(self) -> Vec<u8> {
        self.buffer
    }
}

impl ChobitSexprBuf<Car> {
    /// Drops and pushes car and returns instance to be able to push cdr.
    ///
    /// - `sexpr` : sexpr contained on car.
    /// - _Return_ : ChobitSexprBuf that can push cdr.
    pub fn push_car(self, sexpr: &ChobitSexpr) -> ChobitSexprBuf<Cdr> {
        let Self {mut buffer, ..} = self;

        let bytes = sexpr.as_bytes();

        buffer.extend_from_slice(
            &SexprHeader::new_cons(bytes.len()).to_le_bytes()
        );

        buffer.extend_from_slice(bytes);

        ChobitSexprBuf::<Cdr> {
            buffer: buffer,

            _marker: PhantomData::<Cdr>
        }
    }
}

impl ChobitSexprBuf<Cdr> {
    /// Drops and pushes car and completes.
    ///
    /// - `sexpr` : sexpr contained on car.
    /// - _Return_ : Completed sexpr.
    pub fn push_cdr(self, sexpr: &ChobitSexpr) -> ChobitSexprBuf<Completed> {
        let Self {mut buffer, ..} = self;

        buffer.extend_from_slice(sexpr.as_bytes());

        ChobitSexprBuf::<Completed> {
            buffer: buffer,

            _marker: PhantomData::<Completed>
        }
    }
}

impl ChobitSexprBuf<List> {
    /// Drops and pushes list item and returns instance self.
    ///
    /// - `sexpr` : list item.
    /// - _Return_ : Instance that can push list item.
    pub fn push_item(self, sexpr: &ChobitSexpr) -> ChobitSexprBuf<List> {
        let Self {mut buffer, ..} = self;

        let bytes = sexpr.as_bytes();

        buffer.extend_from_slice(
            &SexprHeader::new_cons(bytes.len()).to_le_bytes()
        );

        buffer.extend_from_slice(bytes);

        ChobitSexprBuf::<List> {
            buffer: buffer,

            _marker: PhantomData::<List>
        }
    }

    /// Drops and pushes nil to cdr and completes.
    ///
    /// - _Return_ : Complete sexpr.
    pub fn finish(self) -> ChobitSexprBuf<Completed> {
        let Self {mut buffer, ..} = self;

        buffer.extend_from_slice(&SexprHeader::new_nil().to_le_bytes());

        ChobitSexprBuf::<Completed> {
            buffer: buffer,

            _marker: PhantomData::<Completed>
        }
    }

    /// Drops and pushes sexpr to cdr and completes.
    ///
    /// - `sexpr` : Last sexpr.
    /// - _Return_ : Complete sexpr.
    pub fn finish_with(
        self,
        sexpr: &ChobitSexpr
    ) -> ChobitSexprBuf<Completed> {
        let Self {mut buffer, ..} = self;

        buffer.extend_from_slice(sexpr.as_bytes());

        ChobitSexprBuf::<Completed> {
            buffer: buffer,

            _marker: PhantomData::<Completed>
        }
    }
}

impl Default for ChobitSexprBuf<Empty> {
    #[inline]
    fn default() -> Self {
        ChobitSexprBuf::<Empty>::new()
    }
}

impl Deref for ChobitSexprBuf<Completed> {
    type Target = ChobitSexpr;

    #[inline]
    fn deref(&self) -> &ChobitSexpr {
        self.as_sexpr()
    }
}

impl DerefMut for ChobitSexprBuf<Completed> {
    #[inline]
    fn deref_mut(&mut self) -> &mut ChobitSexpr {
        self.as_mut_sexpr()
    }
}

impl AsRef<ChobitSexpr> for ChobitSexprBuf<Completed> {
    #[inline]
    fn as_ref(&self) -> &ChobitSexpr {
        self.as_sexpr()
    }
}

impl AsMut<ChobitSexpr> for ChobitSexprBuf<Completed> {
    #[inline]
    fn as_mut(&mut self) -> &mut ChobitSexpr {
        self.as_mut_sexpr()
    }
}

impl Borrow<ChobitSexpr> for ChobitSexprBuf<Completed> {
    #[inline]
    fn borrow(&self) -> &ChobitSexpr {
        self.as_sexpr()
    }
}

macro_rules! def_from {
    ($type:ty) => {
        impl From<$type> for ChobitSexprBuf<Completed> {
            #[inline]
            fn from(value: $type) -> Self {
                ChobitSexprBuf::new().push_atom(&value.to_le_bytes())
            }
        }
    };
}

def_from!(i8);
def_from!(u8);
def_from!(i16);
def_from!(u16);
def_from!(i32);
def_from!(u32);
def_from!(i64);
def_from!(u64);
def_from!(i128);
def_from!(u128);

impl From<f32> for ChobitSexprBuf<Completed> {
    #[inline]
    fn from(value: f32) -> Self {
        ChobitSexprBuf::<Completed>::from(value.to_bits())
    }
}

impl From<f64> for ChobitSexprBuf<Completed> {
    #[inline]
    fn from(value: f64) -> Self {
        ChobitSexprBuf::<Completed>::from(value.to_bits())
    }
}

impl From<&str> for ChobitSexprBuf<Completed> {
    #[inline]
    fn from(value: &str) -> Self {
        ChobitSexprBuf::new().push_atom(value.as_bytes())
    }
}
