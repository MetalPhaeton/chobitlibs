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

// ChobitSexpr ----------------------------------------------------------------

// S-Expression as byte string.
type chobitSexpr

// Creates ChobitSexpr from Uint8Array.
//
// - 1st arg: Source object.
// - Return: ChobitSexpr.
let fromUint8Array: Uint8Array.t => chobitSexpr

// ChobitSexpr as Uint8Array.
//
// - 1st arg: ChobitSexpr.
// - Return: Uint8Array.
let toUint8Array: chobitSexpr => Uint8Array.t

// Creates ChobitSexpr from Uint8Array.
//
// - 1st arg: ArrayBuffer.
// - 2nd arg: start of sexpr in buffer.
// - 3rd arg: end of sexpr in buffer.
// - Return: ChobitSexpr.
let fromBuffer: (ArrayBuffer.t, int, int) => chobitSexpr

// ChobitSexpr as Uint8Array.
//
// - 1st arg: ChobitSexpr
// - Return: (ArrayBuffer, start of sexpr, end of sexpr)
let buffer: chobitSexpr => (ArrayBuffer.t, int, int)

// Sexpr is Nil or not.
//
// - 1st arg: ChobitSexpr.
// - Return: Sexpr is Nil or not.
let isNil: chobitSexpr => bool

// Sexpr is Atom or not.
//
// - 1st arg: ChobitSexpr.
// - Return: Sexpr is Atom or not.
let isAtom: chobitSexpr => bool

// Sexpr is Cons or not.
//
// - 1st arg: ChobitSexpr.
// - Return: Sexpr is Cons or not.
let isCons: chobitSexpr => bool

// If sexpr is atom, returns length. Otherwise None.
//
// - 1st arg: ChobitSexpr.
// - Return: Length of atom.
let getAtomLength: chobitSexpr => option<int>

// If sexpr is atom, returns atom data. Otherwise None.
//
// - 1st arg: ChobitSexpr.
// - Return: Atom data.
let getAtom: chobitSexpr => option<Uint8Array.t>

// If sexpr is cons, returns car sexpr. Otherwise None.
//
// - 1st arg: ChobitSexpr.
// - Return: Car sexpr.
let getCar: chobitSexpr => option<chobitSexpr>

// If sexpr is cons, returns cdr sexpr. Otherwise None.
//
// - 1st arg: ChobitSexpr.
// - Return: Cdr sexpr.
let getCdr: chobitSexpr => option<chobitSexpr>

// If sexpr is cons, returns car and cdr sexpr. Otherwise None.
//
// - 1st arg: ChobitSexpr.
// - Return: (car, cdr).
let getCarCdr: chobitSexpr => option<(chobitSexpr, chobitSexpr)>

// If sexpr is list, executes callback function for eacy car sexpr.
//
// - 1st arg: ChobitSexpr.
// - 2nd arg: Callback function.
// - Return: Last cdr sexpr.
let forEach: (chobitSexpr, chobitSexpr => unit) => chobitSexpr

// SexprDataView --------------------------------------------------------------

// Atom data as data view.
type sexprDataView

// If sexpr is atom, returns atom data as data view. Otherwise None.
//
// - 1st arg: ChobitSexpr.
// - Return: Data view.
let getDataView: chobitSexpr => option<sexprDataView>

// Gets size of data view.
//
// - 1st arg: Sexpr data view
// - Return: Byte size of data view.
let byteSize: sexprDataView => int

// Gets int 8 value from data view.
//
// - 1st arg: Sexpr data view
// - Return: Int 8 value.
let getI8: sexprDataView => option<int> 

// Sets int 8 value to data view.
//
// - 1st arg: Sexpr data view
// - 2nd arg: Int 8 value.
// - Return: Success or not.
let setI8: (sexprDataView, int) => bool 

// Gets unsigned int 8 value from data view.
//
// - 1st arg: Sexpr data view
// - Return: Unsigned int 8 value.
let getU8: sexprDataView => option<int> 

// Sets unsigned int 8 value to data view.
//
// - 1st arg: Sexpr data view
// - 2nd arg: Unsigned int 8 value.
// - Return: Success or not.
let setU8: (sexprDataView, int) => bool 

// Gets int 16 value from data view.
//
// - 1st arg: Sexpr data view
// - Return: Int 16 value.
let getI16: sexprDataView => option<int> 

