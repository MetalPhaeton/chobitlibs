import {
    MessageBuffer,
    ChobitWasm,
    //ChobitWorkerChannel,
    //ChobitWorkerBase
} from "./chobit_module_util.ts";

function test1Core(msgBuffer2: MessageBuffer, msg: ArrayBuffer) {
    console.log("decode***() ----------------------------");
    console.log("decodeInitMsg(): " + msgBuffer2.decodeInitMsg(msg));
    console.log("decodeRecvMsg(): " + msgBuffer2.decodeRecvMsg(msg));
    console.log("decodeSendMsg(): " + msgBuffer2.decodeSendMsg(msg));
}

function test1() {
    const msgBuffer1 = new MessageBuffer(10);
    console.log(msgBuffer1);
    console.log(msgBuffer1.toMsgID("init"));
    console.log(msgBuffer1.initID);
    console.log(msgBuffer1.recvID);
    console.log(msgBuffer1.sendID);

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
}

async function test2() {
    await ChobitWasm.instantiate(
        100n,
        new URL("../tests/test_wasm.wasm", import.meta.url),
        (to, data) => {
            console.log("send to: " + to);
            console.log("send data: " + new TextDecoder().decode(data));
        }
    ).then((chobitWasm) => {
        chobitWasm.postData(
            777n,
            new TextEncoder().encode("Hello from test2!")
        );
    });
}

//function test3() {
//    const channel = new ChobitWorkerChannel(
//        1024,
//        111n,
//        new URL("./chobit_module_util_tests_2.ts", import.meta.url),
//        new URL("../tests/test_wasm.wasm", import.meta.url),
//        (from, data) => {
//            console.log("wasmOK!");
//            console.log("from: " + from);
//            console.log("data: " + new TextDecoder().decode(data));
//
//            channel.postData(
//                222n,
//                new TextEncoder().encode("From ChobitWorkerChannel!")
//            );
//        },
//        (from, data) => {
//            console.log("from: " + from);
//            console.log("data: " + new TextDecoder().decode(data));
//
//            channel.terminateWorker();
//        }
//    );
//}
//
//function test4() {
//    const base = new ChobitWorkerBase((from, data) => {
//        console.log("ChobitWorkerBase receive from " + from);
//        console.log("data: " + new TextDecoder().decode(data));
//    });
//
//    base.addWorker(
//        1024,
//        2n,
//        new URL("./chobit_module_util_tests_2.ts", import.meta.url),
//        new URL("../tests/test_wasm.wasm", import.meta.url),
//    );
//
//    base.addWorker(
//        1024,
//        1n,
//        new URL("./chobit_module_util_tests_2.ts", import.meta.url),
//        new URL("../tests/test_wasm.wasm", import.meta.url),
//    );
//
//    setTimeout(() => {
//        for (const ch of base.channels) {
//            if (ch.channel.moduleID == 2n) {
//                ch.channel.postData(
//                    base.moduleID,
//                    new TextEncoder().encode("Hello")
//                );
//            }
//        }
//    }, 1000);
//}
//
test1();

test2();
//
//test3();
//
//test4();
