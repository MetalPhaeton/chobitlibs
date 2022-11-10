import {MessageEncoder, ChobitWASM} from "./chobit_module_util.ts";

function test1Core(decoder: MessageEncoder, msg: ArrayBuffer) {
    console.log("decode***() ----------------------------");
    console.log("decodeInitMsg(): " + decoder.decodeInitMsg(msg));
    console.log("decodeRecvMsg(): " + decoder.decodeRecvMsg(msg));
    console.log("decodeSendMsg(): " + decoder.decodeSendMsg(msg));
    console.log("decodeWASMOKMsg(): " + decoder.decodeWASMOKMsg(msg));
}

function test1() {
    const encoder = new MessageEncoder(1000);
    console.log(encoder);
    console.log(encoder.toMsgID("init"));
    console.log(encoder.initID);
    console.log(encoder.recvID);
    console.log(encoder.sendID);
    console.log(encoder.wasmOKID);

    const decoder = new MessageEncoder(1000);

    const data = new Uint8Array([1, 2, 3, 4, 5]);

    console.log("----------------------------------------")

    const init = encoder.encodeInitMsg(100n, data);
    test1Core(decoder, init);

    console.log("----------------------------------------")

    const recv = encoder.encodeRecvMsg(100n, data);
    test1Core(decoder, recv);

    console.log("----------------------------------------")

    const send = encoder.encodeSendMsg(100n, data);
    test1Core(decoder, send);

    console.log("----------------------------------------")

    const wasmOK = encoder.encodeWASMOKMsg(100n, data);
    test1Core(decoder, wasmOK);
}

//function test2() {
//    const wasm = new ChobitWASM();
//
//    const imports = wasm.genDefaultImports((to, data) => {
//        console.log(
//            "send_to: " + to.toString()
//                + ", send_data: " + (new TextDecoder()).decode(data)
//        );
//    });
//
//    wasm.genWASM(
//        new URL("../tests/test_wasm.wasm", import.meta.url),
//        111n,
//        imports
//    ).then(() => {
//        wasm.input(222n, (new TextEncoder()).encode("Alice plays chess."))
//    });
//
//}

test1();
