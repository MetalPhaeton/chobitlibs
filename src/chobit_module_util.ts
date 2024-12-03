// Copyright (C) 2022 Hironori Ishibashi
//
// This work is free. You can redistribute it and/or modify it under the
// terms of the Do What The Fuck You Want To Public License, Version 2,
// as published by Sam Hocevar. See http://www.wtfpl.net/ for more details.

/**
 * Error of decoding.
 */
export class DecodeError extends Error {
    constructor() {
        super("DecodeError");
    }
}

/**
 * FNV-1a 64 bits.
 *
 * @param bytes Bytes to hash.
 * @return Hash value.
 */
export function fnv1a64(bytes: Uint8Array): bigint {
    let ret: bigint = BigInt("0xcbf29ce484222325");
    const PRIME: bigint = BigInt("0x00000100000001b3");

    bytes.forEach((x) => {
        ret = (ret ^ BigInt(x)) & BigInt("0xffffffffffffffff");
        ret = (ret * PRIME) & BigInt("0xffffffffffffffff");
    });

    return ret;
}

const HEADER_OFFSET: number = 20 as const;

/**
 * Buffer memory for messages of chobit module.
 */
export class MessageBuffer {
    #textEncoder: TextEncoder;
    #buffer: ArrayBuffer;

    #initID: bigint;
    #recvID: bigint;
    #sendID: bigint;

    /**
     * Constructor.
     *
     * @param bufferSize Initial buffer size.
     */
    constructor(bufferSize: number) {
        this.#textEncoder = new TextEncoder();
        this.#buffer = new ArrayBuffer(bufferSize);

        this.#initID = this.toMsgID("init");
        this.#recvID = this.toMsgID("recv");
        this.#sendID = this.toMsgID("send");
    }

