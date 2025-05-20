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

@val external globalThis: Dom.window = "globalThis"
@get external location: Dom.window => Dom.location = "location"
@get external search: Dom.location => string = "search"

type urlSearchParams
@new external createUrlSearchParams: string => urlSearchParams =
  "URLSearchParams"
@send external getValue: (urlSearchParams, string) => string = "get"
@val external stringToBigint: string => bigint = "BigInt"

let searchParams = createUrlSearchParams(globalThis->location->search)
let selfId = stringToBigint(searchParams->getValue("id"))

type importFuncs = {
    notify_input_buffer: (int, int) => unit,
    notify_output_buffer: (int, int) => unit,
    send: (bigint, int) => unit
}
type wasmImports = {
  env: importFuncs
}

type wasmMemory = {
  buffer: ArrayBuffer.t
}
type wasmExports = {
  memory: wasmMemory,
  init: bigint => unit,
  recv: (bigint, int) => unit
}
type wasmInstance = {
  exports: wasmExports
}
type resultObject = {
  instance: wasmInstance
}

type response
@val external fetch: string => response = "fetch"

@scope("WebAssembly") @val external instantiateStreaming:
(response, wasmImports) => promise<resultObject> = "instantiateStreaming"

type message = (bigint, bigint, Uint8Array.t)
@send external postMessage: (Dom.window, message) => unit = "postMessage"
@send external postMessage2: (
  Dom.window,
  message,
  array<ArrayBuffer.t>
) => unit = "postMessage"

type event = {
  data: message
}
@set external onMessage: (Dom.window, (event) => unit) => unit = "onmessage"

@send external overwrite: (Uint8Array.t, Uint8Array.t) => unit = "set"

let instance: ref<wasmInstance> = ref({
  exports: {
    memory: {buffer: ArrayBuffer.make(0)},
    init: (_) => (),
    recv: (_, _) => ()
  }
})

let inputBuffer = ref(Uint8Array.fromLength(0))
let outputBuffer = ref(Uint8Array.fromLength(0))

let loadWasm: string => promise<unit> = (url) => {
  open TypedArray
  let importObj: wasmImports = {
    env: {
      notify_input_buffer: (address, size) => {
        let mem = Uint8Array.fromBuffer(
          instance.contents.exports.memory.buffer
        )

        inputBuffer := mem->subarray(
          ~start = address,
          ~end = address + size
        )
      },

      notify_output_buffer: (address, size) => {
        let mem = Uint8Array.fromBuffer(
          instance.contents.exports.memory.buffer
        )

        outputBuffer := mem->subarray(
          ~start = address,
          ~end = address + size
        )
      },

      send: (receiverId, size) => {
        let payload = outputBuffer.contents->subarray(
          ~start = 0,
          ~end = size
        )

        globalThis->postMessage((selfId, receiverId, payload))
      }
    }
  }

  open Promise
  instantiateStreaming(fetch(url), importObj)->thenResolve((result) => {
    instance := result.instance

    globalThis->onMessage((event) => {
      let (senderId, _receiverId, payload) = event.data

      let len = if inputBuffer.contents->length < payload->length {
        inputBuffer.contents->length
      } else {
        payload->length
      }

      let payload = payload->subarray(
        ~start = 0,
        ~end = len
      )

      inputBuffer.contents->overwrite(payload)

      instance.contents.exports.recv(senderId, len)
    })

    instance.contents.exports.init(selfId)
  })
}

let sendMessage: (bigint, Uint8Array.t) => unit = (receiverId, data) => {
  globalThis->postMessage2(
    (selfId, receiverId, data),
    [data->TypedArray.buffer]
  )
}

@send external close: (Dom.window, unit) => unit = "close"
let close: unit => unit = () => globalThis->close()

let id: unit => bigint = () => selfId
