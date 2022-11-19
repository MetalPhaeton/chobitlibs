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

//! Hash table library.
//!
//! This library needs `alloc` crate.
//!
//! ```
//! extern crate alloc;
//! ```

use alloc::vec::Vec;

use core::{iter::Iterator, slice::Iter as SIter};

/// This is a so-called `HashMap`, but key is specialized by `u64`.
///
/// `ChobitMap` is the faster than Rust's `HashMap`.
pub struct ChobitMap<T> {
    key_table: Vec<Vec<u64>>,
    value_table: Vec<Vec<T>>,

    key_mask: u64
}

impl<T> ChobitMap<T> {
    /// Creates ChobitMap.
    ///
    /// * `table_size` : Key-value table size. this is repaired into power of 2.
    /// * _Return_ : Instance.
    ///
    /// ```
    /// use chobitlibs::chobit_map::ChobitMap;
    ///
    /// let map = ChobitMap::<i32>::new(200);
    /// assert_eq!(map.table_size(), 128);
    /// ```
    #[inline]
    pub fn new(table_size: usize) -> Self {
        let table_size = Self::check_table_size(table_size);

        Self {
            key_table: Self::init_key_table(table_size),
            value_table: Self::init_value_table(table_size),

            key_mask: Self::init_key_mask(table_size)
        }
    }

    fn check_table_size(table_size: usize) -> usize {
        const MASK_1: u64 = 0xffffffff00000000;
        const MASK_2: u64 = 0xffff0000ffff0000;
        const MASK_3: u64 = 0xff00ff00ff00ff00;
        const MASK_4: u64 = 0xf0f0f0f0f0f0f0f0;
        const MASK_5: u64 = 0xcccccccccccccccc;
        const MASK_6: u64 = 0xaaaaaaaaaaaaaaaa;

        macro_rules! core {
            ($variable:expr, $mask:expr) => {
                match $variable & $mask {
                    0u64 => $variable,
                    masked_variable => masked_variable
                }
            };
        }

        let size = table_size as u64;

        let size = core!(size, MASK_1);
        let size = core!(size, MASK_2);
        let size = core!(size, MASK_3);
        let size = core!(size, MASK_4);
        let size = core!(size, MASK_5);
        let size = core!(size, MASK_6);

        match size as usize{
            0 => 1usize,
            ret => ret
        }
    }

    fn init_key_table(table_size: usize) -> Vec<Vec<u64>> {
        let mut ret = Vec::<Vec<u64>>::with_capacity(table_size);

        for _ in 0..table_size {
            ret.push(Vec::<u64>::new());
        }

        ret
    }

    fn init_value_table(table_size: usize) -> Vec<Vec<T>> {
        let mut ret = Vec::<Vec<T>>::with_capacity(table_size);

        for _ in 0..table_size {
            ret.push(Vec::<T>::new());
        }

        ret
    }

    #[inline]
    fn init_key_mask(table_size: usize) -> u64 {
        (table_size as u64) - 1
    }

    /// Gets key-value table size.
    ///
    /// * _Return_ : Key-value table size.
    #[inline]
    pub fn table_size(&self) -> usize {self.key_table.len()}
    #[inline]
    fn get_index(&self, key: u64) -> Option<(usize, usize)> {
        let table_index = (key & self.key_mask) as usize;

        match self.key_table[table_index].binary_search(&key) {
            Ok(record_index) => Some((table_index, record_index)),

            Err(..) => None
        }
    }


