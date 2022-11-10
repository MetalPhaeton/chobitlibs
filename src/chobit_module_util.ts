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

export function fnv1a64(bytes: Uint8Array): bigint {
    let ret: bigint = 0xcbf29ce484222325n;
    const PRIME: bigint = 0x00000100000001b3n as const;

    bytes.forEach((x) => {
        ret = (ret ^ BigInt(x)) & 0xffffffffffffffffn;
        ret = (ret * PRIME) & 0xffffffffffffffffn;
    });

    return ret;
}

const DATA_LENGTH_OFFSET: number = 16 as const;
const HEADER_OFFSET: number = 20 as const;

export class MessageEncoder {
    private _textEncoder: TextEncoder;
    private _buffer: ArrayBuffer;

    private _initID: bigint;
    private _recvID: bigint;
    private _sendID: bigint;
    private _wasmOKID: bigint;

    constructor(bufferSize: number) {
        this._textEncoder = new TextEncoder();
        this._buffer = new Uint8Array(bufferSize + HEADER_OFFSET).buffer;

        this._initID = this.toMsgID("init");
        this._recvID = this.toMsgID("recv");
        this._sendID = this.toMsgID("send");
        this._wasmOKID = this.toMsgID("wasm-ok");
    }

    get initID(): bigint {return this._initID;}
    get recvID(): bigint {return this._recvID;}
    get sendID(): bigint {return this._sendID;}
    get wasmOKID(): bigint {return this._wasmOKID;}

    resizeBuffer(bufferSize: number) {
        this._buffer = new Uint8Array(bufferSize + HEADER_OFFSET).buffer;
    }

    toMsgID(text: string): bigint {
        return fnv1a64(this._textEncoder.encode(text));
    }

    private _encodeMsg(
        msgID: bigint,
        id: bigint,
        data: Uint8Array
    ): ArrayBuffer | null {
        if ((data.length + HEADER_OFFSET) > this._buffer.byteLength) {
            return null;
        }

        const tmp1 = new BigUint64Array(this._buffer, 0, 2);
        tmp1[0] = msgID;
        tmp1[1] = id;

        const tmp2 = new Uint32Array(this._buffer, DATA_LENGTH_OFFSET, 1);
        tmp2[0] = data.length;

        const tmp3 = new Uint8Array(this._buffer, HEADER_OFFSET, data.length);

        tmp3.set(data);

        return this._buffer;
    }

    private _decodeMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] | null {
        if (msg.byteLength < HEADER_OFFSET) {return null;}

        const tmp1 = new BigUint64Array(msg, 0, 2);
        const tmp2 = new Uint32Array(msg, DATA_LENGTH_OFFSET, 1);

        if ((tmp2[0] + HEADER_OFFSET) > msg.byteLength) {return null;}

        const tmp3 = new Uint8Array(msg, HEADER_OFFSET, tmp2[0]);

        return [tmp1[0], tmp1[1], tmp3];
    }

    encodeInitMsg(id: bigint, data: Uint8Array): ArrayBuffer | null {
        return this._encodeMsg(this._initID, id, data);
    }

    decodeInitMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] | null {
        const ret = this._decodeMsg(msg);

        if (ret && (ret[0] == this._initID)) {
            return ret;
        } else {
            return null;
        }
    }

    encodeRecvMsg(from: bigint, data: Uint8Array): ArrayBuffer | null {
        return this._encodeMsg(this._recvID, from, data);
    }

    decodeRecvMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] | null {
        const ret = this._decodeMsg(msg);

        if (ret && (ret[0] == this._recvID)) {
            return ret;
        } else {
            return null;
        }
    }

    encodeSendMsg(to: bigint, data: Uint8Array): ArrayBuffer | null {
        return this._encodeMsg(this._sendID, to, data);
    }

    decodeSendMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] | null {
        const ret = this._decodeMsg(msg);

        if (ret && (ret[0] == this._sendID)) {
            return ret;
        } else {
            return null;
        }
    }

    encodeWASMOKMsg(id: bigint, data: Uint8Array): ArrayBuffer | null {
        return this._encodeMsg(this._wasmOKID, id, data);
    }

    decodeWASMOKMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] | null {
        const ret = this._decodeMsg(msg);

        if (ret && (ret[0] == this._wasmOKID)) {
            return ret;
        } else {
            return null;
        }
    }
}

