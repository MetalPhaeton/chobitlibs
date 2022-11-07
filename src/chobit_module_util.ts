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
    private _toID: (text: string) => bigint;
    private _initID: bigint;
    private _recvID: bigint;
    private _notifyInputBufferID: bigint;
    private _notifyOutputBufferID: bigint;
    private _sendID: bigint;

    constructor() {
        this._textEncoder = new TextEncoder;

        const fnv1a64 = (bytes: Uint8Array): bigint => {
            const dataArray = new BigUint64Array([
                0xcbf29ce484222325n,
                0x00000100000001b3n,
                0n
            ]);

            bytes.forEach((x) => {
                dataArray[2] = BigInt(x);
                dataArray[0] ^= dataArray[2];
                dataArray[0] *= dataArray[1];
            });

            return dataArray[0];
        };

        this._toID = (text: string): bigint => {
            return fnv1a64(this._textEncoder.encode(text));
        };

        this._initID = this._toID("init");
        this._recvID = this._toID("recv");
        this._notifyInputBufferID = this._toID("notify_input_buffer");
        this._notifyOutputBufferID = this._toID("notify_output_buffer");
        this._sendID = this._toID("send");
    }

    get initID(): bigint {return this._initID;}
    get recvID(): bigint {return this._recvID;}
    get notifyInputBufferID(): bigint {return this._notifyInputBufferID;}
    get notifyOutputBufferID(): bigint {return this._notifyOutputBufferID;}
    get sendID(): bigint {return this._sendID;}

    toID(text: string): bigint {
        return this._toID(text);
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

//export class ChobitModuleBase {
//    private _messageEncoder: MessageEncoder;
//
//    private _thisThread: [bigint, WebAssembly.Instance, [number, number], [number, number] ][];
//
//    private _workerThread: [bigint, Worker][];
//
//    constructor() {
//        this._messageEncoder = new MessageEncoder();
//
//        this._thisThread = [];
//        this._workerThread = [];
//    }
//
//    genChobitModuleInThisThread(url: string, id: bigint): boolean {
//        const self = this;
//
//        // checks if id exists or not.
//        let found = false;
//
//        self._thisThread.forEach((elm) => {
//            if (elm[0] == id) {found = true;}
//        });
//        self._workerThread.forEach((elm) => {
//            if (elm[0] == id) {found = true;}
//        });
//
//        if (found) {return false;}
//
//        // generates imports.
//        const importObject = {
//            env: {
//                notify_input_buffer: (offset: number, size: number) => {
//                    self._thisThread.forEach((elm) => {
//                        if (elm[0] == id) {
//                            elm[2][0] = offset;
//                            elm[2][1] = size;
//                        }
//                    });
//                },
//
//                notify_output_buffer: (offset: number, size: number) => {
//                    self._thisThread.forEach((elm) => {
//                        if (elm[0] == id) {
//                            elm[3][0] = offset;
//                            elm[3][1] = size;
//                        }
//                    });
//                },
//
//                send: (to: bigint, length: number) => {
//                    self._thisThread.forEach((elm) => {
//                        if (elm[0] != id) {return;}
//
//                        const offset = elm[3][0];
//                        const size = elm[3][1];
//
//                        if (length > size) {return;}
//
//                        const buffer = new Uint8Array(
//                            (elm[1].exports.memory as TypedArray).buffer
//                        );
//
//                        const data = buffer.slice(offset, offset + length);
//                        self.sendData(to, data);
//                    });
//                }
//            }
//        };
//
//        // generates wasm.
//        WebAssembly.instantiateStreaming(fetch(url), importObject).then(
//            (obj) => {
//                self._thisThread.push([id, obj.instance, [0, 0], [0, 0]]);
//            }
//        );
//
//        return true;
//    }
//
//    sendData(to: bigint, data: Uint8Array) {
//        // TODO
//    }
//}
