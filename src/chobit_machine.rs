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

//! This module contains stack machine parts.  
//! You can build simple virtual machine with them.
//!
//! - [ChobitStack]: A stack to push and pop values.
//! - [ChobitCode]: A container for instructions of stack machine.
//! - [ChobitEnv]: Environment for stack machine.
//!
//! # Example 1
//!
//! Calculator for addition and multiplication
//!
//! ```ignore
//! extern crate chobitlibs;
//! extern crate alloc;
//! 
//! use chobitlibs::chobit_machine::*;
//! 
//! #[derive(Debug, Clone, PartialEq)]
//! enum Inst {
//!     Push(i32),
//!     Add,
//!     Mul
//! }
//! 
//! #[derive(Debug, Clone, PartialEq)]
//! enum Val {
//!     Int(i32)
//! }
//! 
//! fn eval(
//!     stack: &mut ChobitStack<Val>,
//!     code: &mut ChobitCode<Inst>
//! ) {
//!     match code.next() {
//!         Some(inst) => match *inst {
//!             // Push value.
//!             Inst::Push(val) => {
//!                 stack.push(Val::Int(val));
//!             },
//! 
//!             // Addition.
//!             // Pop val_1.
//!             // Pop val_2.
//!             // Push (val_1 + val_2).
//!             Inst::Add => {
//!                 let val_1 = match stack.pop().expect("No values.") {
//!                     Val::Int(val) => val
//!                 };
//! 
//!                 let val_2 = match stack.pop().expect("No values.") {
//!                     Val::Int(val) => val
//!                 };
//! 
//!                 stack.push(Val::Int(val_1 + val_2));
//!             }
//! 
//!             // Multiplication
//!             // Pop val_1.
//!             // Pop val_2.
//!             // Push (val_1 * val_2).
//!             Inst::Mul => {
//!                 let val_1 = match stack.pop().expect("No values.") {
//!                     Val::Int(val) => val
//!                 };
//! 
//!                 let val_2 = match stack.pop().expect("No values.") {
//!                     Val::Int(val) => val
//!                 };
//! 
//!                 stack.push(Val::Int(val_1 * val_2));
//!             }
//!         },
//! 
//!         None => {println!("Code is over.");}
//!     }
//! }
//! 
//! fn main() {
//!     let mut stack = ChobitStack::<Val>::new();
//!     let mut code = ChobitCode::<Inst>::new();
//! 
//!     // 2 + 3
//!     code.set_code(&[
//!         Inst::Push(2),
//!         Inst::Push(3),
//!         Inst::Add
//!     ]);
//! 
//!     eval(&mut stack, &mut code);  // Push(2)
//!     eval(&mut stack, &mut code);  // Push(3)
//!     eval(&mut stack, &mut code);  // Add
//! 
//!     assert_eq!(stack.pop(), Some(Val::Int(2 + 3)));
//! 
//!     // 2 * 3
//!     code.set_code(&[
//!         Inst::Push(2),
//!         Inst::Push(3),
//!         Inst::Mul
//!     ]);
//! 
//!     eval(&mut stack, &mut code);  // Push(2)
//!     eval(&mut stack, &mut code);  // Push(3)
//!     eval(&mut stack, &mut code);  // Mul
//! 
//!     assert_eq!(stack.pop(), Some(Val::Int(2 * 3)));
//! 
//!     // (2 + 3) * (4 + 5)
//!     code.set_code(&[
//!         Inst::Push(2),
//!         Inst::Push(3),
//!         Inst::Add,
//!         Inst::Push(4),
//!         Inst::Push(5),
//!         Inst::Add,
//!         Inst::Mul
//!     ]);
//! 
//!     eval(&mut stack, &mut code);  // Push(2)
//!     eval(&mut stack, &mut code);  // Push(3)
//!     eval(&mut stack, &mut code);  // Add
//!     assert_eq!(stack.top(), Some(&Val::Int(2 + 3)));
//! 
//!     eval(&mut stack, &mut code);  // Push(4)
//!     eval(&mut stack, &mut code);  // Push(5)
//!     eval(&mut stack, &mut code);  // Add
//!     assert_eq!(stack.top(), Some(&Val::Int(4 + 5)));
//! 
//!     eval(&mut stack, &mut code);  // Mul
//! 
//!     assert_eq!(stack.pop(), Some(Val::Int((2 + 3) * (4 + 5))));
//! }
//! ```
//!
//! # Example 2
//!
//! Define, get, set to environment.
//!
//! ```ignore
//! extern crate chobitlibs;
//! extern crate alloc;
//! 
//! use chobitlibs::chobit_machine::*;
//! 
//! #[derive(Debug, Clone, PartialEq)]
//! enum Inst {
//!     PushInt(i32),
//!     PushKey(u64),
//!     Define,
//!     Get,
//!     Set
//! }
//! 
//! #[derive(Debug, Clone, PartialEq)]
//! enum Val {
//!     Int(i32),
//!     Key(u64)
//! }
//! 
//! fn eval(
//!     stack: &mut ChobitStack<Val>,
//!     code: &mut ChobitCode<Inst>,
//!     env: &mut ChobitEnv<Val>
//! ) {
//!     match code.next() {
//!         Some(inst) => match *inst {
//!             // Push value.
//!             Inst::PushInt(val) => {
//!                 stack.push(Val::Int(val));
//!             },
//! 
//!             // Push key.
//!             Inst::PushKey(key) => {
//!                 stack.push(Val::Key(key));
//!             },
//! 
//!             // Define.
//!             Inst::Define => {
//!                 let key = match stack.pop().expect("No values.") {
//!                     Val::Key(key) => key,
//! 
//!                     _ => {panic!("Wrong type.")}
//!                 };
//! 
//!                 let val = stack.pop().expect("No values.");
//! 
//!                 env.define(key, val);
//!             },
//! 
//!             // Get.
//!             Inst::Get => {
//!                 let key = match stack.pop().expect("No values.") {
//!                     Val::Key(key) => key,
//! 
//!                     _ => {panic!("Wrong type.")}
//!                 };
//! 
//!                 let val = env.get(key).expect("No value in Env.").clone();
//! 
//!                 stack.push(val);
//!             },
//! 
//!             // Set.
//!             Inst::Set => {
//!                 let key = match stack.pop().expect("No values.") {
//!                     Val::Key(key) => key,
//! 
//!                     _ => {panic!("Wrong type.")}
//!                 };
//! 
//!                 let val = stack.pop().expect("No values.");
//! 
//!                 env.set(key, val).expect("No key in Env.");
//!             },
//!         },
//! 
//!         None => {println!("Code is over.");}
//!     }
//! }
//! 
//! fn main() {
//!     let mut stack = ChobitStack::<Val>::new();
//!     let mut code = ChobitCode::<Inst>::new();
//!     let mut env = ChobitEnv::<Val>::new();
//! 
//!     // Define: {key: 100, val: 2}
//!     code.set_code(&[
//!         Inst::PushInt(2),
//!         Inst::PushKey(100),
//!         Inst::Define
//!     ]);
//! 
//!     eval(&mut stack, &mut code, &mut env);  // PushInt(2)
//!     eval(&mut stack, &mut code, &mut env);  // PushKey(100)
//!     eval(&mut stack, &mut code, &mut env);  // Define
//! 
//!     assert_eq!(stack.pop(), None);
//! 
//!     // Get: {key: 100} => 2
//!     code.set_code(&[
//!         Inst::PushKey(100),
//!         Inst::Get
//!     ]);
//! 
//!     eval(&mut stack, &mut code, &mut env);  // PushKey(100)
//!     eval(&mut stack, &mut code, &mut env);  // Get
//! 
//!     assert_eq!(stack.pop(), Some(Val::Int(2)));
//! 
//!     // Set: {key: 100, val: 3}
//!     code.set_code(&[
//!         Inst::PushInt(3),
//!         Inst::PushKey(100),
//!         Inst::Set
//!     ]);
//! 
//!     eval(&mut stack, &mut code, &mut env);  // PushInt(3)
//!     eval(&mut stack, &mut code, &mut env);  // PushKey(100)
//!     eval(&mut stack, &mut code, &mut env);  // Set
//! 
//!     assert_eq!(stack.pop(), None);
//! 
//!     // Get: {key: 100} => 3
//!     code.set_code(&[
//!         Inst::PushKey(100),
//!         Inst::Get
//!     ]);
//! 
//!     eval(&mut stack, &mut code, &mut env);  // PushKey(100)
//!     eval(&mut stack, &mut code, &mut env);  // Get
//! 
//!     assert_eq!(stack.pop(), Some(Val::Int(3)));
//! }
//! ```
//!
//! # Example 3
//!
//! If, then, else.
//!
//! ```ignore
//! extern crate chobitlibs;
//! extern crate alloc;
//! 
//! use chobitlibs::chobit_machine::*;
//! 
//! #[derive(Debug, Clone, PartialEq)]
//! enum Inst {
//!     PushInt(i32),
//!     PushBool(bool),
//!     PushProc(Vec<Inst>),
//!     IfElse
//! }
//! 
//! #[derive(Debug, Clone, PartialEq)]
//! enum Val {
//!     Int(i32),
//!     Bool(bool),
//!     Proc(Vec<Inst>)
//! }
//! 
//! fn eval(
//!     stack: &mut ChobitStack<Val>,
//!     code: &mut ChobitCode<Inst>
//! ) {
//!     match code.next() {
//!         Some(inst) => match inst {
//!             // Push value.
//!             Inst::PushInt(val) => {
//!                 stack.push(Val::Int(*val));
//!             },
//! 
//!             // Push boolean value.
//!             Inst::PushBool(val) => {
//!                 stack.push(Val::Bool(*val));
//!             },
//! 
//!             // Push code.
//!             Inst::PushProc(val) => {
//!                 stack.push(Val::Proc(val.clone()));
//!             },
//! 
//!             // IfElse.
//!             Inst::IfElse => {
//!                 let else_ = match stack.pop().expect("No values.") {
//!                     Val::Proc(code) => code,
//! 
//!                     _ => {panic!("Wrong type.")}
//!                 };
//! 
//!                 let if_ = match stack.pop().expect("No values.") {
//!                     Val::Proc(code) => code,
//! 
//!                     _ => {panic!("Wrong type.")}
//!                 };
//! 
//!                 let cond = match stack.pop().expect("No values.") {
//!                     Val::Bool(val) => val,
//! 
//!                     _ => {panic!("Wrong type.")}
//!                 };
//! 
//!                 code.push_frame();
//!                 if cond {
//!                     code.set_code(&if_);
//!                 } else {
//!                     code.set_code(&else_);
//!                 }
//!             },
//!         },
//! 
//!         None => {panic!("Code is over.");}
//!     }
//! }
//! 
//! fn main() {
//!     let mut stack = ChobitStack::<Val>::new();
//!     let mut code = ChobitCode::<Inst>::new();
//! 
//!     let if_: Vec<Inst> = vec![Inst::PushInt(2)];  // if true => 2
//!     let else_: Vec<Inst> = vec![Inst::PushInt(3)];  // if false => 3
//! 
//!     // if true then 2 else 3 => 2
//!     code.set_code(&[
//!         Inst::PushBool(true),
//!         Inst::PushProc(if_.clone()),
//!         Inst::PushProc(else_.clone()),
//!         Inst::IfElse
//!     ]);
//! 
//!     eval(&mut stack, &mut code);  // PushBool(true)
//!     eval(&mut stack, &mut code);  // PushProc(if_)
//!     eval(&mut stack, &mut code);  // PushProc(else_)
//!     eval(&mut stack, &mut code);  // IfElse
//!     eval(&mut stack, &mut code);  // PushInt(2)
//! 
//!     assert_eq!(stack.top(), Some(&Val::Int(2)));
//! 
//!     code.pop_frame();
//! 
//!     // if false then 2 else 3 => 3
//!     code.set_code(&[
//!         Inst::PushBool(false),
//!         Inst::PushProc(if_.clone()),
//!         Inst::PushProc(else_.clone()),
//!         Inst::IfElse
//!     ]);
//! 
//!     eval(&mut stack, &mut code);  // PushBool(false)
//!     eval(&mut stack, &mut code);  // PushProc(if_)
//!     eval(&mut stack, &mut code);  // PushProc(else_)
//!     eval(&mut stack, &mut code);  // IfElse
//!     eval(&mut stack, &mut code);  // PushInt(3)
//! 
//!     assert_eq!(stack.pop(), Some(Val::Int(3)));
//! 
//!     code.pop_frame();
//! }
//! ```
//!
//! # Example 4
//!
//! Implementation of closure system.
//!
//! ```ignore
//! extern crate chobitlibs;
//! extern crate alloc;
//! 
//! use chobitlibs::chobit_machine::*;
//! 
//! #[derive(Debug, Clone, PartialEq)]
//! enum Inst {
//!     Push(Val),
//!     Lambda,
//!     Call,
//!     Define,
//!     Get
//! }
//! 
//! #[derive(Debug, Clone, PartialEq)]
//! enum Val {
//!     Int(i32),
//!     Key(u64),
//!     Proc(Vec<Inst>),
//!     Closure(Vec<(u64, Val)>, Vec<Inst>)
//! }
//! 
//! fn eval(
//!     stack: &mut ChobitStack<Val>,
//!     code: &mut ChobitCode<Inst>,
//!     env: &mut ChobitEnv<Val>
//! ) {
//!     match code.next() {
//!         Some(inst) => match inst {
//!             // Push value.
//!             Inst::Push(val) => {
//!                 stack.push(val.clone());
//!             },
//! 
//!             // Lambda.
//!             Inst::Lambda => {
//!                 let proc = match stack.pop().expect("No values.") {
//!                     Val::Proc(proc) => proc.clone(),
//! 
//!                     _ => {panic!("Wrong type.")}
//!                 };
//! 
//!                 // save environment.
//!                 let mut env_dump = Vec::<(u64, Val)>::new();
//!                 env.dump(&mut env_dump);
//! 
//!                 // store saved environment onto closure.
//!                 stack.push(Val::Closure(env_dump, proc));
//! 
//!             },
//! 
//!             // Call
//!             Inst::Call => {
//!                 let (env_dump, proc) = match stack.pop().expect("No values.") {
//!                     Val::Closure(env_dump, proc) => (env_dump, proc),
//! 
//!                     _ => {panic!("Wrong type.")}
//!                 };
//! 
//!                 code.push_frame();
//!                 code.set_code(&proc);  // load proc of closure.
//! 
//!                 env.push_frame();
//!                 env.store(&env_dump);  // load environment of closure
//!             },
//! 
//!             // Define.
//!             Inst::Define => {
//!                 let key = match stack.pop().expect("No values.") {
//!                     Val::Key(key) => key,
//! 
//!                     _ => {panic!("Wrong type.")}
//!                 };
//! 
//!                 let val = stack.pop().expect("No values.");
//! 
//!                 env.define(key, val);
//!             },
//! 
//!             // Get.
//!             Inst::Get => {
//!                 let key = match stack.pop().expect("No values.") {
//!                     Val::Key(key) => key,
//! 
//!                     _ => {panic!("Wrong type.")}
//!                 };
//! 
//!                 let val = env.get(key).expect("No value in Env.").clone();
//! 
//!                 stack.push(val);
//!             },
//!         },
//! 
//!         None => {
//!             code.pop_frame();  // purge proc of closure.
//!             env.pop_frame();  // purge environment of closure.
//!         }
//!     }
//! }
//! 
//! fn main() {
//!     let mut stack = ChobitStack::<Val>::new();
//!     let mut code = ChobitCode::<Inst>::new();
//!     let mut env = ChobitEnv::<Val>::new();
//! 
//!     // Closure: {key: 100, value: 30} => {key: 100, value: 20}
//!     let proc: Vec<Inst> = vec![
//!         Inst::Push(Val::Key(100)),
//!         Inst::Get,
//!         Inst::Push(Val::Int(20)),
//!         Inst::Push(Val::Key(100)),
//!         Inst::Define,  // {key: 100, value: 20}
//!         Inst::Push(Val::Key(100)),
//!         Inst::Get
//!     ];
//! 
//!     // Define: {key: 100, value: 30}
//!     code.set_code(&[
//!         Inst::Push(Val::Int(30)),
//!         Inst::Push(Val::Key(100)),
//!         Inst::Define,  // {key: 100, value: 30}
//!         Inst::Push(Val::Key(100)),
//!         Inst::Get
//!     ]);
//! 
//!     eval(&mut stack, &mut code, &mut env);  // Push(Val(30))
//!     eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
//!     eval(&mut stack, &mut code, &mut env);  // Define
//!     assert_eq!(stack.top(), None);
//! 
//!     eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
//!     eval(&mut stack, &mut code, &mut env);  // Get
//!     assert_eq!(stack.top(), Some(&Val::Int(30)));
//! 
//!     // Define closure. Env: {key: 100, value: 30}
//!     code.set_code(&[
//!         Inst::Push(Val::Proc(proc.clone())),
//!         Inst::Lambda,  // Save environment. {key: 100, value: 30}
//!         Inst::Push(Val::Key(200)),
//!         Inst::Define,
//!     ]);
//! 
//!     eval(&mut stack, &mut code, &mut env);  // Push(Proc)
//!     eval(&mut stack, &mut code, &mut env);  // Lambda
//!     eval(&mut stack, &mut code, &mut env);  // Push(Key(200))
//!     eval(&mut stack, &mut code, &mut env);  // Define
//! 
//!     // Define: {key: 100, value: 30} => {key: 100, value: 40}
//!     code.set_code(&[
//!         Inst::Push(Val::Int(40)),
//!         Inst::Push(Val::Key(100)),
//!         Inst::Define,  // {key: 100, value: 40}
//!         Inst::Push(Val::Key(100)),
//!         Inst::Get
//!     ]);
//! 
//!     eval(&mut stack, &mut code, &mut env);  // Push(Val(40))
//!     eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
//!     eval(&mut stack, &mut code, &mut env);  // Define
//!     eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
//!     eval(&mut stack, &mut code, &mut env);  // Get
//!     assert_eq!(stack.top(), Some(&Val::Int(40)));
//! 
//!     // Call closure: {key: 100, value: 30} => {key: 100, value: 20}
//!     code.set_code(&[
//!         Inst::Push(Val::Key(200)),
//!         Inst::Get,
//!         Inst::Call  // Store environment. {key: 100, value: 30}
//!     ]);
//! 
//!     eval(&mut stack, &mut code, &mut env);  // Push(Key(200))
//!     eval(&mut stack, &mut code, &mut env);  // Get
//!     eval(&mut stack, &mut code, &mut env);  // Call
//! 
//!     eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
//!     eval(&mut stack, &mut code, &mut env);  // Get
//!     assert_eq!(stack.top(), Some(&Val::Int(30)));
//! 
//!     eval(&mut stack, &mut code, &mut env);  // Push(Val(20))
//!     eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
//!     eval(&mut stack, &mut code, &mut env);  // Define
//!     eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
//!     eval(&mut stack, &mut code, &mut env);  // Get
//!     assert_eq!(stack.top(), Some(&Val::Int(20)));
//! 
//!     // pop_frame() => {key: 100, value: 40}
//!     eval(&mut stack, &mut code, &mut env);
//! 
//!     // After removed closure: {key: 100, value: 40}
//!     code.set_code(&[
//!         Inst::Push(Val::Key(100)),
//!         Inst::Get
//!     ]);
//! 
//!     eval(&mut stack, &mut code, &mut env);  // Push(Key(100))
//!     eval(&mut stack, &mut code, &mut env);  // Get
//!     assert_eq!(stack.top(), Some(&Val::Int(40)));
//! }
//! ```