// Sets int 16 value to data view.
//
// - 1st arg: Sexpr data view
// - 2nd arg: Int 16 value.
// - Return: Success or not.
let setI16: (sexprDataView, int) => bool 

// Gets unsigned int 16 value from data view.
//
// - 1st arg: Sexpr data view
// - Return: Unsigned int 16 value.
let getU16: sexprDataView => option<int> 

// Sets unsigned int 16 value to data view.
//
// - 1st arg: Sexpr data view
// - 2nd arg: Unsigned int 16 value.
// - Return: Success or not.
let setU16: (sexprDataView, int) => bool 

// Gets int 32 value from data view.
//
// - 1st arg: Sexpr data view
// - Return: Int 32 value.
let getI32: sexprDataView => option<int> 

// Sets int 32 value to data view.
//
// - 1st arg: Sexpr data view
// - 2nd arg: Int 32 value.
// - Return: Success or not.
let setI32: (sexprDataView, int) => bool 

// Gets unsigned int 32 value from data view.
//
// - 1st arg: Sexpr data view
// - Return: Unsigned int 32 value.
let getU32: sexprDataView => option<int> 

// Sets unsigned int 32 value to data view.
//
// - 1st arg: Sexpr data view
// - 2nd arg: Unsigned int 32 value.
// - Return: Success or not.
let setU32: (sexprDataView, int) => bool 

// Gets int 64 value from data view.
//
// - 1st arg: Sexpr data view
// - Return: Int 64 value.
let getI64: sexprDataView => option<bigint> 

// Sets int 64 value to data view.
//
// - 1st arg: Sexpr data view
// - 2nd arg: Int 64 value.
// - Return: Success or not.
let setI64: (sexprDataView, bigint) => bool 

// Gets unsigned int 64 value from data view.
//
// - 1st arg: Sexpr data view
// - Return: Unsigned int 64 value.
let getU64: sexprDataView => option<bigint> 

// Sets unsigned int 64 value to data view.
//
// - 1st arg: Sexpr data view
// - 2nd arg: Unsigned int 64 value.
// - Return: Success or not.
let setU64: (sexprDataView, bigint) => bool 

// Gets float 32 value from data view.
//
// - 1st arg: Sexpr data view
// - Return: Float 32 value.
let getF32: sexprDataView => option<float> 

// Sets float 32 value to data view.
//
// - 1st arg: Sexpr data view
// - 2nd arg: Float 32 value.
// - Return: Success or not.
let setF32: (sexprDataView, float) => bool 

// Gets float 64 value from data view.
//
// - 1st arg: Sexpr data view
// - Return: Float 64 value.
let getF64: sexprDataView => option<float> 

// Sets float 64 value to data view.
//
// - 1st arg: Sexpr data view
// - 2nd arg: Float 64 value.
// - Return: Success or not.
let setF64: (sexprDataView, float) => bool 

// ChobitSexprBuf -------------------------------------------------------------

// ChobitSexpr builder.
//
// ChobitSexprBuf has 4 modes.
//
// - 'Empty': Just after ChobitSexprBuf has created.
// - 'Cmppleted': Sexpr has built.
// - `BuildingCar`: Wating to push Car sexpr.
// - `BuildingCdr`: Wating to push Cdr sexpr.
// - `BuildintList`: Waiting to push list item.
type chobitSexprBuf

// Creates ChobitSexprBuf.
//
// - Return: ChobitSexprBuf of 'Empty' mode.
let createChobitSexprBuf: unit => chobitSexprBuf

// ChobitSexprBuf is 'Empty' mode or not
//
// - 1st arg: ChobitSexprBuf
// - Return: Is 'Empty' mode or not.
let isEmpty: chobitSexprBuf => bool

// ChobitSexprBuf is 'Completed' mode or not
//
// - 1st arg: ChobitSexprBuf
// - Return: Is 'Completed' mode or not.
let isCompleted: chobitSexprBuf => bool

// ChobitSexprBuf is 'BuildingCar' mode or not
//
// - 1st arg: ChobitSexprBuf
// - Return: Is 'BuildingCar' mode or not.
let isBuildingCar: chobitSexprBuf => bool

// ChobitSexprBuf is 'BuildingCdr' mode or not
//
// - 1st arg: ChobitSexprBuf
// - Return: Is 'BuildingCdr' mode or not.
let isBuildingCdr: chobitSexprBuf => bool

