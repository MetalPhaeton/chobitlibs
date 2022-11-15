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
    private constructor(
        private _moduleID: bigint,
        private _instance: WebAssembly.Instance | null,
        private _exports: Exports | null,
        private _inputBufferInfo: [number, number],
        private _outputBufferInfo: [number, number]
    ) {}

    static instantiate(
        moduleID: bigint,
        wasmURL: URL,
        sendMsgHandler: (to: bigint, data: Uint8Array) => void
    ): Promise<ChobitWasm> {
        const chobitWasm = new ChobitWasm(0n, null, null, [0, 0], [0, 0]);

        return WebAssembly.instantiateStreaming(
            fetch(wasmURL),
            chobitWasm._genImports(sendMsgHandler)
        ).then((obj) => {
            chobitWasm._moduleID = moduleID;
            chobitWasm._instance = obj.instance;
            chobitWasm._exports =
                chobitWasm._instance.exports as unknown as Exports;

            chobitWasm._exports.init(chobitWasm._moduleID);

            return chobitWasm;
        });
    }

    private _genImports(
        sendMsgHandler: (to: bigint, data: Uint8Array) => void
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

                        sendMsgHandler(to, data);
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

export class ChobitModule {
    private _msgBuffer: MessageBuffer;

    private _global: Worker;
    private _channel: MessageChannel;
    private _firstMessage: boolean;
    private _moduleID: bigint;

    constructor(messageBufferSize: number) {
        this._msgBuffer = new MessageBuffer(messageBufferSize);

        this._global = globalThis as unknown as Worker;
        this._channel = new MessageChannel();
        this._firstMessage = true;
        this._moduleID = 0n;

        this._global.onmessage = this._genOnMessage();
        this._channel.port1.onmessage = (evt: MessageEvent) => {
            this._global.postMessage(evt.data, [evt.data]);
        };
    }

    private _genOnMessage(): (evt: MessageEvent) => void {
        return (evt) => {
            if (this._firstMessage) {
                const msg = this._msgBuffer.decodeInitMsg(
                    evt.data as unknown as ArrayBuffer
                );

                if (msg) {
                    this._firstMessage = false;
                    this._moduleID = msg[1];

                    ChobitWasm.instantiate(
                        msg[1],
                        new URL(new TextDecoder().decode(msg[2])),
                        (to, data) => {
                            this._channel.port2.postMessage(
                                this._msgBuffer.encodeSendMsg(to, data)
                            );
                        }
                    ).then((wasm) => {
                        this._channel.port2.onmessage = (evt) => {
                            const msg = this._msgBuffer.decodeRecvMsg(
                                evt.data as unknown as ArrayBuffer
                            );

                            if (msg) {
                                wasm.postData(msg[1], msg[2]);
                            }
                        };
                    });
                }
            } else {
                this._channel.port1.postMessage(evt.data, [evt.data]);
            }
        };
    }
}

/**
 * Cannel from main thread to ChobitWorker.
 */
export class ChobitWorker{
    private _msgBuffer: MessageBuffer;

    private _moduleID: bigint;

    private _worker: Worker;

    /**
     * Constructor.
     *
     * @param bufferSize Initial Buffer size for internal MessageBuffer.
     * @param moduleID Module ID for ChobitWasm on ChobitModule.
     * @param moduleURL URL of Javascript file for worker.
     * @param wasmURL URL of wasm file for ChobitWasm on ChobitModule.
     * @param onWasmOK Called when wasm is established on ChobitModule.
     * @param sendMsgHandler Called when ChobitModule send message to this.
     */
    constructor(
        msgBufferSize: number,
        moduleID: bigint,
        moduleURL: URL,
        wasmURL: URL,
        sendMsgHandler: (to: bigint, data: Uint8Array) => void
    ) {
        this._msgBuffer = new MessageBuffer(msgBufferSize);
        this._moduleID = moduleID;

        this._worker = this._initWorker(
            moduleID,
            moduleURL,
            wasmURL,
            sendMsgHandler
        );
    }

    /**
     * Gets module ID of ChobitWasm on ChobitWorker.
     *
     * @return Module ID.
     */
    get moduleID(): bigint {return this._moduleID;}

    private _initWorker(
        moduleID: bigint,
        moduleURL: URL,
        wasmURL: URL,
        sendMsgHandler: (to: bigint, data: Uint8Array) => void
    ): Worker {
        const ret = new Worker(moduleURL, {type: "module"});

        ret.onmessage = (evt) => {
            const msg = this._msgBuffer.decodeSendMsg(
                evt.data as unknown as ArrayBuffer
            );

            if (msg) {
                sendMsgHandler(msg[1], msg[2]);
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
        const msg = this._msgBuffer.encodeRecvMsg(from, data);
        this._worker.postMessage(msg);
    }

    /**
     * Terminates ChobitWorker.
     */
    terminate() {
        this._worker.terminate();
    }
}

export class ChobitBase {
    private _moduleID: bigint;

    private _workers: ChobitWorker[];

    private _onRecv: (from: bigint, data: Uint8Array) => void;

    constructor(onRecv: (from: bigint, data: Uint8Array) => void) {
        this._moduleID = 0n;

        this._workers = [];

        this._onRecv = onRecv;
    }

    get moduleID(): bigint {return this._moduleID;}

    addWorker(
        bufferSize: number,
        moduleID: bigint,
        moduleURL: URL,
        wasmURL: URL
    ) {
        this._workers.push(new ChobitWorker(
            bufferSize,
            moduleID,
            moduleURL,
            wasmURL,
            this._genSendMsgHandler(moduleID)
        ));
    }

    private _genSendMsgHandler(
        moduleID: bigint
    ): (to: bigint, data: Uint8Array) => void {
        return (to, data) => {
            if (to == this._moduleID) {
                this._onRecv(moduleID, data);
            } else {
                for (const worker of this._workers) {
                    if (worker.moduleID == to) {
                        worker.postData(moduleID, data);

                        break;
                    }
                }
            }
        };
    }

    postData(moduleID: bigint, from: bigint, data: Uint8Array) {
        for (const worker of this._workers) {
            if (worker.moduleID == moduleID) {
                worker.postData(from, data);
            }
        }
    }

    terminate(moduleID: bigint) {
        this._workers = this._workers.filter((worker) => {
            if (worker.moduleID == moduleID) {
                worker.terminate();
                return false;
            } else {
                return true;
            }
        });
    }

    numWorkers(): number {return this._workers.length;}
}