use alloc::vec::Vec;
use core::fmt;

/// Error object for [ChobitStack].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChobitStackError {
    /// This occurs when the 2nd argument of [ChobitStack::load()] is wrong.
    WrongBp {bp: usize},

    /// This occurs when the 3rd argument of [ChobitStack::load()] is wrong.
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

/// Error object for [ChobitCode].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChobitCodeError {
    /// This occurs when the 2nd argument of [ChobitCode::load()] is wrong.
    WrongBp {bp: usize},

    /// This occurs when the 3rd argument of [ChobitCode::load()] is wrong.
    WrongIp {ip: usize},

    /// This occurs when the 4th argument of [ChobitCode::load()] is wrong.
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

/// Error object for [ChobitEnv].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChobitEnvError {
    /// This occurs when the 2nd argument of [ChobitEnv::load()] is wrong.
    WrongBp {bp: usize},

    /// This occurs when the 3rd argument of [ChobitEnv::load()] is wrong.
    WrongFrameStack {index: usize, bp: usize},

    /// This occors when the value named 'key' is not found in the environment.
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

/// A stack to push and pop values.
pub struct ChobitStack<V: Clone> {
    body: Vec<V>,
    bp: usize,
    frame_stack: Vec<usize>
}

impl<V: Clone> ChobitStack<V> {
    /// Generates a instance.
    ///
    /// - __Return__ : A instance.
    #[inline]
    pub fn new() -> Self {
        Self {
            body: Vec::<V>::new(),
            bp: 0,
            frame_stack: Vec::<usize>::new()
        }
    }

