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

//! Wasm module library.
//!

use alloc::{boxed::Box, vec};

#[link(wasm_import_module = "env")]
extern {
    fn notify_input_buffer(offset: usize, size: usize);
    fn notify_output_buffer(offset: usize, size: usize);

    fn send(to: u64, length: usize);
}

pub struct ChobitModule<T> {
    id: u64,

    input_buffer: Box<[u8]>,
    output_buffer: Box<[u8]>,

    recv_from: u64,
    recv_length: usize,

    user_object: T
}

impl<T> ChobitModule<T> {
    #[inline]
    pub fn id(&self) -> u64 {self.id}

    #[inline]
    pub fn input_buffer_size(&self) -> usize {(*self.input_buffer).len()}

    #[inline]
    pub fn output_buffer_size(&self) -> usize {(*self.output_buffer).len()}

    #[inline]
    pub fn recv_data(&self) -> (u64, &[u8]) {
        (self.recv_from, &(*self.input_buffer)[..self.recv_length])
    }

    pub fn send(&mut self, to: u64, data: &[u8]) {
        let data_len = data.len();

        (*self.output_buffer)[..data_len].copy_from_slice(data);

        unsafe {
            send(to, data_len);
        }
    }

    pub fn resize_input_buffer(&mut self, size: usize) {
        let buffer = vec![0u8; size].into_boxed_slice();
        let offset = (*buffer).as_ptr() as usize;
        let size = (*buffer).len();

        unsafe {
            notify_input_buffer(offset, size);
        }

        self.input_buffer = buffer;
    }

    pub fn resize_output_buffer(&mut self, size: usize) {
        let buffer = vec![0u8; size].into_boxed_slice();
        let offset = (*buffer).as_ptr() as usize;
        let size = (*buffer).len();

        unsafe {
            notify_output_buffer(offset, size);
        }

        self.output_buffer = buffer;
    }

    #[inline]
    pub fn user_object(&self) -> &T {&self.user_object}

    #[inline]
    pub fn user_object_mut(&mut self) -> &mut T {&mut self.user_object}

    #[doc(hidden)]
    pub fn __new(
        id: u64,
        input_buffer_size: usize,
        output_buffer_size: usize,
        user_object: T
    ) -> Self {
        let ret = Self {
            id: id,

            input_buffer:
                vec![0u8; input_buffer_size].into_boxed_slice(),
            output_buffer:
                vec![0u8; output_buffer_size].into_boxed_slice(),

            recv_from: 0,
            recv_length: 0,

            user_object: user_object
        };

        unsafe {
            notify_input_buffer(
                (*ret.input_buffer).as_ptr() as usize,
                (*ret.input_buffer).len(),
            );

            notify_output_buffer(
                (*ret.output_buffer).as_ptr() as usize,
                (*ret.output_buffer).len(),
            );

            ret
        }
    }

    #[doc(hidden)]
    pub fn __set_recv_info(&mut self, from: u64, length: usize) {
        self.recv_from = from;
        self.recv_length = length;
    }
}

#[macro_export]
macro_rules! chobit_module {
    (
        input_buffer_size = $input_buffer_size:expr;
        output_buffer_size = $output_buffer_size:expr;

        on_created = (): $user_object_type:ty => $proc_1:expr;

        on_received = ($module_name:ident) => $proc_2:expr;
    ) => {
fn on_created() -> $user_object_type {
    $proc_1
}

fn on_received($module_name: &mut ChobitModule<$user_object_type>) {
    $proc_2
}

#[doc(hidden)]
#[allow(dead_code)]
mod chobit_module_core {
    use super::*;

    static mut MODULE: Option<ChobitModule<$user_object_type>> = None;

    #[no_mangle]
    extern fn init(id: u64) {
        let user_object = on_created();

        unsafe {
            MODULE = Some(ChobitModule::__new(
                id,
                $input_buffer_size,
                $output_buffer_size,
                user_object
            ));
        }
    }

    #[no_mangle]
    extern fn recv(from: u64, length: usize) {
        match unsafe {MODULE.as_mut()} {
            Some(module) => {
                module.__set_recv_info(from, length);

                on_received(module);
            },

            None => {}
        }
    }
}
    };
}
pub use chobit_module;
