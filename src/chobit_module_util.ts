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

/**
 * FNV-1a 64 bits.
 *
 * @param bytes Bytes to hash.
 * @return Hash value.
 */
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

/**
 * Buffer memory for messages of chobit module.
 */
export class MessageBuffer {
    private _textEncoder: TextEncoder;
    private _buffer: ArrayBuffer;

    private _initID: bigint;
    private _recvID: bigint;
    private _sendID: bigint;
    private _wasmOKID: bigint;

    /**
     * Constructor.
     *
     * @param bufferSize Initial buffer size.
     */
    constructor(bufferSize: number) {
        this._textEncoder = new TextEncoder();
        this._buffer = new ArrayBuffer(bufferSize);

        this._initID = this.toMsgID("init");
        this._recvID = this.toMsgID("recv");
        this._sendID = this.toMsgID("send");
        this._wasmOKID = this.toMsgID("wasm-ok");
    }

    /**
     * Gets ID of "init" message.
     *
     * @return ID
     */
    get initID(): bigint {return this._initID;}

    /**
     * Gets ID of "recv" message.
     *
     * @return ID
     */
    get recvID(): bigint {return this._recvID;}

    /**
     * Gets ID of "send" message.
     *
     * @return ID
     */
    get sendID(): bigint {return this._sendID;}

    /**
     * Gets ID of "wasm-ok" message.
     *
     * @return ID
     */
    get wasmOKID(): bigint {return this._wasmOKID;}

    private _fixBufferSize(requiredSize: number) {
        if (this._buffer.byteLength < requiredSize) {
            let size = this._buffer.byteLength;
            while (size < requiredSize) {
                size *= 2;
            }

            this._buffer = new ArrayBuffer(size);
        }
    }

    /**
     * Hashes from text into FNV-1a.
     *
     * @param text Text to hash.
     * @return Hash value.
     */
    toMsgID(text: string): bigint {
        return fnv1a64(this._textEncoder.encode(text));
    }

    private _encodeMsg(
        msgID: bigint,
        id: bigint,
        data: Uint8Array
    ): ArrayBuffer {
        this._fixBufferSize(data.length + HEADER_OFFSET);

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

    /**
     * Encodes "init" message data into byte string.
     *
     * @param id Module ID.
     * @param data Additional data.
     * @return Byte string using ArrayBuffer of this MessageBuffer instance.
     */
    encodeInitMsg(id: bigint, data: Uint8Array): ArrayBuffer {
        return this._encodeMsg(this._initID, id, data);
    }

    /**
     * Decodes "init" message byte string into message data.
     *
     * @param msg Message as byte string.
     * @return [message ID, module ID, additional data]
     */
    decodeInitMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] | null {
        const ret = this._decodeMsg(msg);

        if (ret && (ret[0] == this._initID)) {
            return ret;
        } else {
            return null;
        }
    }

    /**
     * Encodes "recv" message data into byte string.
     *
     * @param from Sender ID.
     * @param data Additional data.
     * @return Byte string using ArrayBuffer of this MessageBuffer instance.
     */
    encodeRecvMsg(from: bigint, data: Uint8Array): ArrayBuffer {
        return this._encodeMsg(this._recvID, from, data);
    }

    /**
     * Decodes "recv" message byte string into message data.
     *
     * @param msg Message as byte string.
     * @return [message ID, sender ID, additional data]
     */
    decodeRecvMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] | null {
        const ret = this._decodeMsg(msg);

        if (ret && (ret[0] == this._recvID)) {
            return ret;
        } else {
            return null;
        }
    }

    /**
     * Encodes "send" message data into byte string.
     *
     * @param to Receiver ID.
     * @param data Additional data.
     * @return Byte string using ArrayBuffer of this MessageBuffer instance.
     */
    encodeSendMsg(to: bigint, data: Uint8Array): ArrayBuffer {
        return this._encodeMsg(this._sendID, to, data);
    }

    /**
     * Decodes "send" message byte string into message data.
     *
     * @param msg Message as byte string.
     * @return [message ID, receiver ID, additional data]
     */
    decodeSendMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] | null {
        const ret = this._decodeMsg(msg);

        if (ret && (ret[0] == this._sendID)) {
            return ret;
        } else {
            return null;
        }
    }

    /**
     * Encodes "wasm-ok" message data into byte string.
     *
     * @param to Module ID.
     * @param data Additional data.
     * @return Byte string using ArrayBuffer of this MessageBuffer instance.
     */
    encodeWasmOKMsg(id: bigint, data: Uint8Array): ArrayBuffer {
        return this._encodeMsg(this._wasmOKID, id, data);
    }

    /**
     * Decodes "wasm-ok" message byte string into message data.
     *
     * @param msg Message as byte string.
     * @return [message ID, module ID, additional data]
     */
    decodeWasmOKMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] | null {
        const ret = this._decodeMsg(msg);

        if (ret && (ret[0] == this._wasmOKID)) {
            return ret;
        } else {
            return null;
        }
    }
}

interface Exports {
    memory: WebAssembly.Memory,

    init: (id: bigint) => void,
    recv: (from: bigint, length: number) => void
}

/**
 * Instance of ChobitModule.
 */
export class ChobitWasm {
    private _instance: WebAssembly.Instance | null;
    private _exports: Exports | null;

    private _wasmID: bigint;
    private _inputBufferInfo: [number, number];
    private _outputBufferInfo: [number, number];

    /**
     * Constructor.
     */
    constructor() {
        this._instance = null;
        this._exports = null;

        this._wasmID = 0n;
        this._inputBufferInfo = [0, 0];
        this._outputBufferInfo = [0, 0];
    }

    get wasmID() {return this._wasmID;}

    isEstablished(): boolean {return this._exports != null;}

