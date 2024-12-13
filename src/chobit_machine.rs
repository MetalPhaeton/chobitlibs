// Copyright (C) 2024 Hironori Ishibashi
//
// This work is free. You can redistribute it and/or modify it under the
// terms of the Do What The Fuck You Want To Public License, Version 2,
// as published by Sam Hocevar. See below for more details.
//
// --------------------------------------------------------------------
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use alloc::vec::Vec;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChobitStackError {
    WrongBp {bp: usize},
    WrongFrameStack {index: usize, bp: usize}
}

impl fmt::Display for ChobitStackError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, r#"{{"error":"ChobitStackError","kind":"#)?;

        match self {
            Self::WrongBp {bp} => {
                write!(formatter, r#""WrongBp","bp":{}"#, bp)?;
            },

            Self::WrongFrameStack {index, bp} => {
                write!(
                    formatter,
                    r#""WrongFrameStack","index":{},"bp":{}"#,
                    index,
                    bp
                )?;
            },
        }

        write!(formatter, "}}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChobitCodeError {
    WrongBp {bp: usize},
    WrongIp {ip: usize},
    WrongFrameStack {index: usize, bp: usize, bp_ip: usize}
}

impl fmt::Display for ChobitCodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, r#"{{"error":"ChobitCodeError","kind":"#)?;

        match self {
            Self::WrongBp {bp} => {
                write!(formatter, r#""WrongBp","bp":{}"#, bp)?;
            },

            Self::WrongIp {ip} => {
                write!(formatter, r#""WrongIp","ip":{}"#, ip)?;
            },

            Self::WrongFrameStack {index, bp, bp_ip} => {
                write!(
                    formatter,
                    r#""WrongFrameStack","index":{},"bp":{},"bp_ip":{}"#,
                    index,
                    bp,
                    bp_ip
                )?;
            },
        }

        write!(formatter, "}}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChobitEnvError {
    WrongBp {bp: usize},
    WrongFrameStack {index: usize, bp: usize},
    NotFound {key: u64}
}

impl fmt::Display for ChobitEnvError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, r#"{{"error":"ChobitEnvError","kind":"#)?;

        match self {
            Self::WrongBp {bp} => {
                write!(formatter, r#""WrongBp","bp":{}"#, bp)?;
            },

            Self::WrongFrameStack {index, bp} => {
                write!(
                    formatter,
                    r#""WrongFrameStack","index":{},"bp":{}"#,
                    index,
                    bp
                )?;
            },

            Self::NotFound {key} => {
                write!(formatter, r#""NotFound","key":{}"#, key)?;
            },
        }

        write!(formatter, "}}")
    }
}

pub struct ChobitStack<V: Clone> {
    body: Vec<V>,
    bp: usize,
    frame_stack: Vec<usize>
}

impl<V: Clone> ChobitStack<V> {
    #[inline]
    pub fn new() -> Self {
        Self {
            body: Vec::<V>::new(),
            bp: 0,
            frame_stack: Vec::<usize>::new()
        }
    }

    #[inline]
    pub fn with_capacity(
        body_capacity: usize,
        frame_stack_capacity: usize
    ) -> Self {
        Self {
            body: Vec::<V>::with_capacity(body_capacity),
            bp: 0,
            frame_stack: Vec::<usize>::with_capacity(frame_stack_capacity)
        }
    }

    #[inline]
    pub fn load(
        body: &[V],
        bp: usize,
        frame_stack: &[usize]
    ) -> Result<Self, ChobitStackError> {
        // check bp.
        if bp > body.len() {
            return Err(ChobitStackError::WrongBp {bp: bp});
        }

        // check frame_stack.
        {
            let mut iter = frame_stack.iter();
            match iter.next() {
                Some(prev_bp) => {
                    let mut prev_bp = *prev_bp;

                    let mut prev_index: usize = 0;
                    for bp in iter {
                        if prev_bp > *bp {
                            return Err(ChobitStackError::WrongFrameStack {
                                index: prev_index,
                                bp: prev_bp
                            });
                        }

                        prev_bp = *bp;
                        prev_index += 1;
                    }

                    if prev_bp > bp {
                        return Err(ChobitStackError::WrongFrameStack {
                            index: prev_index,
                            bp: prev_bp
                        });
                    }
                },

                None => {
                    if bp != 0 {
                        return Err(ChobitStackError::WrongBp {bp: bp});
                    }
                }
            }
        }

        Ok(Self {
            body: body.to_vec(),
            bp: bp,
            frame_stack: frame_stack.to_vec()
        })
    }

    #[inline]
    pub fn drop(self) -> (Vec<V>, usize, Vec<usize>) {
        let Self {body, bp, frame_stack} = self;
        (body, bp, frame_stack)
    }

    #[inline]
    pub fn body(&self) -> &[V] {&self.body}

    #[inline]
    pub fn bp(&self) -> usize {self.bp}

    #[inline]
    pub fn frame_stack(&self) -> &[usize] {&self.frame_stack}

    #[inline]
    pub fn push(&mut self, value: V) {
        self.body.push(value);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<V> {
        (self.body.len() > self.bp).then(
            || self.body.pop().expect(
                "Error at chobit_machine::ChobitStack::pop()"
            )
        )
    }

    #[inline]
    pub fn top(&mut self) -> Option<&V> {
        (self.body.len() > self.bp).then(
            || self.body.last().expect(
                "Error at chobit_machine::ChobitStack::top()"
            )
        )
    }

    #[inline]
    pub fn current_frame(&self) -> &[V] {
        &self.body[self.bp..]
    }

    #[inline]
    pub fn push_frame(&mut self) {
        self.frame_stack.push(self.bp);
        self.bp = self.body.len();
    }

    #[inline]
    pub fn pop_frame(&mut self) -> bool {
        if self.body.is_empty() {return false;}

        match self.frame_stack.pop() {
            Some(new_bp) => {
                debug_assert!(self.bp <= self.body.len());
                unsafe {self.body.set_len(self.bp);}
                self.bp = new_bp;

                true
            },

            None => {
                self.body.clear();
                self.bp = 0;

                true
            }
        }
    }
}

pub struct ChobitCode<I: Clone> {
    body: Vec<I>,

    bp: usize,
    bp_ip: usize,

    frame_stack: Vec<(usize, usize)>
}

impl<I: Clone> ChobitCode<I> {
    #[inline]
    pub fn new() -> Self {
        Self {
            body: Vec::<I>::new(),
            bp: 0,
            bp_ip: 0,
            frame_stack: Vec::<(usize, usize)>::new()
        }
    }

    #[inline]
    pub fn with_capacity(
        body_capacity: usize,
        frame_stack_capacity: usize
    ) -> Self {
        Self {
            body: Vec::<I>::with_capacity(body_capacity),
            bp: 0,
            bp_ip: 0,
            frame_stack: Vec::<(usize, usize)>::with_capacity(
                frame_stack_capacity
            )
        }
    }

    pub fn load(
        body: &[I],
        bp: usize,
        ip: usize,
        frame_stack: &[(usize, usize)]
    ) -> Result<Self, ChobitCodeError> {
        let bp_ip = ip + bp;

        // check bp.
        if bp > body.len() {
            return Err(ChobitCodeError::WrongBp {bp: bp});
        }

        // check bp_ip.
        if (bp_ip > body.len()) || (bp_ip < bp) {
            return Err(ChobitCodeError::WrongIp {ip: ip});
        }

        // check frame_stack.
        {
            let mut iter = frame_stack.iter();
            match iter.next() {
                Some((prev_bp, prev_bp_ip)) => {
                    let mut prev_bp = *prev_bp;
                    let mut prev_bp_ip = *prev_bp_ip;

                    let mut prev_index: usize = 0;
                    for (bp, bp_ip) in iter {
                        if prev_bp > *bp {
                            return Err(ChobitCodeError::WrongFrameStack {
                                index: prev_index,
                                bp: prev_bp,
                                bp_ip: prev_bp_ip
                            });
                        }

                        if (prev_bp_ip > *bp) || (prev_bp_ip < prev_bp) {
                            return Err(ChobitCodeError::WrongFrameStack {
                                index: prev_index,
                                bp: prev_bp,
                                bp_ip: prev_bp_ip
                            });
                        }

                        prev_bp = *bp;
                        prev_bp_ip = *bp_ip;
                        prev_index += 1;
                    }

                    if prev_bp > bp {
                        return Err(ChobitCodeError::WrongFrameStack {
                            index: prev_index,
                            bp: prev_bp,
                            bp_ip: prev_bp_ip
                        });
                    }

                    if (prev_bp_ip > bp) || (prev_bp_ip < prev_bp) {
                        return Err(ChobitCodeError::WrongFrameStack {
                            index: prev_index,
                            bp: prev_bp,
                            bp_ip: prev_bp_ip
                        });
                    }
                },

                None => {
                    if bp != 0 {
                        return Err(ChobitCodeError::WrongBp {bp: bp});
                    }
                }
            }
        }

        Ok(Self {
            body: body.to_vec(),
            bp: bp,
            bp_ip: bp_ip,
            frame_stack: frame_stack.to_vec()
        })
    }

    #[inline]
    pub fn drop(self) -> (Vec<I>, usize, usize, Vec<(usize, usize)>) {
        let Self {body, bp, bp_ip, frame_stack} = self;
        (body, bp, bp_ip - bp, frame_stack)
    }

    #[inline]
    pub fn body(&self) -> &[I] {&self.body}

    #[inline]
    pub fn bp(&self) -> usize {self.bp}

    #[inline]
    pub fn ip(&self) -> usize {self.bp_ip - self.bp}

    #[inline]
    pub fn bp_ip(&self) -> usize {self.bp_ip}

    #[inline]
    pub fn frame_stack(&self) -> &[(usize, usize)] {&self.frame_stack}

    #[inline]
    pub fn set_code(&mut self, code: &[I]) {
        debug_assert!(self.bp <= self.body.len());
        unsafe {self.body.set_len(self.bp);}

        self.body.extend_from_slice(code);
        self.bp_ip = self.bp;
    }

    #[inline]
    pub fn next(&mut self) -> Option<&I> {
        self.body.get(self.bp_ip).inspect(|_| {self.bp_ip += 1;})
    }

    #[inline]
    pub fn rewind(&mut self) {self.bp_ip = self.bp;}

    #[inline]
    pub fn set_ip(&mut self, ip: usize) -> Result<(), ChobitCodeError> {
        let bp_ip = ip + self.bp;

        if bp_ip > self.body.len() {
            Err(ChobitCodeError::WrongIp {ip: ip})
        } else {
            self.bp_ip = bp_ip;
            Ok(())
        }
    }

    #[inline]
    pub fn current_frame(&self) -> &[I] {
        &self.body[self.bp..]
    }

    #[inline]
    pub fn push_frame(&mut self) {
        self.frame_stack.push((self.bp, self.bp_ip));
        self.bp = self.body.len();
        self.bp_ip = self.bp;
    }

    #[inline]
    pub fn pop_frame(&mut self) -> bool {
        if self.body.is_empty() {return false;}

        match self.frame_stack.pop() {
            Some((new_bp, new_bp_ip)) => {
                debug_assert!(self.bp <= self.body.len());
                unsafe {self.body.set_len(self.bp);}

                self.bp = new_bp;
                self.bp_ip = new_bp_ip;

                true
            },

            None => {
                self.body.clear();
                self.bp = 0;
                self.bp_ip = 0;

                true
            }
        }
    }
}

pub struct ChobitEnv<V: Clone> {
    keys: Vec<u64>,
    values: Vec<V>,
    bp: usize,
    frame_stack: Vec<usize>
}

impl<V: Clone> ChobitEnv<V> {
    #[inline]
    pub fn new() -> Self {
        Self {
            keys: Vec::<u64>::new(),
            values: Vec::<V>::new(),
            bp: 0,
            frame_stack: Vec::<usize>::new()
        }
    }

    #[inline]
    pub fn with_capacity(
        body_capacity: usize,
        frame_stack_capacity: usize
    ) -> Self {
        Self {
            keys: Vec::<u64>::with_capacity(body_capacity),
            values: Vec::<V>::with_capacity(body_capacity),
            bp: 0,
            frame_stack: Vec::<usize>::with_capacity(frame_stack_capacity)
        }
    }

    #[inline]
    pub fn load(
        body: &[(u64, V)],
        bp: usize,
        frame_stack: &[usize]
    ) -> Result<Self, ChobitEnvError> {
        // check bp.
        if bp > body.len() {
            return Err(ChobitEnvError::WrongBp {bp: bp});
        }

        // check frame_stack.
        {
            let mut iter = frame_stack.iter();
            match iter.next() {
                Some(prev_bp) => {
                    let mut prev_bp = *prev_bp;

                    let mut prev_index: usize = 0;
                    for bp in iter {
                        if prev_bp > *bp {
                            return Err(ChobitEnvError::WrongFrameStack {
                                index: prev_index,
                                bp: prev_bp
                            });
                        }

                        prev_bp = *bp;
                        prev_index += 1;
                    }

                    if prev_bp > bp {
                        return Err(ChobitEnvError::WrongFrameStack {
                            index: prev_index,
                            bp: prev_bp
                        });
                    }
                },

                None => {
                    if bp != 0 {
                        return Err(ChobitEnvError::WrongBp {bp: bp});
                    }
                }
            }
        }

        let (keys, values): (Vec<u64>, Vec<V>) =
             body.to_vec().into_iter().unzip();

        Ok(Self {
            keys: keys,
            values: values,
            bp: bp,
            frame_stack: frame_stack.to_vec()
        })
    }

    #[inline]
    pub fn drop(self) -> (Vec<(u64, V)>, usize, Vec<usize>) {
        let Self {keys, values, bp, frame_stack} = self;

        let body: Vec<(u64, V)> =
            keys.into_iter().zip(values.into_iter()).collect();

        (body, bp, frame_stack)
    }

    #[inline]
    pub fn keys(&self) -> &[u64] {&self.keys}

    #[inline]
    pub fn values(&self) -> &[V] {&self.values}

    #[inline]
    pub fn bp(&self) -> usize {self.bp}

    #[inline]
    pub fn frame_stack(&self) -> &[usize] {&self.frame_stack}

    #[inline]
    pub fn define(&mut self, key: u64, value: V) {
        debug_assert!(self.keys.len() == self.values.len());

        self.keys.push(key);
        self.values.push(value);
    }

    #[inline]
    pub fn set(&mut self, key: u64, value: V) -> Result<V, ChobitEnvError> {
        debug_assert!(self.keys.len() == self.values.len());

        match self.keys.iter().rev().position(|key_2| *key_2 == key) {
            Some(i) => {
                let i = self.values.len() - 1 - i;

                debug_assert!(self.values.get(i).is_some());
                let value_2 = unsafe {self.values.get_unchecked_mut(i)};

                Ok(core::mem::replace(value_2, value))
            },

            None => Err(ChobitEnvError::NotFound {key: key})
        }
    }

    #[inline]
    pub fn get(&self, key: u64) -> Option<&V> {
        debug_assert!(self.keys.len() == self.values.len());

        self.keys.iter().rev().position(|key_2| *key_2 == key).map(
            |i| {
                let i = self.values.len() - 1 - i;

                debug_assert!(self.values.get(i).is_some());
                unsafe {self.values.get_unchecked(i)}
            }
        )
    }

    #[inline]
    pub fn current_frame_keys(&self) -> &[u64] {
        &self.keys[self.bp..]
    }

    #[inline]
    pub fn current_frame_values(&self) -> &[V] {
        &self.values[self.bp..]
    }

    #[inline]
    pub fn push_frame(&mut self) {
        debug_assert!(self.keys.len() == self.values.len());

        self.frame_stack.push(self.bp);
        self.bp = self.keys.len();
    }

    #[inline]
    pub fn pop_frame(&mut self) -> bool {
        debug_assert!(self.keys.len() == self.values.len());

        if self.keys.is_empty() {return false;}

        match self.frame_stack.pop() {
            Some(new_bp) => {
                debug_assert!(self.bp <= self.keys.len());
                debug_assert!(self.bp <= self.values.len());
                unsafe {self.keys.set_len(self.bp);}
                unsafe {self.values.set_len(self.bp);}

                self.bp = new_bp;

                true
            },

            None => {
                self.keys.clear();
                self.values.clear();

                self.bp = 0;

                true
            }
        }
    }

    #[inline]
    pub fn store(&mut self, keys_values: &[(u64, V)]) {
        self.push_frame();

        self.keys.reserve(keys_values.len());
        self.values.reserve(keys_values.len());

        keys_values.iter().for_each(|(key, value)| {
            self.define(*key, value.clone());
        });
    }

    #[inline]
    pub fn dump(&self, keys_values: &mut Vec<(u64, V)>) {
        keys_values.clear();
        keys_values.reserve(self.keys.len());

        self.keys.iter().zip(self.values.iter()).for_each(|(key, value)| {
            keys_values.push((*key, value.clone()));
        });
    }
}