    /// Gets a value by `key`.
    ///
    /// * `key` : A key of the value.
    /// * _Return_ : If `key` exists, returns the value. Otherwise, returns `None`.
    ///
    /// ```
    /// use chobitlibs::chobit_map::ChobitMap;
    ///
    /// let mut map = ChobitMap::<i32>::new(200);
    ///
    /// let key_1: u64 = 111;
    /// let value_1: i32 = 100;
    ///
    /// let key_2: u64 = 222;
    /// let value_2: i32 = 200;
    ///
    /// let key_3: u64 = 333;
    /// let value_3: i32 = 300;
    ///
    /// assert!(map.add(key_1, value_1).is_some());
    /// assert!(map.add(key_2, value_2).is_some());
    /// assert!(map.add(key_3, value_3).is_some());
    ///
    /// assert_eq!(*map.get(key_1).unwrap(), value_1);
    /// assert_eq!(*map.get(key_2).unwrap(), value_2);
    /// assert_eq!(*map.get(key_3).unwrap(), value_3);
    /// ```
    #[inline]
    pub fn get(&self, key: u64) -> Option<&T> {
        let (table_index, record_index) = self.get_index(key)?;

        Some(&self.value_table[table_index][record_index])
    }

    /// Gets a mutable value by `key`.
    ///
    /// * `key` : A key of the value.
    /// * _Return_ : If `key` exists, returns the mutable value. Otherwise, returns `None`.
    ///
    /// ```
    /// use chobitlibs::chobit_map::ChobitMap;
    ///
    /// let mut map = ChobitMap::<i32>::new(200);
    ///
    /// let key_1: u64 = 111;
    /// let value_1: i32 = 100;
    ///
    /// let key_2: u64 = 222;
    /// let value_2: i32 = 200;
    ///
    /// let key_3: u64 = 333;
    /// let value_3: i32 = 300;
    ///
    /// assert!(map.add(key_1, value_1).is_some());
    /// assert!(map.add(key_2, value_2).is_some());
    /// assert!(map.add(key_3, value_3).is_some());
    ///
    /// assert_eq!(*map.get(key_1).unwrap(), value_1);
    /// assert_eq!(*map.get(key_2).unwrap(), value_2);
    /// assert_eq!(*map.get(key_3).unwrap(), value_3);
    ///
    /// let value_1_2: i32 = 1000;
    /// let value_2_2: i32 = 2000;
    /// let value_3_2: i32 = 3000;
    ///
    /// *map.get_mut(key_1).unwrap() = value_1_2;
    /// *map.get_mut(key_2).unwrap() = value_2_2;
    /// *map.get_mut(key_3).unwrap() = value_3_2;
    ///
    /// assert_eq!(*map.get(key_1).unwrap(), value_1_2);
    /// assert_eq!(*map.get(key_2).unwrap(), value_2_2);
    /// assert_eq!(*map.get(key_3).unwrap(), value_3_2);
    /// ```
    #[inline]
    pub fn get_mut(&mut self, key: u64) -> Option<&mut T> {
        let (table_index, record_index) = self.get_index(key)?;

        Some(&mut self.value_table[table_index][record_index])
    }

    /// Adds a value.
    ///
    /// * `key` : A key of the value.
    /// * `value` : A value that you want to put into `ChobitMap`.
    /// * _Return_ : If the key is conflicted, returns `None`. Otherwise, returns `Some(())`.
    ///
    /// ```
    /// use chobitlibs::chobit_map::ChobitMap;
    ///
    /// let mut map = ChobitMap::<i32>::new(200);
    ///
    /// let key_1: u64 = 111;
    /// let value_1: i32 = 100;
    ///
    /// let key_2: u64 = 222;
    /// let value_2: i32 = 200;
    ///
    /// let key_3: u64 = 333;
    /// let value_3: i32 = 300;
    ///
    /// assert!(map.add(key_1, value_1).is_some());
    /// assert!(map.add(key_2, value_2).is_some());
    /// assert!(map.add(key_3, value_3).is_some());
    ///
    /// assert_eq!(*map.get(key_1).unwrap(), value_1);
    /// assert_eq!(*map.get(key_2).unwrap(), value_2);
    /// assert_eq!(*map.get(key_3).unwrap(), value_3);
    /// ```
    pub fn add(&mut self, key: u64, value: T) -> Option<()> {
        let table_index = (key & self.key_mask) as usize;

        let key_vec = &mut self.key_table[table_index];

        match key_vec.binary_search(&key) {
            Ok(..) => None,

            Err(record_index) => {
                key_vec.insert(record_index, key);
                self.value_table[table_index].insert(record_index, value);

                Some(())
            }
        }
    }