    /// Generates a instance with capacity.
    ///
    /// - `body_capacity` : Initial capacity how many values can be contained.
    /// - `frame_stack_capacity` : Initial capacity how many frames can be contained.
    /// - __Return__ : A instance.
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

    /// Generates a instance with initial state.
    ///
    /// - `body` : Variables.
    /// - `bp` : Base pointer for current frame.
    /// - `frame_stack` : Array of base pointers of parent frames.
    /// - __Return__ : A instance.
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

    /// Drops this object.
    ///
    /// - __Return__ : (Body, Base pointer, Parent frames)
    #[inline]
    pub fn drop(self) -> (Vec<V>, usize, Vec<usize>) {
        let Self {body, bp, frame_stack} = self;
        (body, bp, frame_stack)
    }

    /// Gets all values.
    ///
    /// - __Return__ : Body.
    #[inline]
    pub fn body(&self) -> &[V] {&self.body}

    /// Gets current base pointer.
    ///
    /// - __Return__ : Current base pointer.
    #[inline]
    pub fn bp(&self) -> usize {self.bp}

    /// Gets parent frames.
    ///
    /// - __Return__ : Parent frames.
    #[inline]
    pub fn frame_stack(&self) -> &[usize] {&self.frame_stack}

    /// Pushes a value.
    ///
    /// - `value` : A value to push.
    #[inline]
    pub fn push(&mut self, value: V) {
        self.body.push(value);
    }

