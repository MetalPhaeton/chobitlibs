// Copyright (C) 2022 Hironori Ishibashi
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

#![no_std]

#![doc = include_str!("../README.md")]

extern crate alloc;

pub mod chobit_map;

pub mod chobit_hash;

pub mod chobit_rand;

pub mod chobit_ai;

pub mod chobit_sexpr;

pub mod chobit_module;

pub mod chobit_complex;

pub mod chobit_playbook;

pub mod chobit_ani_value;

pub mod chobit_flow;

pub mod chobit_machine;
