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

// Adds new chobit module actor, and runs on worker thread.  
// This main thread ID is 0, so don't use 0 for actor ID.  
// If an actor that same ID already exists, the old actor is terminated
// and registor new actor.
//
// - 1st arg: Path to actor script.
// - 2nd arg: Actor ID.
// - 3rd arg: Handler that the main thread received a message from the actor.
//     - 1st arg: The actor ID.
//     - 2nd arg: Message from the actor.
let addActor: (string, bigint, (bigint, Uint8Array.t) => unit) => unit

// Terminates and deletes chobit module actor.
//
// - 1st arg: Actor ID that is deleted.
// - Return: If the actor exists, returns true.
let deleteActor: bigint => bool

// Sends message to an actor.  
// (The message buffer is transfered to the actor, so after sending, the message becames empty on this main thread.)
//
// - 1st arg: Actor ID.
// - 2nd arg: Message.
let sendMessage: (bigint, Uint8Array.t) => unit

// Counts chobit module actors
//
// - Return: Number of actors.
let countActors: unit => int

// Gets whether the actor exists or not.
//
// - 1st arg: Actor ID.
let hasActor: bigint => bool