    /// Pops a value.
    ///
    /// - __Return__ : If there are pushed values on the current frame, returns the top of values. Otherwise None.
    #[inline]
    pub fn pop(&mut self) -> Option<V> {
        (self.body.len() > self.bp).then(
            || self.body.pop().expect(
                "Error at chobit_machine::ChobitStack::pop()"
            )
        )
    }

    /// Refers a value.
    ///
    /// - __Return__ : If there are pushed values on the current frame, returns a reference of the top of values. Otherwise None.
    #[inline]
    pub fn top(&mut self) -> Option<&V> {
        (self.body.len() > self.bp).then(
            || self.body.last().expect(
                "Error at chobit_machine::ChobitStack::top()"
            )
        )
    }

    /// Gets current frame.
    ///
    /// - __Return__ : Current Frame.
    #[inline]
    pub fn current_frame(&self) -> &[V] {
        &self.body[self.bp..]
    }

    /// Pushes a frame.
    #[inline]
    pub fn push_frame(&mut self) {
        self.frame_stack.push(self.bp);
        self.bp = self.body.len();
    }

    /// Pops a frame. If there is no parent frames, it clears current frame.
    #[inline]
    pub fn pop_frame(&mut self) {
        match self.frame_stack.pop() {
            Some(new_bp) => {
                debug_assert!(self.bp <= self.body.len());
                unsafe {self.body.set_len(self.bp);}
                self.bp = new_bp;
            },

            None => {
                self.body.clear();
                self.bp = 0;
            }
        }
    }