    /// Removes a value.
    ///
    /// * `key` : A key of the value.
    /// * _Return_ : If `key` exists, returns the value. Otherwise, returns `None`.
    ///
    /// ```
    /// use chobitlibs::chobit_map::ChobitMap;
    ///
    /// let mut map = ChobitMap::<i32>::new(200);
    ///
    /// let key_1: u64 = 111;
    /// let value_1: i32 = 100;
    ///
    /// let key_2: u64 = 222;
    /// let value_2: i32 = 200;
    ///
    /// let key_3: u64 = 333;
    /// let value_3: i32 = 300;
    ///
    /// assert!(map.add(key_1, value_1).is_some());
    /// assert!(map.add(key_2, value_2).is_some());
    /// assert!(map.add(key_3, value_3).is_some());
    ///
    /// assert_eq!(*map.get(key_1).unwrap(), value_1);
    /// assert_eq!(*map.get(key_2).unwrap(), value_2);
    /// assert_eq!(*map.get(key_3).unwrap(), value_3);
    ///
    /// assert_eq!(map.remove(key_1).unwrap(), value_1);
    /// assert_eq!(map.remove(key_2).unwrap(), value_2);
    /// assert_eq!(map.remove(key_3).unwrap(), value_3);
    ///
    /// assert!(map.get(key_1).is_none());
    /// assert!(map.get(key_2).is_none());
    /// assert!(map.get(key_3).is_none());
    /// ```
    pub fn remove(&mut self, key: u64) -> Option<T> {
        let table_index = (key & self.key_mask) as usize;

        let key_vec = &mut self.key_table[table_index];

        match key_vec.binary_search(&key) {
            Ok(record_index) => {
                key_vec.remove(record_index);
                Some(self.value_table[table_index].remove(record_index))
            },

            Err(..) => None
        }
    }

    /// Makes a iterator.
    ///
    /// * _Return_ : A iterator of `ChobitMap`.
    ///
    /// ```
    /// use chobitlibs::chobit_map::ChobitMap;
    ///
    /// let mut map = ChobitMap::<i32>::new(200);
    ///
    /// let key_1: u64 = 1;
    /// let value_1: i32 = 100;
    ///
    /// let key_2: u64 = 2;
    /// let value_2: i32 = 200;
    ///
    /// let key_3: u64 = 3;
    /// let value_3: i32 = 300;
    ///
    /// assert!(map.add(key_1, value_1).is_some());
    /// assert!(map.add(key_2, value_2).is_some());
    /// assert!(map.add(key_3, value_3).is_some());
    ///
    /// let mut iter = map.iter();
    ///
    /// assert_eq!(iter.next().unwrap(), (key_1, &value_1));
    /// assert_eq!(iter.next().unwrap(), (key_2, &value_2));
    /// assert_eq!(iter.next().unwrap(), (key_3, &value_3));
    /// assert!(iter.next().is_none());
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::<'_, T> {
            key_table_iter: self.key_table.iter(),
            value_table_iter: self.value_table.iter(),

            key_record_iter: None,
            value_record_iter: None
        }
    }
}

/// A iterator of `ChobitMap`.
pub struct Iter<'a, T> {
    key_table_iter: SIter<'a, Vec<u64>>,
    value_table_iter: SIter<'a, Vec<T>>,

    key_record_iter: Option<SIter<'a, u64>>,
    value_record_iter: Option<SIter<'a, T>>
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = (u64, &'a T);

    fn next(&mut self) -> Option<(u64, &'a T)> {
        match &mut self.key_record_iter {
            Some(key_record_iter) => match &key_record_iter.next() {
                Some(key) => Some((
                    **key,
                    self.value_record_iter.as_mut().unwrap().next().unwrap()
                )),

                None => {
                    self.key_record_iter = None;
                    self.value_record_iter = None;

                    self.next()
                }
            },

            None => {
                self.key_record_iter =
                    Some(self.key_table_iter.next()?.iter());
                self.value_record_iter =
                    Some(self.value_table_iter.next()?.iter());

                self.next()
            }
        }
    }
}
