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

export class MessageEncoder {
    private _textEncoder: TextEncoder;
    private _initID: bigint;
    private _recvID: bigint;
    private _notifyInputBufferID: bigint;
    private _notifyOutputBufferID: bigint;
    private _sendID: bigint;

    constructor() {
        this._textEncoder = new TextEncoder;

        this._initID = this.toID("init");
        this._recvID = this.toID("recv");
        this._notifyInputBufferID = this.toID("notify_input_buffer");
        this._notifyOutputBufferID = this.toID("notify_output_buffer");
        this._sendID = this.toID("send");
    }

    private _fnv1a64(bytes: Uint8Array) {
        let ret: bigint = 0xcbf29ce484222325n;
        const PRIME: bigint = 0x00000100000001b3n as const;

        bytes.forEach((x) => {
            ret = (ret ^ BigInt(x)) & 0xffffffffffffffffn;
            ret = (ret * PRIME) & 0xffffffffffffffffn;
        });

        return ret;
    }

    get initID(): bigint {return this._initID;}
    get recvID(): bigint {return this._recvID;}
    get notifyInputBufferID(): bigint {return this._notifyInputBufferID;}
    get notifyOutputBufferID(): bigint {return this._notifyOutputBufferID;}
    get sendID(): bigint {return this._sendID;}

    toID(text: string): bigint {
        return this._fnv1a64(this._textEncoder.encode(text));
    }

    decodeMsgID(bytes: Uint8Array): bigint | null {
        if (bytes.length < 8) {return null;}

        const array = new BigUint64Array(bytes.buffer, 0, 1);

        return array[0];
    }

    isInitMsg(bytes: Uint8Array): boolean {
        return this.decodeMsgID(bytes) == this._initID;
    }

    isRecvMsg(bytes: Uint8Array): boolean {
        return this.decodeMsgID(bytes) == this._recvID;
    }

    isNotifyInputBufferMsg(bytes: Uint8Array): boolean {
        return this.decodeMsgID(bytes) == this._notifyInputBufferID;
    }

    isNotifyOutputBufferMsg(bytes: Uint8Array): boolean {
        return this.decodeMsgID(bytes) == this._notifyOutputBufferID;
    }

    isSendMsg(bytes: Uint8Array): boolean {
        return this.decodeMsgID(bytes) == this._sendID;
    }

    encodeInitMsg(id: bigint): Uint8Array {
        const ret = new Uint8Array(16);

        const tmp = new BigUint64Array(ret.buffer);
        tmp[0] = this._initID;
        tmp[1] = id;

        return ret;
    }

    decodeInitMsg(bytes: Uint8Array): [bigint, bigint] | null {
        if (bytes.length < 16) {return null;}

        const array = new BigUint64Array(bytes.buffer, 0, 2);

        if (array[0] == this._initID) {
            return [array[0], array[1]];
        } else {
            return null;
        }
    }

    encodeRecvMsg(from: bigint, data: Uint8Array): Uint8Array {
        const ret = new Uint8Array(16 + data.length);

        const tmp = new BigUint64Array(ret.buffer, 0, 2);
        tmp[0] = this._recvID;
        tmp[1] = from;

        ret.set(data, 16);

        return ret;
    }

    decodeRecvMsg(bytes: Uint8Array): [bigint, bigint, Uint8Array] | null {
        if (bytes.length < 16) {return null;}

        const header = new BigUint64Array(bytes.buffer, 0, 2);
        const data = new Uint8Array(bytes.buffer, 16, bytes.length - 16);

        if (header[0] == this._recvID) {
            return [header[0], header[1], data];
        } else {
            return null;
        }
    }

    encodeNotifyInputBufferMsg(offset: number, size: number): Uint8Array {
        const ret = new Uint8Array(16);

        const tmp1 = new BigUint64Array(ret.buffer, 0, 1);
        const tmp2 = new Uint32Array(ret.buffer, 8, 2);

        tmp1[0] = this._notifyInputBufferID;
        tmp2[0] = offset;
        tmp2[1] = size;

        return ret;
    }

    decodeNotifyInputBufferMsg(
        bytes: Uint8Array
    ): [bigint, number, number] | null {
        if (bytes.length < 16) {return null;}

        const id = new BigUint64Array(bytes.buffer, 0, 1);
        const info = new Uint32Array(bytes.buffer, 8, 2);

        if (id[0] == this._notifyInputBufferID) {
            return [id[0], info[0], info[1]];
        } else {
            return null;
        }
    }

