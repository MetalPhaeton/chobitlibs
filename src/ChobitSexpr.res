type sexprHeader = | @as(0) Nil | @as(1) Atom(int) | @as(2) Cons(int)
type chobitSexpr = Uint8Array.t
type sexprDataView = DataView.t
type rec chobitSexprBuf =
  | @as(0) Empty
  | @as(1) Completed(chobitSexpr)
  | @as(2) Car
  | @as(3) Cdr(chobitSexpr)
  | @as(4) List(array<chobitSexpr>)

let fromUint8Array: Uint8Array.t => chobitSexpr =
  (ary) => ary->TypedArray.sliceToEnd(~start = 0)

let toUint8Array: chobitSexpr => Uint8Array.t =
  (sexpr) => sexpr->TypedArray.sliceToEnd(~start = 0)

let fromBuffer: (ArrayBuffer.t, int, int) => chobitSexpr =
  (buf, offset, len) => Uint8Array.fromBufferWithRange(
    buf,
    ~byteOffset = offset,
    ~length = len
  )

let buffer: chobitSexpr => (ArrayBuffer.t, int, int) =
  (sexpr) => (
    sexpr->TypedArray.buffer,
    sexpr->TypedArray.byteOffset,
    sexpr->TypedArray.length
  )

@send external overwrite: (Uint8Array.t, Uint8Array.t, int) => unit = "set"

@send external getI8Core: (DataView.t, int, bool) => int = "getInt8"
@send external setI8Core: (DataView.t, int, int, bool) => unit = "setInt8"
@send external getU8Core: (DataView.t, int, bool) => int = "getUint8"
@send external setU8Core: (DataView.t, int, int, bool) => unit = "setUint8"
@send external getI16Core: (DataView.t, int, bool) => int = "getInt16"
@send external setI16Core: (DataView.t, int, int, bool) => unit = "setInt16"
@send external getU16Core: (DataView.t, int, bool) => int = "getUint16"
@send external setU16Core: (DataView.t, int, int, bool) => unit = "setUint16"
@send external getI32Core: (DataView.t, int, bool) => int = "getInt32"
@send external setI32Core: (DataView.t, int, int, bool) => unit = "setInt32"
@send external getU32Core: (DataView.t, int, bool) => int = "getUint32"
@send external setU32Core: (DataView.t, int, int, bool) => unit = "setUint32"
@send external getI64Core: (DataView.t, int, bool) => bigint = "getBigInt64"
@send external setI64Core: (DataView.t, int, bigint, bool) => unit =
  "setBigInt64"
@send external getU64Core: (DataView.t, int, bool) => bigint = "getBigUint64"
@send external setU64Core: (DataView.t, int, bigint, bool) => unit =
  "setBigUint64"
@send external getF32Core: (DataView.t, int, bool) => float = "getFloat32"
@send external setF32Core: (DataView.t, int, float, bool) => unit = "setFloat32"
@send external getF64Core: (DataView.t, int, bool) => float = "getFloat64"
@send external setF64Core: (DataView.t, int, float, bool) => unit = "setFloat64"

let consFlag: int = 0x80_00_00_00
let notConsFlag: int = 0x7f_ff_ff_ff
let headerSize: int = 4

let arrayToHeadDataView: Uint8Array.t => option<DataView.t> = (ary) => {
  let len = ary->TypedArray.byteLength

  if len >= 4 {
    Some(DataView.fromBufferWithRange(
      ary->TypedArray.buffer,
      ~byteOffset = ary->TypedArray.byteOffset,
      ~length = headerSize
    ))
  } else {
    None
  }
}

let dataViewToSexprHeader: DataView.t => sexprHeader = (view) => {
  let size = view->getU32Core(0, true)

  if size === 0 {
    Nil
  } else if land(size, notConsFlag) === size {
    Atom(size)
  } else {
    Cons(land(size, notConsFlag))
  }
}

let setSexprHeaderToDataView: (DataView.t, sexprHeader) => unit =
  (view, header) => switch header {
    | Nil => view->setU32Core(0, 0, true)
    | Atom(size) => view->setU32Core(0, land(size, notConsFlag), true)
    | Cons(size) =>
      view->setU32Core(0, lor(land(size, notConsFlag), consFlag), true)
  }

