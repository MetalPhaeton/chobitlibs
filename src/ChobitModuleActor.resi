// Copyright (C) 2025 Hironori Ishibashi
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

// Loads WebAssembly written by "chobitlibs::chobit_modlue.rs".
//
// - 1st arg: Path to chobit_module wasm file.
// - Return: Promise after wasm file is loaded.
let loadWasm: string => promise<unit>

// Sends message to another actor or main thread.  
// (The main thread's ID is 0)  
// (The message buffer is transfered to the actor, so after sending, the message becames empty on this main thread.)
//
// - 1st arg: Actor ID.
// - 2nd arg: Message.
let sendMessage: (bigint, Uint8Array.t) => unit

// Closes this thread.
let close: unit => unit

// Gets this ID.
//
// - Return: This worker ID
let id: unit => bigint