    /// Merges current frame and the previous parent frame.
    #[inline]
    pub fn merge_frame(&mut self) {
        match self.frame_stack.pop() {
            Some(new_bp) => {
                self.bp = new_bp;
            },

            None => {}
        }
    }
}

/// A container for instructions of stack machine.
pub struct ChobitCode<I: Clone> {
    body: Vec<I>,

    bp: usize,
    bp_ip: usize,

    frame_stack: Vec<(usize, usize)>
}

impl<I: Clone> ChobitCode<I> {
    /// Generates a instance.
    ///
    /// __Return__ : A instance.
    #[inline]
    pub fn new() -> Self {
        Self {
            body: Vec::<I>::new(),
            bp: 0,
            bp_ip: 0,
            frame_stack: Vec::<(usize, usize)>::new()
        }
    }

    /// Generates a instance with capacity.
    ///
    /// - `body_capacity` : Initial capacity how many instructions can be contained.
    /// - `frame_stack_capacity` : Initial capacity how many frames can be contained.
    /// - __Return__ : A instance.
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


    /// Generates a instance with initial state.
    ///
    /// - `body` : Variables.
    /// - `bp` : Base pointer for current frame.
    /// - `ip` : Instruction pointer for current frame.
    /// - `frame_stack` : Array of base pointers and instruction pointers of parent frames.
    /// - __Return__ : A instance.
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


