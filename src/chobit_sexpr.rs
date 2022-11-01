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

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct SexprHeader {
    body: u32
}

impl SexprHeader {
    #[inline]
    pub const fn from_bytes(bytes: [u8; HEADER_SIZE]) -> Self {
        Self {
            body: u32::from_le_bytes(bytes)
        }
    }

    #[inline]
    pub const fn to_bytes(&self) -> [u8; HEADER_SIZE] {
        self.body.to_le_bytes()
    }

    #[inline]
    pub fn from_slice(slice: &[u8]) -> Option<Self> {
        (slice.len() >= HEADER_SIZE).then(|| {
            Self {
                body: u32::from_le(unsafe {*(slice.as_ptr() as *const u32)})
            }
        })
    }

    #[inline]
    pub const fn new_nil() -> Self {
        Self {body: 0}
    }

    #[inline]
    const fn new_core(flag: u32, size: usize) -> Self {
        if size <= SIZE_MAX {
            Self {body: (flag & FLAG_MASK) | (size as u32)}
        } else {
            Self::new_nil()
        }
    }

    #[inline]
    pub const fn new_atom(size: usize) -> Self {
        Self::new_core(ATOM_FLAG, size)
    }

    #[inline]
    pub const fn new_cons(car_size: usize) -> Self {
        Self::new_core(CONS_FLAG, car_size)
    }

    #[inline]
    pub fn is_atom(&self) -> bool {
        (self.body & FLAG_MASK) == ATOM_FLAG
    }

    #[inline]
    pub fn is_cons(&self) -> bool {
        (self.body & FLAG_MASK) == CONS_FLAG
    }