    establish(
        url: URL,
        id: bigint,
        onSend: (to: bigint, data: Uint8Array) => void,
        additionalImports: any = {}
    ): Promise<void> {
        const imports = this._genDefaultImports(onSend);
        this._addProps(imports, additionalImports);

        return WebAssembly.instantiateStreaming(
            fetch(url),
            imports
        ).then((obj) => {
            if (this.isEstablished()) {
                throw "wasmID: " + this._wasmID + " is already built";
            }

            this._instance = obj.instance;
            this._exports = this._instance.exports as unknown as Exports;

            this._wasmID = id;
            this._exports.init(id);
        });
    }

    private _genDefaultImports(
        onSend: (to: bigint, data: Uint8Array) => void
    ): any {
        return {
            env: {
                notify_input_buffer: (offset: number, size: number) => {
                    this._inputBufferInfo = [offset, size];
                },

                notify_output_buffer: (offset: number, size: number) => {
                    this._outputBufferInfo = [offset, size];
                },

                send: (to: bigint, length: number) => {
                    if (length > this._outputBufferInfo[1]) {return;}

                    if (this._exports) {
                        const data = new Uint8Array(
                            this._exports.memory.buffer,
                            this._outputBufferInfo[0],
                            length
                        );

                        onSend(to, data);
                    }
                }
            }
        };
    }

    private _addProps(obj: any, additional: any) {
        for (const key in additional) {
            obj.env[key] = additional[key];
        }
    }

    input(from: bigint, data: Uint8Array) {
        if (data.length > this._inputBufferInfo[1]) {return;}

        if (this._exports) {
            const inputBuffer = new Uint8Array(
                this._exports.memory.buffer,
                this._inputBufferInfo[0],
                this._inputBufferInfo[1]
            );

            inputBuffer.set(data);

            this._exports.recv(from, data.length);
        }
    }
}

export class ChobitWorkerChannel {
    private _msgBuffer: MessageBuffer;
    private _worker: Worker;

    private _wasmID: bigint;

    constructor(
        bufferSize: number,
        workerURL: URL,
        wasmID: bigint,
        wasmURL: URL,
        onWasmOK: (from: bigint, data: Uint8Array) => void,
        onMessage: (from: bigint, data: Uint8Array) => void
    ) {
        this._msgBuffer = new MessageBuffer(bufferSize);
        this._wasmID = wasmID;

        this._worker = this._initWorker(
            workerURL,
            wasmID,
            wasmURL,
            onWasmOK,
            onMessage
        );
    }

    get wasmID(): bigint {return this._wasmID;}

    private _initWorker(
        workerURL: URL,
        wasmID: bigint,
        wasmURL: URL,
        onWasmOK: (from: bigint, data: Uint8Array) => void,
        onMessage: (from: bigint, data: Uint8Array) => void
    ): Worker {
        const ret = new Worker(workerURL, {type: "module"});

        ret.onmessage = (msg) => {
            const decodedMsg = this._msgBuffer.decodeSendMsg(
                msg.data as unknown as ArrayBuffer
            );

            if (decodedMsg) {
                onMessage(decodedMsg[1], decodedMsg[2]);
            } else {
                const decodedMsg = this._msgBuffer.decodeWasmOKMsg(
                    msg.data as unknown as ArrayBuffer
                );

                if (decodedMsg) {
                    onWasmOK(decodedMsg[1], decodedMsg[2]);
                }
            }
        };

        const msg = this._msgBuffer.encodeInitMsg(
            wasmID,
            new TextEncoder().encode(wasmURL.href)
        );

        ret.postMessage(msg);

        return ret;
    }

    postMessage(from: bigint, data: Uint8Array) {
        const msg = this._msgBuffer.encodeSendMsg(from, data);
        this._worker.postMessage(msg);
    }

    terminateWorker() {
        this._worker.terminate();
    }
}

export class ChobitWorker {
    private _global: Worker;
    private _msgBuffer: MessageBuffer;

    private _wasmID: bigint;
    private _wasm: ChobitWasm;

    constructor(bufferSize: number) {
        this._global = globalThis as unknown as Worker;

        this._msgBuffer = new MessageBuffer(bufferSize);

        this._wasmID = 0n;

        this._wasm = new ChobitWasm();

        this._global.onmessage = (msg) => {
            this._handleMsg(msg.data as unknown as ArrayBuffer);
        };
    }

    get wasmID() {return this._wasmID;}

    private _handleMsg(msg: ArrayBuffer) {
        if (this._wasm.isEstablished()) {
            this._handleSendMsg(msg);
        } else {
            this._handleInitMsg(msg);
        }
    }

    private _handleInitMsg(msg: ArrayBuffer) {
        const decodedMsg = this._msgBuffer.decodeInitMsg(msg);

        if (decodedMsg) {
            const id = decodedMsg[1];

            this._wasm.establish(
                new URL(new TextDecoder().decode(decodedMsg[2])),
                id,
                this._genOutputHandler()
            ).then(() => {
                this._wasmID = id;

                const msg = this._msgBuffer.encodeWasmOKMsg(
                    id,
                    new Uint8Array(0)
                );

                this._global.postMessage(msg);
            });
        }
    }

    private _genOutputHandler(): (to: bigint, data: Uint8Array) => void {
        return (to: bigint, data: Uint8Array) => {
            const msg = this._msgBuffer.encodeSendMsg(to, data);

            this._global.postMessage(msg);
        };
    }

    private _handleSendMsg(msg: ArrayBuffer) {
        const decodedMsg = this._msgBuffer.decodeSendMsg(msg);

        if (decodedMsg) {
            this._wasm.input(decodedMsg[1], decodedMsg[2]);
        }
    }
}
