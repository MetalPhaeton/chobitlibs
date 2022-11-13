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

        const view = new DataView(this._buffer);
        view.setBigUint64(0, msgID, true);
        view.setBigUint64(8, id, true);

        view.setUint32(16, data.length, true);

        const tmp = new Uint8Array(this._buffer, HEADER_OFFSET, data.length);

        tmp.set(data);

        return this._buffer;
    }

    private _decodeMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] | null {
        if (msg.byteLength < HEADER_OFFSET) {return null;}

        const view = new DataView(msg);
        const msgID = view.getBigUint64(0, true);
        const moduleID = view.getBigUint64(8, true);
        const dataLength = view.getUint32(16, true);

        if ((dataLength + HEADER_OFFSET) > msg.byteLength) {return null;}

        const tmp = new Uint8Array(msg, HEADER_OFFSET, dataLength);

        return [msgID, moduleID, tmp];
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
 * Instance of ChobitWasm.
 */
export class ChobitWasm {
    private _instance: WebAssembly.Instance | null;
    private _exports: Exports | null;

    private _moduleURL: URL;
    private _moduleID: bigint;
    private _imports: any;
    private _inputBufferInfo: [number, number];
    private _outputBufferInfo: [number, number];

    /**
     * Constructor. But Wasm is not established.
     *
     * @param url URL of Wasm file.
     * @param id Module id.
     * @param onSend Called when the module calls send() from wasm.
     */
    constructor(
        url: URL,
        id: bigint,
        onSend: (to: bigint, data: Uint8Array) => void
    ) {
        this._instance = null;
        this._exports = null;

        this._imports = this._genDefaultImports(onSend);

        this._moduleURL = url;
        this._moduleID = id;
        this._inputBufferInfo = [0, 0];
        this._outputBufferInfo = [0, 0];
    }

    /**
     * Gets module ID. If this is not established, returns 0.
     *
     * @return Module ID.
     */
    get moduleID() {return this._moduleID;}

    /**
     * Whether Wasm is established or not.
     *
     * @return If Wasm is established, returns true.
     */
    isEstablished(): boolean {return this._exports != null;}

    /**
     * Establishes Wasm.
     *
     * @return Promise.
     */
    establish(): Promise<void> {
        return WebAssembly.instantiateStreaming(
            fetch(this._moduleURL),
            this._imports
        ).then((obj) => {
            if (this.isEstablished()) {
                throw "moduleID: " + this._moduleID + " is already built";
            }

            this._instance = obj.instance;
            this._exports = this._instance.exports as unknown as Exports;

            this._exports.init(this._moduleID);
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

    /**
     * Posts data to wasm.
     *
     * @param from Sender ID.
     * @param data Data.
     */
    postData(from: bigint, data: Uint8Array) {
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

/**
 * Cannel from main thread to ChobitWorker.
 */
export class ChobitWorkerChannel {
    private _msgBuffer: MessageBuffer;
    private _worker: Worker;

    private _moduleID: bigint;

    /**
     * Constructor.
     *
     * @param bufferSize Buffer size for internal MessageBuffer.
     * @param workerURL URL of Javascript file for worker.
     * @param moduleID Module ID for ChobitWasm on ChobitWorker.
     * @param wasmURL URL of wasm file for ChobitWasm on ChobitWorker.
     * @param onWasmOK Called when wasm is established on ChobitWorker.
     * @param onRecv Called when ChobitWorker send message to this.
     */
    constructor(
        bufferSize: number,
        workerURL: URL,
        moduleID: bigint,
        wasmURL: URL,
        onWasmOK: (from: bigint, data: Uint8Array) => void,
        onRecv: (from: bigint, data: Uint8Array) => void
    ) {
        this._msgBuffer = new MessageBuffer(bufferSize);
        this._moduleID = moduleID;

        this._worker = this._initWorker(
            workerURL,
            moduleID,
            wasmURL,
            onWasmOK,
            onRecv
        );
    }

    /**
     * Gets module ID of ChobitWasm on ChobitWorker.
     *
     * @return Module ID.
     */
    get moduleID(): bigint {return this._moduleID;}

    private _initWorker(
        workerURL: URL,
        moduleID: bigint,
        wasmURL: URL,
        onWasmOK: (from: bigint, data: Uint8Array) => void,
        onRecv: (from: bigint, data: Uint8Array) => void
    ): Worker {
        const ret = new Worker(workerURL, {type: "module"});

        ret.onmessage = (msg) => {
            const decodedMsg = this._msgBuffer.decodeSendMsg(
                msg.data as unknown as ArrayBuffer
            );

            if (decodedMsg) {
                onRecv(decodedMsg[1], decodedMsg[2]);
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
            moduleID,
            new TextEncoder().encode(wasmURL.href)
        );

        ret.postMessage(msg);

        return ret;
    }

    /**
     * Posts data to ChobitWasm on ChobitWorker.
     *
     * @param from Sender ID.
     * @param data Data.
     */
    postData(from: bigint, data: Uint8Array) {
        const msg = this._msgBuffer.encodeSendMsg(from, data);
        this._worker.postMessage(msg);
    }

    /**
     * Terminates ChobitWorker.
     */
    terminateWorker() {
        this._worker.terminate();
    }
}

export class ChobitWorker {
    private _global: Worker;
    private _msgBuffer: MessageBuffer;

    private _moduleID: bigint;
    private _wasm: ChobitWasm | null;

    constructor(bufferSize: number) {
        this._global = globalThis as unknown as Worker;

        this._msgBuffer = new MessageBuffer(bufferSize);

        this._moduleID = 0n;

        this._wasm = null;

        this._global.onmessage = (msg) => {
            this._handleMsg(msg.data as unknown as ArrayBuffer);
        };
    }

    get moduleID() {return this._moduleID;}

    private _handleMsg(msg: ArrayBuffer) {
        if (this._wasm) {
            this._handleSendMsg(msg);
        } else {
            this._handleInitMsg(msg);
        }
    }

    private _handleInitMsg(msg: ArrayBuffer) {
        const decodedMsg = this._msgBuffer.decodeInitMsg(msg);

        if (decodedMsg) {
            const id = decodedMsg[1];

            this._wasm = new ChobitWasm(
                new URL(new TextDecoder().decode(decodedMsg[2])),
                id,
                this._genOutputHandler()
            );

            this._wasm.establish().then(() => {
                this._moduleID = id;

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

        if (decodedMsg&& this._wasm) {
            this._wasm.postData(decodedMsg[1], decodedMsg[2]);
        }
    }
}
