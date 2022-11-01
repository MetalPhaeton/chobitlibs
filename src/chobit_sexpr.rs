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
//! ```
//! extern crate alloc;
//! ```

use alloc::vec::Vec;

use core::{
    mem::size_of,
    slice::{from_raw_parts, from_raw_parts_mut},
    marker::PhantomData,
    ops::{Deref, DerefMut}
};

const HEADER_SIZE: usize = size_of::<u32>();

const FLAG_MASK: u32 = 0b10000000_00000000_00000000_00000000;
const SIZE_MASK: u32 = !FLAG_MASK;

const SIZE_MAX: usize = SIZE_MASK as usize;

const ATOM_FLAG: u32 = 0;
const CONS_FLAG: u32 = FLAG_MASK;

/// Header of ChobitSexpr.
///
/// [SexprHeader] is `u32` value.  
/// This can use methods of `u32` because of [Deref].
///
/// ```text
/// High bit
/// |---|
/// |   | <-- Cons flag (1 bit) : if header of cons, 1. if header of atom, 0.
/// |---|
/// |   | -|
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |- Size (31 bits) :
/// |---|  |      if header of cons, here is size of car.
/// |   |  |      if header of atom, here is size of payload.
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   | -|
/// |---|
/// Low bit
/// ```
///
/// But this header is recorded on the head of byte string in __little endian__.  
/// So...
///
/// ```text
/// Head of sexpr on byte string.
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   | <--- Cons flag is here, because of little endian.
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |   |
/// |---|
/// |
/// |
/// |
/// |
/// |
/// |
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct SexprHeader {
    body: u32
}

impl SexprHeader {
    /// Creates from slice.
    ///
    /// * `slice` : Header as byte string.
    /// * _Return_ : If slice length is 4 bytes or more, returns instance. Otherwise `None`.
    #[inline]
    pub fn from_slice(slice: &[u8]) -> Option<Self> {
        (slice.len() >= HEADER_SIZE).then(|| {
            Self {
                body: u32::from_le(unsafe {*(slice.as_ptr() as *const u32)})
            }
        })
    }

    /// Creates Nil header.
    ///
    /// Nil is 0 size atom.
    ///
    /// * _Return_ : Instance.
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
    /// * `size` : Size of payload.
    /// * _Return_ : Instance.
    #[inline]
    pub const fn new_atom(size: usize) -> Self {
        Self {body: Self::new_core(ATOM_FLAG, size)}
    }

    /// Creates atom header.
    ///
    /// * `car_size` : Size of sexpr on car.
    /// * _Return_ : Instance.
    #[inline]
    pub const fn new_cons(car_size: usize) -> Self {
        Self {body: Self::new_core(CONS_FLAG, car_size)}
    }

    /// Convert into u32.
    ///
    /// * _Return_ : Header as u32.
    #[inline]
    pub const fn to_u32(&self) -> u32 {
        self.body
    }

    /// Judge if atom or not.
    ///
    /// * _Return_ : If atom, true.
    #[inline]
    pub fn is_atom(&self) -> bool {
        (self.body & FLAG_MASK) == ATOM_FLAG
    }

    /// Judge if cons or not.
    ///
    /// * _Return_ : If cons, true.
    #[inline]
    pub fn is_cons(&self) -> bool {
        (self.body & FLAG_MASK) == CONS_FLAG
    }