    /// Drops this object.
    ///
    /// - __Return__ : (Body, Base pointer, Instruction pointer, Parent frames)
    #[inline]
    pub fn drop(self) -> (Vec<I>, usize, usize, Vec<(usize, usize)>) {
        let Self {body, bp, bp_ip, frame_stack} = self;
        (body, bp, bp_ip - bp, frame_stack)
    }

    /// Gets all instructions.
    ///
    /// - __Return__ : Body.
    #[inline]
    pub fn body(&self) -> &[I] {&self.body}

    /// Gets current base pointer.
    ///
    /// - __Return__ : Current base pointer.
    #[inline]
    pub fn bp(&self) -> usize {self.bp}

    /// Gets current instruction pointer.
    ///
    /// - __Return__ : Current instruction pointer.
    #[inline]
    pub fn ip(&self) -> usize {self.bp_ip - self.bp}

    /// Gets current base pointer + instruction pointer.
    ///
    /// - __Return__ : Current base pointer + instruction pointer.
    #[inline]
    pub fn bp_ip(&self) -> usize {self.bp_ip}

    /// Gets parent frames.
    ///
    /// - __Return__ : Parent frames.
    #[inline]
    pub fn frame_stack(&self) -> &[(usize, usize)] {&self.frame_stack}

    /// Set instructions on the current frame.
    ///
    /// code: Instructions.
    #[inline]
    pub fn set_code(&mut self, code: &[I]) {
        debug_assert!(self.bp <= self.body.len());
        unsafe {self.body.set_len(self.bp);}

        self.body.extend_from_slice(code);
        self.bp_ip = self.bp;
    }

