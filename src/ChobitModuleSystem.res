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

type worker
type message = (bigint, bigint, Uint8Array.t)
type event = {
  data: message
}
@new external createWorker: (string, {"type": string}) => worker = "Worker"
@send external postMessage: (
  worker,
  message,
  array<ArrayBuffer.t>
) => unit = "postMessage"
@send external terminate: (worker, unit) => unit = "terminate"
@set external onMessage: (worker, (event) => unit) => unit = "onmessage"

// system data ----------
let registry: Map.t<bigint, worker> = Map.make()
let selfId: bigint = 0n
// ----------

let tell: (bigint, bigint, Uint8Array.t) => unit =
  (senderId, receiverId, data) => {
    switch registry->Map.get(receiverId) {
      | None => ()
      | Some(receiver) => {
        receiver->postMessage(
          (senderId, receiverId, data),
          [data->TypedArray.buffer]
        )
      }
    }
  }

let addActor: (string, bigint, (bigint, Uint8Array.t) => unit) => unit =
  (actorJsUrl, id, onMessageHandler) => {
    switch registry->Map.get(id) {
      | None => ()
      | Some(worker) => {
        Console.warn(
          "{\"warn\":\"ActorAlreadyExists\",\"id\":"
            ++ id->BigInt.toString
            ++ "}"
        )
        worker->terminate()
      }
    }

    let actorJsUrl = actorJsUrl ++ "?id=" ++ id->BigInt.toString

    let worker = createWorker(actorJsUrl, {"type": "module"})

    worker->onMessage((event) => {
      let (senderId, receiverId, payload) = event.data

      if receiverId === selfId {
        onMessageHandler(senderId, payload)
      } else {
        tell(senderId, receiverId, payload)
      }
    })

    registry->Map.set(id, worker)
  }

let deleteActor: (bigint) => bool = (id) => {
  switch registry->Map.get(id) {
    | None => Console.warn(
      "{\"warn\":\"ActorNotFound\",\"id\":"
        ++ id->BigInt.toString
        ++ "}"
    )
    | Some(worker) => {
      worker->terminate()
    }
  }

  registry->Map.delete(id)
}

let sendMessage: (bigint, Uint8Array.t) => unit =
  (receiverId, data) => tell(selfId, receiverId, data)
  
let countActors: unit => int = () => registry->Map.size

let hasActor: bigint => bool = (id) => registry->Map.has(id)