let getHeader: chobitSexpr => option<sexprHeader> =
  (sexpr) => sexpr->arrayToHeadDataView->Option.map(
    (view) => view->dataViewToSexprHeader
  )

let isNil: chobitSexpr => bool =
  (sexpr) => switch sexpr->getHeader {
    | None => false
    | Some(header) => switch header {
      | Nil => true
      | _ => false
    }
  }

let isAtom: chobitSexpr => bool =
  (sexpr) => switch sexpr->getHeader {
    | None => false
    | Some(header) => switch header {
      | Atom(_) => true
      | _ => false
    }
  }

let isCons: chobitSexpr => bool =
  (sexpr) => switch sexpr->getHeader {
    | None => false
    | Some(header) => switch header {
      | Cons(_) => true
      | _ => false
    }
  }

let getAtomLength: chobitSexpr => option<int> =
  (sexpr) => switch sexpr->getHeader {
    | None => None
    | Some(header) => switch header {
      | Atom(len) => Some(len)
      | _ => None
    }
  }

let getAtom: chobitSexpr => option<Uint8Array.t> =
  (sexpr) => switch sexpr->getHeader {
    | None => None
    | Some(header) => switch header {
      | Atom(size) => {
        let end = headerSize + size
        if sexpr->TypedArray.length <= end {
            Some(sexpr->TypedArray.subarray(
              ~start = headerSize,
              ~end = end
            ))
        } else {
          None
        }
      }
      | _ => None
    }
  }

let getDataView: chobitSexpr => option<sexprDataView> =
  (sexpr) => switch sexpr->getHeader {
    | None => None
    | Some(header) => switch header {
      | Atom(size) => {
        if sexpr->TypedArray.length <= (headerSize + size) {
          let offset = sexpr->TypedArray.byteOffset + headerSize

          Some(DataView.fromBufferWithRange(
            sexpr->TypedArray.buffer,
            ~byteOffset = offset,
            ~length = size
          ))
        } else {
          None
        }
      }
      | _ => None
    }
  }

let byteSize: sexprDataView => int = (view) => view->DataView.byteLength

let getCar: chobitSexpr => option<chobitSexpr> =
  (sexpr) => switch sexpr->getHeader {
    | None => None
    | Some(header) => switch header {
      | Cons(size) => Some(sexpr->TypedArray.subarray(
        ~start = headerSize,
        ~end = headerSize + size
      ))
      | _ => None
    }
  }

let getCdr: chobitSexpr => option<chobitSexpr> =
  (sexpr) => switch sexpr->getHeader {
    | None => None
    | Some(header) => switch header {
      | Cons(size) => Some(sexpr->TypedArray.subarrayToEnd(
        ~start = headerSize + size,
      ))
      | _ => None
    }
  }

let getCarCdr: chobitSexpr => option<(chobitSexpr, chobitSexpr)> =
  (sexpr) => switch sexpr->getHeader {
    | None => None
    | Some(header) => switch header {
      | Cons(size) => {
        let cdrOffset = headerSize + size

        Some((
          sexpr->TypedArray.subarray(
            ~start = headerSize,
            ~end = cdrOffset
          ),
          sexpr->TypedArray.subarrayToEnd(
            ~start = cdrOffset
          )
        ))
      }
      | _ => None
    }
  }

let forEach: (chobitSexpr, chobitSexpr => unit) => chobitSexpr =
  (sexpr, proc) => {
    let sexpr = ref(sexpr)
    let loop = ref(true)

    while loop.contents {
      switch sexpr.contents->getCarCdr {
        | None => {loop := false}
        | Some((car, cdr)) => {
          proc(car)
          sexpr := cdr
        }
      }
    }

    sexpr.contents
  }

let getI8: sexprDataView => option<int> =
  (view) => if (view->DataView.byteLength) == 1 {
    Some(view->getI8Core(0, true))
  } else {
    None
  }
let setI8: (sexprDataView, int) => bool =
  (view, val) => if (view->DataView.byteLength) == 1 {
    view->setI8Core(0, val, true)
    true
  } else {
    false
  }

let getU8: sexprDataView => option<int> =
  (view) => if (view->DataView.byteLength) == 1 {
    Some(view->getU8Core(0, true))
  } else {
    None
  }