//interface Exports {
//    memory: WebAssembly.Memory,
//
//    init: (id: bigint) => void,
//    recv: (from: bigint, length: number) => void
//}
//
//export class ChobitWASM {
//    private _exports: Exports | null;
//
//    private _inputBufferInfo: [number, number];
//    private _outputBufferInfo: [number, number];
//
//    constructor() {
//        this._exports = null;
//
//        this._inputBufferInfo = [0, 0];
//        this._outputBufferInfo = [0, 0];
//    }
//
//    isBuilt(): boolean {
//        return this._exports != null;
//    }
//
//    genWASM(
//        url: URL,
//        id: bigint,
//        imports: any
//    ): Promise<void> | null {
//        return WebAssembly.instantiateStreaming(
//            fetch(url),
//            imports
//        ).then((obj) => {
//            this._exports = obj.instance.exports as unknown as Exports;
//
//            this._exports.init(id);
//        });
//    }
//
//    genDefaultImports(
//        outputHandler: (to: bigint, data: Uint8Array) => void
//    ): any {
//        return {
//            env: {
//                notify_input_buffer: (offset: number, size: number) => {
//                    this._inputBufferInfo = [offset, size];
//                },
//
//                notify_output_buffer: (offset: number, size: number) => {
//                    this._outputBufferInfo = [offset, size];
//                },
//
//                send: (to: bigint, length: number) => {
//                    if (length > this._outputBufferInfo[1]) {return;}
//
//                    if (this._exports) {
//                        const data = new Uint8Array(
//                            this._exports.memory.buffer,
//                            this._outputBufferInfo[0],
//                            length
//                        );
//
//                        outputHandler(to, data);
//                    }
//                }
//            }
//        };
//    }
//
//    input(from: bigint, data: Uint8Array) {
//        if (data.length > this._inputBufferInfo[1]) {return;}
//
//        if (this._exports) {
//            const inputBuffer = new Uint8Array(
//                this._exports.memory.buffer,
//                this._inputBufferInfo[0],
//                this._inputBufferInfo[1]
//            );
//
//            inputBuffer.set(data);
//
//            this._exports.recv(from, data.length);
//        }
//    }
//}
//
//class ChobitWorkerChannel {
//    private _messageEncoder: MessageEncoder;
//    private _worker: Worker;
//
//    private _wasmID: bigint;
//
//    constructor(
//        workerURL: URL,
//        wasmID: bigint,
//        wasmURL: URL,
//        recvHandler: (from: bigint, data: Uint8Array) => void
//    ) {
//        this._messageEncoder = new MessageEncoder();
//        this._wasmID = wasmID;
//
//        this._worker =
//            this._initWorker(workerURL, wasmID, wasmURL, recvHandler);
//    }
//
//    get wasmID(): bigint {return this._wasmID;}
//
//    private _initWorker(
//        workerURL: URL,
//        wasmID: bigint,
//        wasmURL: URL,
//        recvHandler: (from: bigint, data: Uint8Array) => void
//    ): Worker {
//        const ret = new Worker(workerURL, {type: "module"});
//
//        ret.onmessage = (msg) => {
//            const decodedMsg = this._messageEncoder.decodeSendMsg(
//                msg.data as unknown as ArrayBuffer
//            );
//
//            if (decodedMsg) {
//                recvHandler(decodedMsg[1], decodedMsg[2]);
//            }
//        };
//
//        const msgBuffer = this._messageEncoder.encodeInitMsg(
//            wasmID,
//            new TextEncoder().encode(wasmURL.href)
//        );
//
//        ret.postMessage(msgBuffer, [msgBuffer]);
//
//        return ret;
//    }
//
//    sendMsg(from: bigint, data: Uint8Array) {
//        const msg = this._messageEncoder.encodeSendMsg(from, data);
//        this._worker.postMessage(msg, [msg]);
//    }
//}
//
//class ChobitWorker {
//    private _global: Worker;
//    private _messageEncoder: MessageEncoder;
//    private _wasm: ChobitWASM;
//
//    constructor() {
//        this._global = globalThis as unknown as Worker;
//
//        this._messageEncoder = new MessageEncoder();
//
//        this._wasm = new ChobitWASM();
//
//        this._global.onmessage = (msg) => {
//            this._handleMsg(msg.data as unknown as ArrayBuffer);
//        };
//    }
//
//    private _handleMsg(msg: ArrayBuffer) {
//        if (this._wasm.isBuilt()) {
//            this._handleSendMsg(msg);
//        } else {
//            this._handleInitMsg(msg);
//        }
//    }
//
//    private _handleInitMsg(msg: ArrayBuffer) {
//        const decodedMsg = this._messageEncoder.decodeInitMsg(msg);
//
//        if (decodedMsg) {
//            const id = decodedMsg[1];
//
//            const imports = this._wasm.genDefaultImports(
//                this._genOutputHandler()
//            );
//
//            const promise = this._wasm.genWASM(
//                new URL(new TextDecoder().decode(decodedMsg[2])),
//                id,
//                imports
//            );
//
//            if (promise) {
//                promise.then(() => {
//                    const msg = this._messageEncoder.encodeWASMOKMsg(
//                        id,
//                        new Uint8Array(0)
//                    );
//
//                    this._global.postMessage(msg, [msg]);
//                })
//            }
//        }
//    }
//
//    private _genOutputHandler(): (to: bigint, data: Uint8Array) => void {
//        return (to: bigint, data: Uint8Array) => {
//            const msg = this._messageEncoder.encodeSendMsg(to, data);
//
//            this._global.postMessage(msg, [msg]);
//        };
//    }
//
//    private _handleSendMsg(msg: ArrayBuffer) {
//        const decodedMsg = this._messageEncoder.decodeSendMsg(msg);
//
//        if (decodedMsg) {
//            this._wasm.input(decodedMsg[1], decodedMsg[2]);
//        }
//    }
//}