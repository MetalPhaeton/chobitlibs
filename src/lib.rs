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

#![no_std]

#![doc = include_str!("../README.md")]

extern crate alloc;
#[cfg(test)] extern crate std;

pub mod chobit_map;
#[cfg(test)] mod chobit_map_tests;

pub mod chobit_hash;
#[cfg(test)] mod chobit_hash_tests;

pub mod chobit_rand;
#[cfg(test)] mod chobit_rand_tests;

pub mod chobit_ai;
#[cfg(test)] mod chobit_ai_tests;

pub mod chobit_sexpr;
#[cfg(test)] mod chobit_sexpr_tests;

pub mod chobit_module;
#[cfg(test)] mod chobit_module_tests;

#[cfg(test)]
mod tests;