    /// Gets size.
    ///
    /// * _Return_ : If atom, returns size of payload. If cons, returns size of car.
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
/// This is byte string structured by S-Expression.
///
/// # Atom
///
/// ```text
/// Head of byte string
/// |---|
/// |   | -|
/// |---|  |
/// |   |  |- Header (4 bytes)
/// |---|  |      Size of atom is written here.
/// |   |  |
/// |---|  |
/// |   | -|
/// |---| 
/// |   | -|
/// |---|  |
/// |   |  |- Payload (size is written in the header)
/// |---|  |      Contains byte data.
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |      |
/// |      |
/// |      |
/// |      |
/// |      |
/// |      |
/// |      |
/// |      |
///
/// # Cons
///
/// ```text
/// Head of byte string
/// |---|
/// |   | -|
/// |---|  |
/// |   |  |- Header (4 bytes)
/// |---|  |      Size of car is written here.
/// |   |  |
/// |---|  |
/// |   | -|
/// |---| 
/// |   | -|
/// |---|  |
/// |   |  |- Car (size is written in the header) 
/// |---|  |      Contains ChobitSexpr.
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |      |
/// |      |
/// |      |
/// |      |
/// |      |
/// |      |
/// |      |
/// |      |
/// |   | -|
/// |---| 
/// |   | -|
/// |---|  |
/// |   |  |- Cdr (the rest of byte string)
/// |---|  |      Contains ChobitSexpr
/// |   |  |
/// |---|  |
/// |   |  |
/// |---|  |
/// |      |
/// |      |
/// |      |
/// |      |
/// |      |
/// |      |
/// |      |
/// |      |
/// ```
#[derive(Debug, PartialEq)]
pub struct ChobitSexpr {
    body: [u8]
}

impl ChobitSexpr {
    /// Creates immutable ChobitSexpr.
    ///
    /// * `value` : Body of the instance.
    /// * _Return_ : Instance.
    #[inline]
    pub fn new<S: AsRef<[u8]> + ?Sized>(value: &S) -> &ChobitSexpr {
        unsafe {&*(value.as_ref() as *const [u8] as *const ChobitSexpr)}
    }

    /// Creates mutable ChobitSexpr.
    ///
    /// * `value` : Body of the instance.
    /// * _Return_ : Instance.
    #[inline]
    pub fn new_mut<S: AsMut<[u8]> + ?Sized>(
        value: &mut S
    ) -> &mut ChobitSexpr {
        unsafe {&mut *(value.as_mut() as *mut [u8] as *mut ChobitSexpr)}
    }

    /// Gets body as slice.
    ///
    /// * _Return_ : Body.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {&self.body}

    /// Gets header.
    ///
    /// * _Return_ : If it has header, returns it. otherwise returns `None`.
    #[inline]
    pub fn header(&self) -> Option<SexprHeader> {
        SexprHeader::from_slice(&self.body)
    }

    #[inline]
    fn get_atom_size(&self) -> Option<usize> {
        let header = self.header()?;

        if header.is_atom() {
            let size = header.size();

            (size <= (self.body.len() - HEADER_SIZE)).then(|| size)
        } else {
            None
        }
    }

    /// Gets immutable payload of atom.
    ///
    /// * _Return_ : If it is correct atom, returns its payload. otherwise returns `None`.
    #[inline]
    pub fn atom(&self) -> Option<&[u8]> {
        let size = self.get_atom_size()?;

        Some(unsafe {
            from_raw_parts(self.body.as_ptr().add(HEADER_SIZE), size)
        })
    }

    /// Gets mutable payload of atom.
    ///
    /// * _Return_ : If it is correct atom, returns its payload. otherwise returns `None`.
    #[inline]
    pub fn atom_mut(&mut self) -> Option<&mut [u8]> {
        let size = self.get_atom_size()?;

        Some(unsafe {
            from_raw_parts_mut(self.body.as_mut_ptr().add(HEADER_SIZE), size)
        })
    }

    #[inline]
    fn cons_size(&self) -> Option<usize> {
        let header = self.header()?;

        if header.is_cons() {
            let size = header.size();

            (size <= (self.body.len() - HEADER_SIZE)).then(|| size)
        } else {
            None
        }
    }

    /// Gets immutable car of cons.
    ///
    /// * _Return_ : If it is correct cons, returns its car. otherwise returns `None`.
    pub fn car(&self) -> Option<&ChobitSexpr> {
        let size = self.cons_size()?;

        Some(ChobitSexpr::new(unsafe {
            from_raw_parts(
                self.body.as_ptr().add(HEADER_SIZE),
                size
            )
        }))
    }

    /// Gets mutable car of cons.
    ///
    /// * _Return_ : If it is correct cons, returns its car. otherwise returns `None`.
    pub fn car_mut(&mut self) -> Option<&mut ChobitSexpr> {
        let size = self.cons_size()?;

        Some(ChobitSexpr::new_mut(unsafe {
            from_raw_parts_mut(
                self.body.as_mut_ptr().add(HEADER_SIZE),
                size
            )
        }))
    }

    /// Gets immutable cdr of cons.
    ///
    /// * _Return_ : If it is correct cons, returns its cdr. otherwise returns `None`.
    pub fn cdr(&self) -> Option<&ChobitSexpr> {
        let cdr_pos = self.cons_size()? + HEADER_SIZE;

        Some(ChobitSexpr::new(unsafe {
            from_raw_parts(
                self.body.as_ptr().add(cdr_pos),
                self.body.len() - cdr_pos
            )
        }))
    }