    /**
     * Gets ID of "init" message.
     *
     * @return ID
     */
    get initID(): bigint {return this.#initID;}

    /**
     * Gets ID of "recv" message.
     *
     * @return ID
     */
    get recvID(): bigint {return this.#recvID;}

    /**
     * Gets ID of "send" message.
     *
     * @return ID
     */
    get sendID(): bigint {return this.#sendID;}

    #fixBufferSize(requiredSize: number) {
        if (this.#buffer.byteLength < requiredSize) {
            let size = this.#buffer.byteLength;
            while (size < requiredSize) {
                size *= 2;
            }

            this.#buffer = new ArrayBuffer(size);
        }
    }

    /**
     * Hashes from text into FNV-1a.
     *
     * @param text Text to hash.
     * @return Hash value.
     */
    toMsgID(text: string): bigint {
        return fnv1a64(this.#textEncoder.encode(text));
    }

    #encodeMsg(
        msgID: bigint,
        id: bigint,
        data: Uint8Array
    ): ArrayBuffer {
        this.#fixBufferSize(data.length + HEADER_OFFSET);

        const view = new DataView(this.#buffer);
        view.setBigUint64(0, msgID, true);
        view.setBigUint64(8, id, true);

        view.setUint32(16, data.length, true);

        const tmp = new Uint8Array(this.#buffer, HEADER_OFFSET, data.length);

        tmp.set(data);

        return this.#buffer;
    }

    #decodeMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] {
        if (msg.byteLength < HEADER_OFFSET) {
            throw new DecodeError();
        }

        const view = new DataView(msg);
        const msgID = view.getBigUint64(0, true);
        const moduleID = view.getBigUint64(8, true);
        const dataLength = view.getUint32(16, true);

        if ((dataLength + HEADER_OFFSET) > msg.byteLength) {
            throw new DecodeError();
        }

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
        return this.#encodeMsg(this.#initID, id, data);
    }

    /**
     * Decodes "init" message byte string into message data.
     *
     * @param msg Message as byte string.
     * @return [message ID, module ID, additional data]
     * @throws {DecodeError}
     */
    decodeInitMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] {
        const ret = this.#decodeMsg(msg);

        if (ret[0] == this.#initID) {
            return ret;
        } else {
            throw new DecodeError();
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
        return this.#encodeMsg(this.#recvID, from, data);
    }

    /**
     * Decodes "recv" message byte string into message data.
     *
     * @param msg Message as byte string.
     * @return [message ID, sender ID, additional data]
     * @throws {DecodeError}
     */
    decodeRecvMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] {
        const ret = this.#decodeMsg(msg);

        if (ret[0] == this.#recvID) {
            return ret;
        } else {
            throw new DecodeError();
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
        return this.#encodeMsg(this.#sendID, to, data);
    }

    /**
     * Decodes "send" message byte string into message data.
     *
     * @param msg Message as byte string.
     * @return [message ID, receiver ID, additional data]
     * @throws {DecodeError}
     */
    decodeSendMsg(msg: ArrayBuffer): [bigint, bigint, Uint8Array] {
        const ret = this.#decodeMsg(msg);

        if (ret[0] == this.#sendID) {
            return ret;
        } else {
            throw new DecodeError();
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
    #moduleID: bigint;
    #instance: WebAssembly.Instance | null;
    #exports: Exports | null;
    #inputBufferInfo: [number, number];
    #outputBufferInfo: [number, number];

    private constructor(
        moduleID: bigint,
        instance: WebAssembly.Instance | null,
        exports: Exports | null,
        inputBufferInfo: [number, number],
        outputBufferInfo: [number, number]
    ) {
        this.#moduleID = moduleID;
        this.#instance = instance;
        this.#exports = exports;
        this.#inputBufferInfo = inputBufferInfo;
        this.#outputBufferInfo = outputBufferInfo;
    }

    /**
     * Instatiates ChobitWasm.
     *
     * @param moduleID Module ID.
     * @param wasmURL URL of wasm file.
     * @param sendMsgHandler Calls when wasm call `send(to, length)`.
     */
    static instantiate(
        moduleID: bigint,
        wasmURL: URL,
        sendMsgHandler: (to: bigint, data: Uint8Array) => void
    ): Promise<ChobitWasm> {
        const chobitWasm = new ChobitWasm(
            BigInt(0),
            null,
            null,
            [0, 0],
            [0, 0]
        );

        return WebAssembly.instantiateStreaming(
            fetch(wasmURL),
            chobitWasm.#genImports(sendMsgHandler)
        ).then((obj) => {
            chobitWasm.#moduleID = moduleID;
            chobitWasm.#instance = obj.instance;
            chobitWasm.#exports =
                chobitWasm.#instance.exports as unknown as Exports;

            chobitWasm.#exports.init(chobitWasm.#moduleID);

            return chobitWasm;
        });
    }

    #genImports(
        sendMsgHandler: (to: bigint, data: Uint8Array) => void
    ): any {
        return {
            env: {
                notify_input_buffer: (offset: number, size: number) => {
                    this.#inputBufferInfo = [offset, size];
                },

                notify_output_buffer: (offset: number, size: number) => {
                    this.#outputBufferInfo = [offset, size];
                },

                send: (to: bigint, length: number) => {
                    if (length > this.#outputBufferInfo[1]) {return;}

                    if (this.#exports != null) {
                        const data = new Uint8Array(
                            this.#exports.memory.buffer,
                            this.#outputBufferInfo[0],
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
        if (data.length > this.#inputBufferInfo[1]) {return;}

        if (this.#exports != null) {
            const inputBuffer = new Uint8Array(
                this.#exports.memory.buffer,
                this.#inputBufferInfo[0],
                this.#inputBufferInfo[1]
            );

            inputBuffer.set(data);

            this.#exports.recv(from, data.length);
        }
    }
}

export class ChobitModule {
    #msgBuffer: MessageBuffer;

    #global: Worker;
    #channel: MessageChannel;
    #firstMessage: boolean;
    #moduleID: bigint;

    /**
     * Constructor.
     *
     * @param msgBufferSize A size of internal MessageBuffer.
     */
    constructor(msgBufferSize: number) {
        this.#msgBuffer = new MessageBuffer(msgBufferSize);

        this.#global = globalThis as unknown as Worker;
        this.#channel = new MessageChannel();
        this.#firstMessage = true;
        this.#moduleID = BigInt(0);

        this.#global.onmessage = this.#genOnMessage();
        this.#channel.port1.onmessage = (evt: MessageEvent) => {
            this.#global.postMessage(evt.data, [evt.data]);
        };
    }

    /**
     * Gets this own ID.
     *
     * @return Module ID. It is 0.
     */
    get moduleID(): bigint {return this.#moduleID;}

    #genOnMessage(): (evt: MessageEvent) => void {
        return (evt) => {
            if (this.#firstMessage) {
                const msg = this.#msgBuffer.decodeInitMsg(
                    evt.data as unknown as ArrayBuffer
                );

                this.#firstMessage = false;
                this.#moduleID = msg[1];

                ChobitWasm.instantiate(
                    msg[1],
                    new URL(new TextDecoder().decode(msg[2])),
                    (to, data) => {
                        this.#channel.port2.postMessage(
                            this.#msgBuffer.encodeSendMsg(to, data)
                        );
                    }
                ).then((wasm) => {
                    this.#channel.port2.onmessage = (evt) => {
                        const msg = this.#msgBuffer.decodeRecvMsg(
                            evt.data as unknown as ArrayBuffer
                        );

                        wasm.postData(msg[1], msg[2]);
                    };
                });
            } else {
                this.#channel.port1.postMessage(evt.data, [evt.data]);
            }
        };
    }
}

/**
 * Cannel from main thread to ChobitWorker.
 */
export class ChobitWorker{
    #msgBuffer: MessageBuffer;

    #moduleID: bigint;

    #worker: Worker;

    /**
     * Constructor.
     *
     * @param msgBufferSize A size of internal MessageBuffer.
     * @param moduleID Module ID for ChobitModule.
     * @param moduleURL URL of Javascript file for ChobitModule.
     * @param wasmURL URL of wasm file for ChobitModule.
     * @param sendMsgHandler Called when ChobitModule send message to this.
     */
    constructor(
        msgBufferSize: number,
        moduleID: bigint,
        moduleURL: URL,
        wasmURL: URL,
        sendMsgHandler: (to: bigint, data: Uint8Array) => void
    ) {
        this.#msgBuffer = new MessageBuffer(msgBufferSize);
        this.#moduleID = moduleID;

        this.#worker = this.#initWorker(
            moduleID,
            moduleURL,
            wasmURL,
            sendMsgHandler
        );
    }

    /**
     * Gets module ID.
     *
     * @return Module ID.
     */
    get moduleID(): bigint {return this.#moduleID;}

    #initWorker(
        moduleID: bigint,
        moduleURL: URL,
        wasmURL: URL,
        sendMsgHandler: (to: bigint, data: Uint8Array) => void
    ): Worker {
        const ret = new Worker(moduleURL, {type: "module"});

        ret.onmessage = (evt) => {
            const msg = this.#msgBuffer.decodeSendMsg(
                evt.data as unknown as ArrayBuffer
            );

            sendMsgHandler(msg[1], msg[2]);
        };

        const msg = this.#msgBuffer.encodeInitMsg(
            moduleID,
            new TextEncoder().encode(wasmURL.href)
        );

        ret.postMessage(msg);

        return ret;
    }

    /**
     * Posts data to ChobitModule.
     *
     * @param from Sender ID.
     * @param data Data.
     */
    postData(from: bigint, data: Uint8Array) {
        const msg = this.#msgBuffer.encodeRecvMsg(from, data);
        this.#worker.postMessage(msg);
    }

    /**
     * Terminates worker.
     */
    terminate() {
        this.#worker.terminate();
    }
}

/**
 * ChobitWorker's communication terminal.
 */
export class ChobitBase {
    #moduleID: bigint;

    #workers: ChobitWorker[];

    #onRecv: (from: bigint, data: Uint8Array) => void;

    /**
     * Constructor.
     *
     * @param onRecv Called when a worker sends message to ID 0.
     */
    constructor(onRecv: (from: bigint, data: Uint8Array) => void) {
        this.#moduleID = BigInt(0);

        this.#workers = [];

        this.#onRecv = onRecv;
    }

    /**
     * Gets module ID of this base.
     *
     * @return Module ID.
     */
    get moduleID(): bigint {return this.#moduleID;}

    /**
     * Adds ChobitWorker.
     *
     * @param msgBufferSize A size of internal MessageBuffer on the worker.
     * @param moduleID Module ID for the worker.
     * @param moduleURL URL of Javascript file for ChobitModule.
     * @param wasmURL URL of wasm file for ChobitModule.
     */
    addWorker(
        msgBufferSize: number,
        moduleID: bigint,
        moduleURL: URL,
        wasmURL: URL
    ) {
        this.#workers.push(new ChobitWorker(
            msgBufferSize,
            moduleID,
            moduleURL,
            wasmURL,
            this.#genSendMsgHandler(moduleID)
        ));
    }

    #genSendMsgHandler(
        moduleID: bigint
    ): (to: bigint, data: Uint8Array) => void {
        return (to, data) => {
            if (to == this.#moduleID) {
                this.#onRecv(moduleID, data);
            } else {
                for (const worker of this.#workers) {
                    if (worker.moduleID == to) {
                        worker.postData(moduleID, data);

                        break;
                    }
                }
            }
        };
    }

    /**
     * Posts data to a worker.
     *
     * @param moduleID Receiver ID.
     * @param from Sender ID.
     * @param data Data.
     */
    postData(moduleID: bigint, from: bigint, data: Uint8Array) {
        for (const worker of this.#workers) {
            if (worker.moduleID == moduleID) {
                worker.postData(from, data);
            }
        }
    }

    /**
     * Broadcasts data to all workers.
     *
     * @param from Sender ID.
     * @param data Data.
     */
    broadcastData(from: bigint, data: Uint8Array) {
        for (const worker of this.#workers) {
            worker.postData(from, data);
        }
    }

    /**
     * Terminats a worker.
     *
     * @param moduleID Module ID of the worker.
     */
    terminate(moduleID: bigint) {
        this.#workers = this.#workers.filter((worker) => {
            if (worker.moduleID == moduleID) {
                worker.terminate();
                return false;
            } else {
                return true;
            }
        });
    }

    /**
     * Terminats all workers.
     */
    terminateAll() {
        for (const worker of this.#workers) {
            worker.terminate();
        }

        this.#workers = [];
    }

    /**
     * Gets number of  workers.
     *
     * @return Number of workers.
     */
    numWorkers(): number {return this.#workers.length;}
}