    encodeNotifyOutputBufferMsg(offset: number, size: number): Uint8Array {
        const ret = new Uint8Array(16);

        const tmp1 = new BigUint64Array(ret.buffer, 0, 1);
        const tmp2 = new Uint32Array(ret.buffer, 8, 2);

        tmp1[0] = this._notifyOutputBufferID;
        tmp2[0] = offset;
        tmp2[1] = size;

        return ret;
    }

    decodeNotifyOutputBufferMsg(
        bytes: Uint8Array
    ): [bigint, number, number] | null {
        if (bytes.length < 16) {return null;}

        const id = new BigUint64Array(bytes.buffer, 0, 1);
        const info = new Uint32Array(bytes.buffer, 8, 2);

        if (id[0] == this._notifyOutputBufferID) {
            return [id[0], info[0], info[1]];
        } else {
            return null;
        }
    }

    encodeSendMsg(to: bigint, data: Uint8Array): Uint8Array {
        const ret = new Uint8Array(16 + data.length);

        const tmp = new BigUint64Array(ret.buffer, 0, 2);
        tmp[0] = this._sendID;
        tmp[1] = to;

        ret.set(data, 16);

        return ret;
    }

    decodeSendMsg(bytes: Uint8Array): [bigint, bigint, Uint8Array] | null {
        if (bytes.length < 16) {return null;}

        const header = new BigUint64Array(bytes.buffer, 0, 2);
        const data = new Uint8Array(bytes.buffer, 16, bytes.length - 16);

        if (header[0] == this._sendID) {
            return [header[0], header[1], data];
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

export class ChobitWASM {
    private _exports: Exports | null;

    private _inputBufferInfo: [number, number];
    private _outputBufferInfo: [number, number];

    constructor() {
        this._exports = null;

        this._inputBufferInfo = [0, 0];
        this._outputBufferInfo = [0, 0];
    }

    genWASM(
        url: URL,
        id: bigint,
        imports: any
    ): Promise<void> | null {
        return WebAssembly.instantiateStreaming(
            fetch(url),
            imports
        ).then((obj) => {
            this._exports = obj.instance.exports as unknown as Exports;

            this._exports.init(id);
        });
    }

    genDefaultImports(
        outputHandler: (to: bigint, data: Uint8Array) => void
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

                        outputHandler(to, data);
                    }
                }
            }
        };
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

//interface ChobitWASM {
//    memory: WebAssembly.Memory,
//    init: (id: bigint) => void,
//    recv: (from: bigint, length: number) => void
//};
//
//export class ChobitModuleBase {
//    private _messageEncoder: MessageEncoder;
//
//    private _thisThread: [
//        bigint,
//        ChobitWASM,
//        [number, number],
//        [number, number]
//    ][];
//
//    private _workerThread: [bigint, Worker][];
//
//    private _onMessageHandler: (event: MessageEvent) => void;
//
//    constructor() {
//        this._messageEncoder = new MessageEncoder();
//
//        this._thisThread = [];
//        this._workerThread = [];
//
//        this._onMessageHandler = this._genOnMessageHandler();
//    }
//
//    private _genOnMessageHandler() {
//        const self = this;
//
//        return (event: MessageEvent) => {
//            const msg = new Uint8Array(event.data as unknown as ArrayBuffer);
//
//            self._handleMsg(msg);
//        };
//    }
//
//    private _handleMsg(msg: Uint8Array) {
//        const parsedData = this._messageEncoder.decodeSendMsg(msg);
//
//        if (parsedData) {
//            this.sendData(parsedData[1], parsedData[2]);
//        }
//    }
//
//    genChobitModuleInThisThread(
//        url: string,
//        id: bigint,
//        importObject: any = this.genDefaultInportObject(id)
//    ): Promise<void> | null {
//        const self = this;
//
//        // checks if id exists or not.
//        for (const elm of this._thisThread) {
//            if (elm[0] == id) {return null;}
//        }
//        for (const elm of this._workerThread) {
//            if (elm[0] == id) {return null;}
//        }
//
//        return this._genWebAssemblyInstance(url, id, importObject);
//    }
//
//    private _genWebAssemblyInstance(
//        url: string,
//        id: bigint,
//        importObject: any
//    ): Promise<void> {
//        const self = this;
//
//        return WebAssembly.instantiateStreaming(fetch(url), importObject).then(
//            (obj) => {
//                let chobitInstance =
//                    obj.instance.exports as unknown as ChobitWASM;
//
//                self._thisThread.push([
//                    id,
//                    chobitInstance,
//                    [0, 0],
//                    [0, 0]
//                ]);
//
//                chobitInstance.init(id);
//            }
//        );
//    }
//
//    genChobitModuleWorker(
//        worker_url: string,
//        wasm_url: string,
//        id: bigint
//    ): Promise<void> | null {
//        const self = this;
//
//        // checks if id exists or not.
//        for (const elm of this._thisThread) {
//            if (elm[0] == id) {return null;}
//        }
//        for (const elm of this._workerThread) {
//            if (elm[0] == id) {return null;}
//        }
//
//        return fetch(wasm_url).then((response) => {
//            return response.arrayBuffer();
//        }).then((buffer) => {
//            const wasmFile = new Uint8Array(buffer);
//
//            const msg = new Uint8Array(wasmFile.length + 16);
//
//            const tmp = new BigUint64Array(msg.buffer, 0, 2);
//            tmp[0] = self._messageEncoder.initID;
//            tmp[1] = id;
//
//            msg.set(wasmFile, 16);
//
//            const worker = new Worker(worker_url, {type: "module"});
//            worker.onmessage = this._onMessageHandler;
//
//            const msgBuffer = msg.buffer;
//            worker.postMessage(msgBuffer, [msgBuffer]);
//
//            self._workerThread.push([id, worker]);
//        });
//    }
//
//    genDefaultInportObject(id: bigint): any {
//        const self = this;
//        return {
//            env: {
//                notify_input_buffer: (offset: number, size: number) => {
//                    for (const elm of self._thisThread) {
//                        if (elm[0] == id) {
//                            elm[2][0] = offset;
//                            elm[2][1] = size;
//
//                            return;
//                        }
//                    }
//                },
//
//                notify_output_buffer: (offset: number, size: number) => {
//                    for (const elm of self._thisThread) {
//                        if (elm[0] == id) {
//                            elm[3][0] = offset;
//                            elm[3][1] = size;
//
//                            return;
//                        }
//                    }
//                },
//
//                send: (to: bigint, length: number) => {
//                    for (const elm of self._thisThread) {
//                        if (elm[0] == id) {
//                            const offset = elm[3][0];
//                            const size = elm[3][1];
//
//                            if (length > size) {return;}
//
//                            const data = new Uint8Array(
//                                elm[1].memory.buffer,
//                                offset,
//                                length
//                            );
//
//                            self.sendData(to, data);
//
//                            return;
//                        }
//                    }
//                }
//            }
//        };
//    }
//
//    sendData(to: bigint, data: Uint8Array) {
//        for (const elm of this._thisThread) {
//            if (elm[0] == to) {
//                const offset = elm[2][0];
//                const size = elm[2][1];
//
//                const inputBuffer =
//                    new Uint8Array(elm[1].memory.buffer, offset, size);
//
//                inputBuffer.set(data);
//
//                elm[1].recv(to, data.length);
//
//                return;
//            }
//        }
//
//        for (const elm of this._workerThread) {
//            if (elm[0] == to) {
//                const msg =
//                    this._messageEncoder.encodeRecvMsg(to, data).buffer;
//
//                elm[1].postMessage(msg, [msg]);
//
//                return;
//            }
//        }
//    }
//}
//
//export class ChobitWorker {
//    private _messageEncoder: MessageEncoder;
//    private _global: Worker;
//
//    private _id: bigint;
//    private _imporObject: any;
//    private _wasm: ChobitWASM | null;
//    private _input_buffer_info: [number, number];
//    private _output_buffer_info: [number, number];
//
//    constructor(importObject: any) {
//        this._messageEncoder = new MessageEncoder();
//        this._global = globalThis as unknown as Worker;
//
//        const self = this;
//        this._global.onmessage = (event: MessageEvent) => {
//            const msg = new Uint8Array(event.data);
//
//            self.handleMsg(msg);
//        };
//
//        this._id = 0n;
//        this._imporObject = importObject;
//        this._wasm = null;
//
//        this._input_buffer_info = [0, 0];
//        this._output_buffer_info = [0, 0];
//    }
//
//    private handleMsg(msg: Uint8Array): Promise<void> | null {
//        const initInfo = this._messageEncoder.decodeInitMsg(msg);
//
//        if (initInfo) {
//            const data = new Uint8Array(msg.buffer, 16, msg.length - 16);
//
//            return this.initWASM(initInfo[1], data);
//        }
//
//        const sendInfo = this._messageEncoder.decodeSendMsg(msg);
//
//        if (sendInfo) {
//            // TODO
//            return null;
//        }
//    }
//
//    private initWASM(id: bigint, data: Uint8Array): Promise<void> | null {
//        if (this._wasm) {return null;}
//        if (data <= 0) {return null;}
//
//        return WebAssembly.instantiateStreaming(
//            data,
//            this._importObject
//        ).then((obj) => {
//            // TODO
//        });
//    }
//}