    #[inline]
    pub fn size(&self) -> usize {
        (self.body & SIZE_MASK) as usize
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

#[derive(Debug, PartialEq)]
pub struct ChobitSexpr {
    body: [u8]
}

impl ChobitSexpr {
    #[inline]
    pub fn new<S: AsRef<[u8]> + ?Sized>(value: &S) -> &ChobitSexpr {
        unsafe {&*(value.as_ref() as *const [u8] as *const ChobitSexpr)}
    }

    #[inline]
    pub fn new_mut<S: AsMut<[u8]> + ?Sized>(value: &mut S) -> &mut ChobitSexpr {
        unsafe {&mut *(value.as_mut() as *mut [u8] as *mut ChobitSexpr)}
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {&self.body}

    #[inline]
    pub fn is_empty(&self) -> bool {self.body.is_empty()}

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

    #[inline]
    pub fn atom(&self) -> Option<&[u8]> {
        let size = self.get_atom_size()?;

        Some(unsafe {
            from_raw_parts(self.body.as_ptr().add(HEADER_SIZE), size)
        })
    }

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

    pub fn car(&self) -> Option<&ChobitSexpr> {
        let size = self.cons_size()?;

        Some(ChobitSexpr::new(unsafe {
            from_raw_parts(
                self.body.as_ptr().add(HEADER_SIZE),
                size
            )
        }))
    }

    pub fn car_mut(&mut self) -> Option<&mut ChobitSexpr> {
        let size = self.cons_size()?;

        Some(ChobitSexpr::new_mut(unsafe {
            from_raw_parts_mut(
                self.body.as_mut_ptr().add(HEADER_SIZE),
                size
            )
        }))
    }

    pub fn cdr(&self) -> Option<&ChobitSexpr> {
        let cdr_pos = self.cons_size()? + HEADER_SIZE;

        Some(ChobitSexpr::new(unsafe {
            from_raw_parts(
                self.body.as_ptr().add(cdr_pos),
                self.body.len() - cdr_pos
            )
        }))
    }

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

#[derive(Debug, PartialEq)]
pub enum Empty {}

#[derive(Debug, PartialEq)]
pub enum Completed {}

#[derive(Debug, PartialEq)]
pub enum Car {}

#[derive(Debug, PartialEq)]
pub enum Cdr {}

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

#[derive(Debug, Clone, PartialEq)]
pub struct ChobitSexprBuf<Mode = Completed>
where
    Mode: private::Sealed
{
    buffer: Vec<u8>,

    _marker: PhantomData<Mode>
}

impl<Mode> ChobitSexprBuf<Mode> where Mode: private::Sealed {
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
    #[inline]
    pub fn new() -> Self {
        Self {
            buffer: Vec::<u8>::new(),

            _marker: PhantomData::<Empty>
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::<u8>::with_capacity(capacity),

            _marker: PhantomData::<Empty>
        }
    }

    #[inline]
    pub fn empty_sexpr(self) -> ChobitSexprBuf<Completed> {
        let Self {buffer, ..} = self;

        ChobitSexprBuf::<Completed> {
            buffer: buffer,
            _marker: PhantomData::<Completed>
        }
    }

    #[inline]
    pub fn build_cons(self) -> ChobitSexprBuf<Car> {
        let Self {buffer, ..} = self;

        ChobitSexprBuf::<Car> {
            buffer: buffer,

            _marker: PhantomData::<Car>
        }
    }

    #[inline]
    pub fn build_list(self) -> ChobitSexprBuf<List> {
        let Self {buffer, ..} = self;

        ChobitSexprBuf::<List> {
            buffer: buffer,

            _marker: PhantomData::<List>
        }
    }

    pub fn push_atom(self, value: &[u8]) -> ChobitSexprBuf<Completed> {
        let Self {mut buffer, ..} = self;

        buffer.extend_from_slice(
            &SexprHeader::new_atom(value.len()).to_bytes()
        );

        buffer.extend_from_slice(value);

        ChobitSexprBuf::<Completed> {
            buffer: buffer,

            _marker: PhantomData::<Completed>
        }
    }

    pub fn push_nil(self) -> ChobitSexprBuf<Completed> {
        let Self {mut buffer, ..} = self;

        buffer.extend_from_slice(&SexprHeader::new_nil().to_bytes());

        ChobitSexprBuf::<Completed> {
            buffer: buffer,

            _marker: PhantomData::<Completed>
        }
    }
}

impl ChobitSexprBuf<Completed> {
    #[inline]
    pub fn as_sexpr(&self) -> &ChobitSexpr {
        ChobitSexpr::new(self.buffer.as_slice())
    }

    #[inline]
    pub fn as_mut_sexpr(&mut self) -> &mut ChobitSexpr {
        ChobitSexpr::new_mut(self.buffer.as_mut_slice())
    }

    #[inline]
    pub fn drop_buffer(self) -> Vec<u8> {
        self.buffer
    }
}

impl ChobitSexprBuf<Car> {
    pub fn push_car(self, sexpr: &ChobitSexpr) -> ChobitSexprBuf<Cdr> {
        let Self {mut buffer, ..} = self;

        let bytes = sexpr.as_bytes();

        buffer.extend_from_slice(
            &SexprHeader::new_cons(bytes.len()).to_bytes()
        );

        buffer.extend_from_slice(bytes);

        ChobitSexprBuf::<Cdr> {
            buffer: buffer,

            _marker: PhantomData::<Cdr>
        }
    }
}

impl ChobitSexprBuf<Cdr> {
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
    pub fn push_item(self, sexpr: &ChobitSexpr) -> ChobitSexprBuf<List> {
        let Self {mut buffer, ..} = self;

        let bytes = sexpr.as_bytes();

        buffer.extend_from_slice(
            &SexprHeader::new_cons(bytes.len()).to_bytes()
        );

        buffer.extend_from_slice(bytes);

        ChobitSexprBuf::<List> {
            buffer: buffer,

            _marker: PhantomData::<List>
        }
    }

    pub fn finish(self) -> ChobitSexprBuf<Completed> {
        let Self {mut buffer, ..} = self;

        buffer.extend_from_slice(&SexprHeader::new_nil().to_bytes());

        ChobitSexprBuf::<Completed> {
            buffer: buffer,

            _marker: PhantomData::<Completed>
        }
    }

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