let setU8: (sexprDataView, int) => bool =
  (view, val) => if (view->DataView.byteLength) == 1 {
    view->setU8Core(0, val, true)
    true
  } else {
    false
  }

let getI16: sexprDataView => option<int> =
  (view) => if (view->DataView.byteLength) == 2 {
    Some(view->getI16Core(0, true))
  } else {
    None
  }
let setI16: (sexprDataView, int) => bool =
  (view, val) => if (view->DataView.byteLength) == 2 {
    view->setI16Core(0, val, true)
    true
  } else {
    false
  }

let getU16: sexprDataView => option<int> =
  (view) => if (view->DataView.byteLength) == 2 {
    Some(view->getU16Core(0, true))
  } else {
    None
  }
let setU16: (sexprDataView, int) => bool =
  (view, val) => if (view->DataView.byteLength) == 2 {
    view->setU16Core(0, val, true)
    true
  } else {
    false
  }

let getI32: sexprDataView => option<int> =
  (view) => if (view->DataView.byteLength) == 4 {
    Some(view->getI32Core(0, true))
  } else {
    None
  }
let setI32: (sexprDataView, int) => bool =
  (view, val) => if (view->DataView.byteLength) == 4 {
    view->setI32Core(0, val, true)
    true
  } else {
    false
  }

let getU32: sexprDataView => option<int> =
  (view) => if (view->DataView.byteLength) == 4 {
    Some(view->getU32Core(0, true))
  } else {
    None
  }
let setU32: (sexprDataView, int) => bool =
  (view, val) => if (view->DataView.byteLength) == 4 {
    view->setU32Core(0, val, true)
    true
  } else {
    false
  }

let getI64: sexprDataView => option<bigint> =
  (view) => if (view->DataView.byteLength) == 8 {
    Some(view->getI64Core(0, true))
  } else {
    None
  }
let setI64: (sexprDataView, bigint) => bool =
  (view, val) => if (view->DataView.byteLength) == 8 {
    view->setI64Core(0, val, true)
    true
  } else {
    false
  }

let getU64: sexprDataView => option<bigint> =
  (view) => if (view->DataView.byteLength) == 8 {
    Some(view->getU64Core(0, true))
  } else {
    None
  }
let setU64: (sexprDataView, bigint) => bool =
  (view, val) => if (view->DataView.byteLength) == 8 {
    view->setU64Core(0, val, true)
    true
  } else {
    false
  }

let getF32: sexprDataView => option<float> =
  (view) => if (view->DataView.byteLength) == 4 {
    Some(view->getF32Core(0, true))
  } else {
    None
  }
let setF32: (sexprDataView, float) => bool =
  (view, val) => if (view->DataView.byteLength) == 4 {
    view->setF32Core(0, val, true)
    true
  } else {
    false
  }

let getF64: sexprDataView => option<float> =
  (view) => if (view->DataView.byteLength) == 8 {
    Some(view->getF64Core(0, true))
  } else {
    None
  }
let setF64: (sexprDataView, float) => bool =
  (view, val) => if (view->DataView.byteLength) == 8 {
    view->setF64Core(0, val, true)
    true
  } else {
    false
  }

let createChobitSexprBuf: unit => chobitSexprBuf = () => Empty

let isEmpty: chobitSexprBuf => bool =
  (buf) => switch buf {
    | Empty => true
    | _ => false
  }

let isCompleted: chobitSexprBuf => bool =
  (buf) => switch buf {
    | Completed(_) => true
    | _ => false
  }

let isBuildingCar: chobitSexprBuf => bool =
  (buf) => switch buf {
    | Car => true
    | _ => false
  }

let isBuildingCdr: chobitSexprBuf => bool =
  (buf) => switch buf {
    | Cdr(_) => true
    | _ => false
  }

let isBuildingList: chobitSexprBuf => bool =
  (buf) => switch buf {
    | List(_) => true
    | _ => false
  }

let getSexpr: chobitSexprBuf => option<chobitSexpr> =
  (buf) => switch buf {
    | Completed(sexpr) => Some(sexpr)
    | _ => None
  }

