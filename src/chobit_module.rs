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

//! WASM module library.
//!
//! See [ChobitModule] for detail.

use alloc::{boxed::Box, vec};

#[link(wasm_import_module = "env")]
extern {
    fn notify_input_buffer(offset: usize, size: usize);
    fn notify_output_buffer(offset: usize, size: usize);

    fn send(to: u64, length: usize);
}

/// An object that has all information and data of WASM.
///
/// # Exapmle
///
/// ```ignore
/// use chobitlibs::chobit_module::{ChobitModule, chobit_module};
///
/// struct MyObject {
///     pub value: i32
/// }
///
/// chobit_module! {
///     input_buffer_size = 16 * 1024;
///     output_buffer_size = 16 * 1024;
///
///     on_created() -> MyObject {
///         MyObject {
///             value: 100
///         }
///     }
///
///     on_received(module: &mut ChobitModule<MyObject>) {
///         module.send(
///             123,
///             format!("Hello {}", module.user_object().value).as_bytes()
///         );
///     }
/// }
/// ```
pub struct ChobitModule<T> {
    id: u64,

    input_buffer: Box<[u8]>,
    output_buffer: Box<[u8]>,

    recv_from: u64,
    recv_length: usize,

    user_object: T
}

impl<T> ChobitModule<T> {
    /// Gets module ID.
    ///
    /// ID is given from runtime when the module is initialized.
    ///
    /// - _Return_ : Module ID.
    #[inline]
    pub fn id(&self) -> u64 {self.id}

    /// Gets input buffer size.
    ///
    /// Input buffer is a buffer that is put input data from runtime.
    ///
    /// - _Return_ : A size of input buffer.
    #[inline]
    pub fn input_buffer_size(&self) -> usize {(*self.input_buffer).len()}

    /// Gets output buffer size.
    ///
    /// Output buffer is a buffer that the module puts output data.
    ///
    /// - _Return_ : A size of output buffer.
    #[inline]
    pub fn output_buffer_size(&self) -> usize {(*self.output_buffer).len()}

    /// Gets recieved data from other module.
    ///
    /// - _Return_ : (other_module_id, data)
    #[inline]
    pub fn recv_data(&self) -> (u64, &[u8]) {
        (self.recv_from, &(*self.input_buffer)[..self.recv_length])
    }

    fn copy_to_output_buffer(&mut self, data: &[u8]) -> usize {
        let data_len = data.len();

        (*self.output_buffer)[..data_len].copy_from_slice(data);

        data_len
    }

    /// Sends data to other module.
    ///
    /// - `to` : Other module ID.
    /// - `data` : Data that you want to send.
    #[inline]
    pub fn send(&mut self, to: u64, data: &[u8]) {
        unsafe {
            send(to, self.copy_to_output_buffer(data));
        }
    }

    /// Resizes input buffer.
    ///
    /// - `size` : New size of input buffer.
    pub fn resize_input_buffer(&mut self, size: usize) {
        let buffer = vec![0u8; size].into_boxed_slice();
        let offset = (*buffer).as_ptr() as usize;
        let size = (*buffer).len();

        unsafe {
            notify_input_buffer(offset, size);
        }

        self.input_buffer = buffer;
    }

    /// Resizes output buffer.
    ///
    /// - `size` : New size of output buffer.
    pub fn resize_output_buffer(&mut self, size: usize) {
        let buffer = vec![0u8; size].into_boxed_slice();
        let offset = (*buffer).as_ptr() as usize;
        let size = (*buffer).len();

        unsafe {
            notify_output_buffer(offset, size);
        }

        self.output_buffer = buffer;
    }

    /// Gets immutable user object.
    ///
    /// - _Return_ : Immutable user object.
    #[inline]
    pub fn user_object(&self) -> &T {&self.user_object}

    /// Gets mutable user object.
    ///
    /// - _Return_ : Mutable user object.
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

/// Defines WASM module. Defined in _chobit_module.rs_ .
///
/// ```ignore
/// use chobitlibs::chobit_module::{ChobitModule, chobit_module};
///
/// struct MyObject {
///     pub value: i32
/// }
///
/// chobit_module! {
///     input_buffer_size = 16 * 1024;  // Initial input buffer size.
///     output_buffer_size = 16 * 1024;  // Initial output buffer size.
///
///     // This is called when this module has created.
///     on_created() -> MyObject {
///         MyObject {
///             value: 100
///         }
///     }
///
///     // This is called when received data from other module.
///     on_received(module: &mut ChobitModule<MyObject>) {
///         module.send(
///             123,
///             format!("Hello {}", module.user_object().value).as_bytes()
///         );
///     }
/// }
/// ```
#[macro_export]
macro_rules! chobit_module {
    (
        input_buffer_size = $input_buffer_size:expr;
        output_buffer_size = $output_buffer_size:expr;

        on_created() -> $user_object_type:ty {
            $($code_1:tt)*
        }

        on_received($($args:tt)*) {
            $($code_2:tt)*
        }
    ) => {
static mut __MODULE: Option<ChobitModule<$user_object_type>> = None;

fn __on_created() -> $user_object_type {
    $($code_1)*
}

fn __on_received($($args)*) {
    $($code_2)*
}

#[allow(dead_code)]
#[no_mangle]
extern fn init(id: u64) {
    unsafe {
        __MODULE = Some(ChobitModule::<$user_object_type>::__new(
            id,
            $input_buffer_size,
            $output_buffer_size,
            __on_created()
        ));
    }
}

#[allow(dead_code)]
#[no_mangle]
extern fn recv(from: u64, length: usize) {
    match unsafe {__MODULE.as_mut()} {
        Some(module) => {
            module.__set_recv_info(from, length);

            __on_received(module);
        },

        None => {}
    }
}
    };
}
pub use chobit_module;