// ChobitSexprBuf is 'BuildingList' mode or not
//
// - 1st arg: ChobitSexprBuf
// - Return: Is 'BuildingList' mode or not.
let isBuildingList: chobitSexprBuf => bool

// If 'Completed' mode, return sexpr. Otherwise None.
//
// - 1st arg: ChobitSexprBuf
// - Return: ChobitSexpr.
let getSexpr: chobitSexprBuf => option<chobitSexpr>

// If 'Empty' mode, pushes sexpr and returns self.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: Sexpr to push
// - Return: Self.
let pushSexpr: (chobitSexprBuf, chobitSexpr) => chobitSexprBuf

// If 'Empty' mode, pushes nil and returns self.
//
// - 1st arg: ChobitSexprBuf
// - Return: Self.
let pushNil: chobitSexprBuf => chobitSexprBuf

// If 'Empty' mode, pushes data and returns self.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: Data to push
// - Return: Self.
let pushAtom: (chobitSexprBuf, Uint8Array.t) => chobitSexprBuf

// If 'Empty' mode, changes mode into building cons.
//
// - 1st arg: ChobitSexprBuf
// - Return: Self.
let buildCons: chobitSexprBuf => chobitSexprBuf

// If 'BuildingCar' mode, pushes sexpr as car.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: Sexpr to push
// - Return: Self.
let pushCar: (chobitSexprBuf, chobitSexpr) => chobitSexprBuf

// If 'BuildingCdr' mode, pushes sexpr as cdr.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: Sexpr to push
// - Return: Self.
let pushCdr: (chobitSexprBuf, chobitSexpr) => chobitSexprBuf

// If 'Empty' mode, changes mode into building cons.
//
// - 1st arg: ChobitSexprBuf
// - Return: Self.
let buildList: chobitSexprBuf => chobitSexprBuf

// If 'BuildingList' mode, pushes sexpr.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: Sexpr to push
// - Return: Self.
let pushItem: (chobitSexprBuf, chobitSexpr) => chobitSexprBuf

// If 'BuildingList' mode, pushes sexpr as last cdr.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: Sexpr to push as last cdr. If None, then pushes nil.
// - Return: Self.
let finishList: (chobitSexprBuf, ~lastSexpr: chobitSexpr=?) => chobitSexprBuf

// If 'Empty' mode, pushes int 8 value and returns self.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: int 8 value to push
// - Return: Self.
let pushI8: (chobitSexprBuf, int) => chobitSexprBuf

// If 'Empty' mode, pushes unsigned int 8 value and returns self.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: unsigned int 8 value to push
// - Return: Self.
let pushU8: (chobitSexprBuf, int) => chobitSexprBuf

// If 'Empty' mode, pushes int 16 value and returns self.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: int 16 value to push
// - Return: Self.
let pushI16: (chobitSexprBuf, int) => chobitSexprBuf

// If 'Empty' mode, pushes unsigned int 16 value and returns self.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: unsigned int 16 value to push
// - Return: Self.
let pushU16: (chobitSexprBuf, int) => chobitSexprBuf

// If 'Empty' mode, pushes int 32 value and returns self.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: int 32 value to push
// - Return: Self.
let pushI32: (chobitSexprBuf, int) => chobitSexprBuf

// If 'Empty' mode, pushes unsigned int 32 value and returns self.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: unsigned int 32 value to push
// - Return: Self.
let pushU32: (chobitSexprBuf, int) => chobitSexprBuf

// If 'Empty' mode, pushes int 64 value and returns self.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: int 64 value to push
// - Return: Self.
let pushI64: (chobitSexprBuf, bigint) => chobitSexprBuf

// If 'Empty' mode, pushes unsigned int 64 value and returns self.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: unsigned int 64 value to push
// - Return: Self.
let pushU64: (chobitSexprBuf, bigint) => chobitSexprBuf

// If 'Empty' mode, pushes float 32 value and returns self.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: float 32 value to push
// - Return: Self.
let pushF32: (chobitSexprBuf, float) => chobitSexprBuf

// If 'Empty' mode, pushes float 64 value and returns self.
//
// - 1st arg: ChobitSexprBuf
// - 2nd arg: float 64 value to push
// - Return: Self.
let pushF64: (chobitSexprBuf, float) => chobitSexprBuf
