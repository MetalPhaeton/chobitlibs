import {MessageBuffer, ChobitWASM} from "./chobit_module_util.ts";

function test1Core(msgBuffer2: MessageBuffer, msg: ArrayBuffer) {
    console.log("decode***() ----------------------------");
    console.log("decodeInitMsg(): " + msgBuffer2.decodeInitMsg(msg));
    console.log("decodeRecvMsg(): " + msgBuffer2.decodeRecvMsg(msg));
    console.log("decodeSendMsg(): " + msgBuffer2.decodeSendMsg(msg));
    console.log("decodeWASMOKMsg(): " + msgBuffer2.decodeWASMOKMsg(msg));
}

function test1() {
    const msgBuffer1 = new MessageBuffer(10);
    console.log(msgBuffer1);
    console.log(msgBuffer1.toMsgID("init"));
    console.log(msgBuffer1.initID);
    console.log(msgBuffer1.recvID);
    console.log(msgBuffer1.sendID);
    console.log(msgBuffer1.wasmOKID);

    const msgBuffer2 = new MessageBuffer(10);

    const data = new Uint8Array([
        1, 2, 3, 4, 5, 7, 8, 9, 10,
        1, 2, 3, 4, 5, 7, 8, 9, 10,
        1, 2, 3, 4, 5, 7, 8, 9, 10,
        1, 2, 3, 4, 5, 7, 8, 9, 10,
        1, 2, 3, 4, 5, 7, 8, 9, 10,
        1, 2, 3, 4, 5, 7, 8, 9, 10,
        1, 2, 3, 4, 5, 7, 8, 9, 10,
        1, 2, 3, 4, 5, 7, 8, 9, 10,
        1, 2, 3, 4, 5, 7, 8, 9, 10,
        1, 2, 3, 4, 5, 7, 8, 9, 10
    ]);

    console.log("----------------------------------------");

    const init = msgBuffer1.encodeInitMsg(100n, data);
    if (init) {
        test1Core(msgBuffer2, init);
    } else {
        console.log("init is null");
    }

    console.log("----------------------------------------");

    const recv = msgBuffer1.encodeRecvMsg(100n, data);
    if (recv) {
        test1Core(msgBuffer2, recv);
    } else {
        console.log("recv is null");
    }

    console.log("----------------------------------------");

    const send = msgBuffer1.encodeSendMsg(100n, data);
    if (send) {
        test1Core(msgBuffer2, send);
    } else {
        console.log("send is null");
    }

    console.log("----------------------------------------");

    const wasmOK = msgBuffer1.encodeWASMOKMsg(100n, data);
    if (wasmOK) {
        test1Core(msgBuffer2, wasmOK);
    } else {
        console.log("wasmOK is null");
    }
}

function test2() {
    const wasm = new ChobitWASM();

    const imports = wasm.genDefaultImports((to, data) => {
        console.log(
            "send_to: " + to.toString()
                + ", send_data: " + (new TextDecoder()).decode(data)
        );
    });

    const promise = wasm.genWASM(
        new URL("../tests/test_wasm.wasm", import.meta.url),
        111n,
        imports
    );

    if (promise) {
        promise.then(() => {
            wasm.input(222n, (new TextEncoder()).encode("Alice plays chess."))
        });
    } else {
        console.log("promise is null");
    }
}

console.log("test1 ==================================")
test1();

console.log("test2 ==================================")
test2();