    /// Gets next instruction.
    ///
    /// - __Return__ : If there is next instruction, returns it. Otherwise None.
    #[inline]
    pub fn next(&mut self) -> Option<&I> {
        self.body.get(self.bp_ip).inspect(|_| {self.bp_ip += 1;})
    }

    /// Rewinds instruction pointer to the base pointer.
    #[inline]
    pub fn rewind(&mut self) {self.bp_ip = self.bp;}

    /// Sets an instruction pointer.  
    ///
    /// - `ip` : An instruction pointer.
    /// - __Return__ : If the instruction pointer is over the end of instructions, returns an error.
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

    /// Gets current frame.
    ///
    /// - __Return__ : Current Frame.
    #[inline]
    pub fn current_frame(&self) -> &[I] {
        &self.body[self.bp..]
    }

    /// Pushes a frame.
    #[inline]
    pub fn push_frame(&mut self) {
        self.frame_stack.push((self.bp, self.bp_ip));
        self.bp = self.body.len();
        self.bp_ip = self.bp;
    }

    /// Pops a frame. If there is no parent frames, it clears current frame.
    #[inline]
    pub fn pop_frame(&mut self) {
        match self.frame_stack.pop() {
            Some((new_bp, new_bp_ip)) => {
                debug_assert!(self.bp <= self.body.len());
                unsafe {self.body.set_len(self.bp);}

                self.bp = new_bp;
                self.bp_ip = new_bp_ip;
            },

            None => {
                self.body.clear();
                self.bp = 0;
                self.bp_ip = 0;
            }
        }
    }
}

/// Environment for stack machine.
pub struct ChobitEnv<V: Clone> {
    keys: Vec<u64>,
    values: Vec<V>,
    bp: usize,
    frame_stack: Vec<usize>
}

impl<V: Clone> ChobitEnv<V> {
    /// Generates a instance.
    ///
    /// - __Return__ : A instance.
    #[inline]
    pub fn new() -> Self {
        Self {
            keys: Vec::<u64>::new(),
            values: Vec::<V>::new(),
            bp: 0,
            frame_stack: Vec::<usize>::new()
        }
    }

    /// Generates a instance with capacity.
    ///
    /// - `body_capacity` : Initial capacity how many values can be contained.
    /// - `frame_stack_capacity` : Initial capacity how many frames can be contained.
    /// - __Return__ : A instance.
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