let pushSexpr: (chobitSexprBuf, chobitSexpr) => chobitSexprBuf =
  (buf, sexpr) => switch buf {
    | Empty => Completed(sexpr)
    | _ => buf
  }

let pushNil: chobitSexprBuf => chobitSexprBuf =
  (buf) => switch buf {
    | Empty => {
      let retBuf = ArrayBuffer.make(headerSize)
      let ret = Uint8Array.fromBuffer(retBuf)

      let view = DataView.fromBuffer(retBuf)

      view->setSexprHeaderToDataView(Nil)

      Completed(ret)
    }

    | _ => buf
  }

let pushAtom: (chobitSexprBuf, Uint8Array.t) => chobitSexprBuf =
  (buf, ary) => switch buf {
    | Empty => {
      let len = ary->TypedArray.length

      let retBuf = ArrayBuffer.make(headerSize + len)
      let ret = Uint8Array.fromBuffer(retBuf)

      let header = Atom(len)

      let view = DataView.fromBufferWithRange(
        retBuf,
        ~byteOffset = 0,
        ~length = headerSize
      )

      view->setSexprHeaderToDataView(header)
      ret->overwrite(ary, headerSize)

      Completed(ret)
    }

    | _ => buf
  }

let buildCons: chobitSexprBuf => chobitSexprBuf =
  (buf) => switch buf {
    | Empty => Car
    | _ => buf
  }

let pushCar: (chobitSexprBuf, chobitSexpr) => chobitSexprBuf =
  (buf, sexpr) => switch buf {
    | Car => Cdr(sexpr)
    | _ => buf
  }

let pushCdr: (chobitSexprBuf, chobitSexpr) => chobitSexprBuf =
  (buf, sexpr) => switch buf {
    | Cdr(car) => {
      let carLen = car->TypedArray.length
      let cdrLen = sexpr->TypedArray.length

      let retBuf = ArrayBuffer.make(headerSize + carLen + cdrLen)
      let ret = Uint8Array.fromBuffer(retBuf)

      let headerOffset = 0
      let carOffset = headerOffset + headerSize
      let cdrOffset = carOffset + carLen
      let cdrEnd = cdrOffset + cdrLen

      let headerPart = DataView.fromBufferWithRange(
        retBuf,
        ~byteOffset = headerOffset,
        ~length = headerSize
      )

      let carPart = ret->TypedArray.subarray(
        ~start = carOffset,
        ~end = cdrOffset
      )

      let cdrPart = ret->TypedArray.subarray(
        ~start = cdrOffset,
        ~end = cdrEnd
      )

      headerPart->setSexprHeaderToDataView(Cons(carLen))
      carPart->overwrite(car, 0)
      cdrPart->overwrite(sexpr, 0)

      Completed(ret)
    }

    | _ => buf
  }

let buildList: chobitSexprBuf => chobitSexprBuf =
  (buf) => switch buf {
    | Empty => List([])
    | _ => buf
  }

let pushItem: (chobitSexprBuf, chobitSexpr) => chobitSexprBuf =
  (buf, sexpr) => switch buf {
    | List(sexprAry) => {
      sexprAry->Array.push(sexpr)
      List(sexprAry)
    }

    | _ => buf
  }

let finishList: (chobitSexprBuf, ~lastSexpr: chobitSexpr=?) => chobitSexprBuf =
  (buf, ~lastSexpr: option<chobitSexpr>=?) => switch buf {
    | List(sexprAry) => {
      let len = ref(0)

      sexprAry->Array.forEach(
        (sexpr) => {
          len := len.contents + headerSize + sexpr->TypedArray.length
        }
      )

      let len = len.contents + switch lastSexpr {
        | Some(sexpr) => sexpr->TypedArray.length
        | None => headerSize
      }

      let retBuf = ArrayBuffer.make(len)
      let ret = Uint8Array.fromBuffer(retBuf)

      let offset = ref(ret->TypedArray.byteOffset)

      sexprAry->Array.forEach(
        (sexpr) => {
          let sexprLen = sexpr->TypedArray.length

          let headerPart = DataView.fromBufferWithRange(
            retBuf,
            ~byteOffset = offset.contents,
            ~length = headerSize
          )

          let carPart = Uint8Array.fromBufferWithRange(
            retBuf,
            ~byteOffset = offset.contents + headerSize,
            ~length = sexprLen
          )

          headerPart->setSexprHeaderToDataView(Cons(sexprLen))
          carPart->overwrite(sexpr, 0)

          offset := offset.contents + headerSize + sexprLen
        }
      )

      switch lastSexpr {
        | Some(last) => {
          let lastLen = last->TypedArray.length

          let cdrPart = Uint8Array.fromBufferWithRange(
            retBuf,
            ~byteOffset = offset.contents,
            ~length = lastLen
          )

          cdrPart->overwrite(last, 0)

          Completed(ret)
        }

        | None => {
          let headerPart = DataView.fromBufferWithRange(
            retBuf,
            ~byteOffset = offset.contents,
            ~length = headerSize
          )

          headerPart->setSexprHeaderToDataView(Nil)

          Completed(ret)
        }
      }
    }

    | _ => buf
  }

