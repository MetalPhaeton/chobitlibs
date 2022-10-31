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

//! Chobit
//! ======
//! 
//! Chobit is single file libraries.  
//! You can put each src file into your project.
//! 
//! All libraries are WTFPL License.
//! 
//! Libraries
//! ---------
//! 
//! * `chobit_map.rs` : Hash table.
//! * `hash.rs` : Hash functions.
//! * `chobit_rand.rs` : Random number generator.

extern crate alloc;
#[cfg(test)] extern crate std;

pub mod chobit_map;
#[cfg(test)] mod chobit_map_tests;

pub mod hash;
#[cfg(test)] mod hash_tests;

pub mod chobit_rand;
#[cfg(test)] mod chobit_rand_tests;

pub mod chobit_ai;
#[cfg(test)] mod chobit_ai_tests;

#[cfg(test)]
mod tests;