    /// Generates a instance with initial state.
    ///
    /// - `body` : Keys and values.
    /// - `bp` : Base pointer for current frame.
    /// - `frame_stack` : Array of base pointers of parent frames.
    /// - __Return__ : A instance.
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

    /// Drops this object.
    ///
    /// - __Return__ : (Body, Base pointer, Parent frames)
    #[inline]
    pub fn drop(self) -> (Vec<(u64, V)>, usize, Vec<usize>) {
        let Self {keys, values, bp, frame_stack} = self;

        let body: Vec<(u64, V)> =
            keys.into_iter().zip(values.into_iter()).collect();

        (body, bp, frame_stack)
    }

    /// Gets all keys.
    ///
    /// - __Return__ : All keys.
    #[inline]
    pub fn keys(&self) -> &[u64] {&self.keys}

    /// Gets all values.
    ///
    /// - __Return__ : All values.
    #[inline]
    pub fn values(&self) -> &[V] {&self.values}

    /// Gets current base pointer.
    ///
    /// - __Return__ : Current base pointer.
    #[inline]
    pub fn bp(&self) -> usize {self.bp}

    /// Gets parent frames.
    ///
    /// - __Return__ : Parent frames.
    #[inline]
    pub fn frame_stack(&self) -> &[usize] {&self.frame_stack}

    /// Defines a value.
    ///
    /// - `key` : An access key.
    /// - `value` : A value.
    #[inline]
    pub fn define(&mut self, key: u64, value: V) {
        debug_assert!(self.keys.len() == self.values.len());

        self.keys.push(key);
        self.values.push(value);
    }

    /// Sets a value.
    ///
    /// - `key` : An access key.
    /// - `value` : A value.
    /// - __Return__ : If the key is not exists, returns error. Otherwise returns previous value.
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

    /// Gets a value.
    ///
    /// - `key` : An access key.
    /// - __Return__ : If a value named by the key is exists, returns it. Otherwise returns None.
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

    /// Gets keys of the current frame.
    ///
    /// - __Return__ : Keys.
    #[inline]
    pub fn current_frame_keys(&self) -> &[u64] {
        &self.keys[self.bp..]
    }

    /// Gets values of the current frame.
    ///
    /// - __Return__ : Values.
    #[inline]
    pub fn current_frame_values(&self) -> &[V] {
        &self.values[self.bp..]
    }

    /// Pushes a frame.
    #[inline]
    pub fn push_frame(&mut self) {
        debug_assert!(self.keys.len() == self.values.len());

        self.frame_stack.push(self.bp);
        self.bp = self.keys.len();
    }

    /// Pops a frame. If there is no parent frames, it clears current frame.
    #[inline]
    pub fn pop_frame(&mut self) {
        debug_assert!(self.keys.len() == self.values.len());

        match self.frame_stack.pop() {
            Some(new_bp) => {
                debug_assert!(self.bp <= self.keys.len());
                debug_assert!(self.bp <= self.values.len());
                unsafe {self.keys.set_len(self.bp);}
                unsafe {self.values.set_len(self.bp);}

                self.bp = new_bp;
            },

            None => {
                self.keys.clear();
                self.values.clear();

                self.bp = 0;
            }
        }
    }

    /// Merges current frame and the previous parent frame.
    #[inline]
    pub fn merge_frame(&mut self) {
        debug_assert!(self.keys.len() == self.values.len());

        match self.frame_stack.pop() {
            Some(new_bp) => {
                self.bp = new_bp;
            },

            None => {}
        }
    }

    /// Stores keys and values onto the current frame.
    #[inline]
    pub fn store(&mut self, keys_values: &[(u64, V)]) {
        self.keys.reserve(keys_values.len());
        self.values.reserve(keys_values.len());

        keys_values.iter().for_each(|(key, value)| {
            self.define(*key, value.clone());
        });
    }

    /// Dumps all keys and values onto keys_values.
    ///
    /// - `keys_values` : Outputs of all keys and values.
    #[inline]
    pub fn dump(&self, keys_values: &mut Vec<(u64, V)>) {
        keys_values.clear();
        keys_values.reserve(self.keys.len());

        self.keys.iter().zip(self.values.iter()).for_each(|(key, value)| {
            keys_values.push((*key, value.clone()));
        });
    }
}