let pushI8: (chobitSexprBuf, int) => chobitSexprBuf =
  (buf, num) => switch buf {
    | Empty => {
      let len = 1

      let retBuf = ArrayBuffer.make(len + headerSize)
      let ret = Uint8Array.fromBuffer(retBuf)

      let header = Atom(len)
      let headerView = DataView.fromBufferWithRange(
        retBuf,
        ~byteOffset = ret->TypedArray.byteOffset,
        ~length = headerSize
      )

      let payloadView = DataView.fromBufferToEnd(
        retBuf,
        ~byteOffset = headerSize
      )

      headerView->setSexprHeaderToDataView(header)
      payloadView->setI8Core(0, num, true)

      Completed(ret)
    }

    | _ => buf
  }

let pushU8: (chobitSexprBuf, int) => chobitSexprBuf =
  (buf, num) => switch buf {
    | Empty => {
      let len = 1

      let retBuf = ArrayBuffer.make(len + headerSize)
      let ret = Uint8Array.fromBuffer(retBuf)

      let header = Atom(len)
      let headerView = DataView.fromBufferWithRange(
        retBuf,
        ~byteOffset = ret->TypedArray.byteOffset,
        ~length = headerSize
      )

      let payloadView = DataView.fromBufferToEnd(
        retBuf,
        ~byteOffset = headerSize
      )

      headerView->setSexprHeaderToDataView(header)
      payloadView->setU8Core(0, num, true)

      Completed(ret)
    }

    | _ => buf
  }

let pushI16: (chobitSexprBuf, int) => chobitSexprBuf =
  (buf, num) => switch buf {
    | Empty => {
      let len = 2

      let retBuf = ArrayBuffer.make(len + headerSize)
      let ret = Uint8Array.fromBuffer(retBuf)

      let header = Atom(len)
      let headerView = DataView.fromBufferWithRange(
        retBuf,
        ~byteOffset = ret->TypedArray.byteOffset,
        ~length = headerSize
      )

      let payloadView = DataView.fromBufferToEnd(
        retBuf,
        ~byteOffset = headerSize
      )

      headerView->setSexprHeaderToDataView(header)
      payloadView->setI16Core(0, num, true)

      Completed(ret)
    }

    | _ => buf
  }

let pushU16: (chobitSexprBuf, int) => chobitSexprBuf =
  (buf, num) => switch buf {
    | Empty => {
      let len = 2

      let retBuf = ArrayBuffer.make(len + headerSize)
      let ret = Uint8Array.fromBuffer(retBuf)

      let header = Atom(len)
      let headerView = DataView.fromBufferWithRange(
        retBuf,
        ~byteOffset = ret->TypedArray.byteOffset,
        ~length = headerSize
      )

      let payloadView = DataView.fromBufferToEnd(
        retBuf,
        ~byteOffset = headerSize
      )

      headerView->setSexprHeaderToDataView(header)
      payloadView->setU16Core(0, num, true)

      Completed(ret)
    }

    | _ => buf
  }