    /// Gets mutable cdr of cons.
    ///
    /// * _Return_ : If it is correct cons, returns its cdr. otherwise returns `None`.
    pub fn cdr_mut(&mut self) -> Option<&mut ChobitSexpr> {
        let cdr_pos = self.cons_size()? + HEADER_SIZE;

        Some(ChobitSexpr::new_mut(unsafe {
            from_raw_parts_mut(
                self.body.as_mut_ptr().add(cdr_pos),
                self.body.len() - cdr_pos
            )
        }))
    }
}

impl Deref for ChobitSexpr {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        &self.body
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
    /// * _Return_ : ChobitSexpr of Empty state.
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

impl ChobitSexprBuf<Empty> {
    /// Creates ChobitSexprBuf. Not allocated on heap memory yet.
    ///
    /// * _Return_ : Instance.
    #[inline]
    pub fn new() -> Self {
        Self {
            buffer: Vec::<u8>::new(),

            _marker: PhantomData::<Empty>
        }
    }

    /// Creates ChobitSexprBuf with memory allocation.
    ///
    /// * `capacity` : allocation size.
    /// * _Return_ : Instance.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::<u8>::with_capacity(capacity),

            _marker: PhantomData::<Empty>
        }
    }

    /// Drops and returns completed empty sexpr.
    ///
    /// * _Return_ : Completed sexpr.
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
    /// * _Return_ : ChobitSexprBuf that can push car.
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
    /// * _Return_ : ChobitSexprBuf that can push list item.
    #[inline]
    pub fn build_list(self) -> ChobitSexprBuf<List> {
        let Self {buffer, ..} = self;

        ChobitSexprBuf::<List> {
            buffer: buffer,

            _marker: PhantomData::<List>
        }
    }

    /// Drops and push atom and completes.
    ///
    /// * `value` : Payload of atom.
    /// * _Return_ : Completed sexpr.
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
    /// * _Return_ : Completed sexpr.
    pub fn push_nil(self) -> ChobitSexprBuf<Completed> {
        let Self {mut buffer, ..} = self;

        buffer.extend_from_slice(&SexprHeader::new_nil().to_le_bytes());

        ChobitSexprBuf::<Completed> {
            buffer: buffer,

            _marker: PhantomData::<Completed>
        }
    }
}

impl ChobitSexprBuf<Completed> {
    /// Borrows self as immutable sexpr.
    ///
    /// * _Return_ : Self as immutable ChobitSexpr.
    #[inline]
    pub fn as_sexpr(&self) -> &ChobitSexpr {
        ChobitSexpr::new(self.buffer.as_slice())
    }

    /// Borrows self as mutable sexpr.
    ///
    /// * _Return_ : Self as mutable ChobitSexpr.
    #[inline]
    pub fn as_mut_sexpr(&mut self) -> &mut ChobitSexpr {
        ChobitSexpr::new_mut(self.buffer.as_mut_slice())
    }

    /// Drops self and returns buffer.
    ///
    /// * _Return_ : buffer.
    #[inline]
    pub fn drop_buffer(self) -> Vec<u8> {
        self.buffer
    }
}

impl ChobitSexprBuf<Car> {
    /// Drops and push car and returns instance to be able to push cdr.
    ///
    /// * `sexpr` : sexpr contained on car.
    /// * _Return_ : ChobitSexprBuf that can push cdr.
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
    /// Drops and push car and completes.
    ///
    /// * `sexpr` : sexpr contained on car.
    /// * _Return_ : Completed sexpr.
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
    /// Drops and push list item and returns instance self.
    ///
    /// * `sexpr` : list item.
    /// * _Return_ : Instance that can push list item.
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

    /// Drops and push nil to cdr and completes.
    ///
    /// * _Return_ : Complete sexpr.
    pub fn finish(self) -> ChobitSexprBuf<Completed> {
        let Self {mut buffer, ..} = self;

        buffer.extend_from_slice(&SexprHeader::new_nil().to_le_bytes());

        ChobitSexprBuf::<Completed> {
            buffer: buffer,

            _marker: PhantomData::<Completed>
        }
    }

    /// Drops and push sexpr to cdr and completes.
    ///
    /// * `sexpr` : Last sexpr.
    /// * _Return_ : Complete sexpr.
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