let pushI32: (chobitSexprBuf, int) => chobitSexprBuf =
  (buf, num) => switch buf {
    | Empty => {
      let len = 4

      let retBuf = ArrayBuffer.make(len + headerSize)
      let ret = Uint8Array.fromBuffer(retBuf)

      let header = Atom(len)
      let headerView = DataView.fromBufferWithRange(
        retBuf,
        ~byteOffset = ret->TypedArray.byteOffset,
        ~length = headerSize
      )

      let payloadView = DataView.fromBufferToEnd(
        retBuf,
        ~byteOffset = headerSize
      )

      headerView->setSexprHeaderToDataView(header)
      payloadView->setI32Core(0, num, true)

      Completed(ret)
    }

    | _ => buf
  }

let pushU32: (chobitSexprBuf, int) => chobitSexprBuf =
  (buf, num) => switch buf {
    | Empty => {
      let len = 4

      let retBuf = ArrayBuffer.make(len + headerSize)
      let ret = Uint8Array.fromBuffer(retBuf)

      let header = Atom(len)
      let headerView = DataView.fromBufferWithRange(
        retBuf,
        ~byteOffset = ret->TypedArray.byteOffset,
        ~length = headerSize
      )

      let payloadView = DataView.fromBufferToEnd(
        retBuf,
        ~byteOffset = headerSize
      )

      headerView->setSexprHeaderToDataView(header)
      payloadView->setU32Core(0, num, true)

      Completed(ret)
    }

    | _ => buf
  }

let pushI64: (chobitSexprBuf, bigint) => chobitSexprBuf =
  (buf, num) => switch buf {
    | Empty => {
      let len = 8

      let retBuf = ArrayBuffer.make(len + headerSize)
      let ret = Uint8Array.fromBuffer(retBuf)

      let header = Atom(len)
      let headerView = DataView.fromBufferWithRange(
        retBuf,
        ~byteOffset = ret->TypedArray.byteOffset,
        ~length = headerSize
      )

      let payloadView = DataView.fromBufferToEnd(
        retBuf,
        ~byteOffset = headerSize
      )

      headerView->setSexprHeaderToDataView(header)
      payloadView->setI64Core(0, num, true)

      Completed(ret)
    }

    | _ => buf
  }

let pushU64: (chobitSexprBuf, bigint) => chobitSexprBuf =
  (buf, num) => switch buf {
    | Empty => {
      let len = 8

      let retBuf = ArrayBuffer.make(len + headerSize)
      let ret = Uint8Array.fromBuffer(retBuf)

      let header = Atom(len)
      let headerView = DataView.fromBufferWithRange(
        retBuf,
        ~byteOffset = ret->TypedArray.byteOffset,
        ~length = headerSize
      )

      let payloadView = DataView.fromBufferToEnd(
        retBuf,
        ~byteOffset = headerSize
      )

      headerView->setSexprHeaderToDataView(header)
      payloadView->setU64Core(0, num, true)

      Completed(ret)
    }

    | _ => buf
  }

let pushF32: (chobitSexprBuf, float) => chobitSexprBuf =
  (buf, num) => switch buf {
    | Empty => {
      let len = 4

      let retBuf = ArrayBuffer.make(len + headerSize)
      let ret = Uint8Array.fromBuffer(retBuf)

      let header = Atom(len)
      let headerView = DataView.fromBufferWithRange(
        retBuf,
        ~byteOffset = ret->TypedArray.byteOffset,
        ~length = headerSize
      )

      let payloadView = DataView.fromBufferToEnd(
        retBuf,
        ~byteOffset = headerSize
      )

      headerView->setSexprHeaderToDataView(header)
      payloadView->setF32Core(0, num, true)

      Completed(ret)
    }

    | _ => buf
  }

let pushF64: (chobitSexprBuf, float) => chobitSexprBuf =
  (buf, num) => switch buf {
    | Empty => {
      let len = 8

      let retBuf = ArrayBuffer.make(len + headerSize)
      let ret = Uint8Array.fromBuffer(retBuf)

      let header = Atom(len)
      let headerView = DataView.fromBufferWithRange(
        retBuf,
        ~byteOffset = ret->TypedArray.byteOffset,
        ~length = headerSize
      )

      let payloadView = DataView.fromBufferToEnd(
        retBuf,
        ~byteOffset = headerSize
      )

      headerView->setSexprHeaderToDataView(header)
      payloadView->setF64Core(0, num, true)

      Completed(ret)
    }

    | _ => buf
  }
