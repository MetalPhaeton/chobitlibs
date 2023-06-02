import {
    DecodeError,
    MessageBuffer,
    ChobitWasm,
    ChobitWorker,
    ChobitBase
} from "../src/chobit_module_util.ts";

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

    const init = msgBuffer1.encodeInitMsg(BigInt(100), data);
    console.log("decodeInitMsg(): " + msgBuffer2.decodeInitMsg(init));
    try {
        msgBuffer2.decodeRecvMsg(init);
        console.assert(false);
    } catch (e) {
        console.assert(e instanceof DecodeError);
    }

    try {
        msgBuffer2.decodeSendMsg(init);
        console.assert(false);
    } catch (e) {
        console.assert(e instanceof DecodeError);
    }

    console.log("----------------------------------------");

    const recv = msgBuffer1.encodeRecvMsg(BigInt(100), data);
    console.log("decodeRecvMsg(): " + msgBuffer2.decodeRecvMsg(recv));
    try {
        msgBuffer2.decodeInitMsg(recv);
        console.assert(false);
    } catch (e) {
        console.log("mark");
        console.assert(e instanceof DecodeError);
    }

    try {
        msgBuffer2.decodeSendMsg(recv);
        console.assert(false);
    } catch (e) {
        console.assert(e instanceof DecodeError);
    }

    console.log("----------------------------------------");

    const send = msgBuffer1.encodeSendMsg(BigInt(100), data);
    console.log("decodeSendMsg(): " + msgBuffer2.decodeSendMsg(send));
    try {
        msgBuffer2.decodeInitMsg(send);
        console.assert(false);
    } catch (e) {
        console.assert(e instanceof DecodeError);
    }

    try {
        msgBuffer2.decodeRecvMsg(send);
        console.assert(false);
    } catch (e) {
        console.assert(e instanceof DecodeError);
    }
}

async function test2() {
    await ChobitWasm.instantiate(
        BigInt(100),
        new URL("./test_wasm.wasm", import.meta.url),
        (to, data) => {
            console.log("send to: " + to);
            console.log("send data: " + new TextDecoder().decode(data));
        }
    ).then((chobitWasm) => {
        chobitWasm.postData(
            BigInt(777),
            new TextEncoder().encode("Hello from test2!")
        );
    });
}

function test3() {
    const worker = new ChobitWorker(
        1024,
        BigInt(111),
        new URL("./chobit_module_util_tests_2.ts", import.meta.url),
        new URL("../tests/test_wasm.wasm", import.meta.url),
        (to, data) => {
            console.log("send' to: " + to);
            console.log("send' data: " + new TextDecoder().decode(data));

            worker.terminate();
        }
    );

    worker.postData(
        BigInt(1000),
        new TextEncoder().encode("Hello from test3!")
    );
}

function test4() {
    const base = new ChobitBase((from, data) => {
        console.log("ChobitBase receive from " + from);
        console.log("data: " + new TextDecoder().decode(data));

        base.terminate(BigInt(100));
        console.assert(base.numWorkers() == 2);

        base.terminate(BigInt(2));
        console.assert(base.numWorkers() == 1);

        base.terminate(BigInt(1));
        console.assert(base.numWorkers() == 0);
    });

    base.addWorker(
        1024,
        BigInt(2),
        new URL("./chobit_module_util_tests_2.ts", import.meta.url),
        new URL("../tests/test_wasm.wasm", import.meta.url),
    );

    base.addWorker(
        1024,
        BigInt(1),
        new URL("./chobit_module_util_tests_2.ts", import.meta.url),
        new URL("../tests/test_wasm.wasm", import.meta.url),
    );

    base.postData(
        BigInt(2),
        BigInt(99),
        new TextEncoder().encode("Let's Go!")
    );
}

function test5() {
    const base = new ChobitBase((from, data) => {
        console.log("ChobitBase receive from " + from);
        console.log("data: " + new TextDecoder().decode(data));
    });

    base.addWorker(
        1024,
        BigInt(2),
        new URL("./chobit_module_util_tests_2.ts", import.meta.url),
        new URL("../tests/test_wasm.wasm", import.meta.url),
    );

    base.addWorker(
        1024,
        BigInt(1),
        new URL("./chobit_module_util_tests_2.ts", import.meta.url),
        new URL("../tests/test_wasm.wasm", import.meta.url),
    );

    base.broadcastData(
        BigInt(99),
        new TextEncoder().encode("Let's broadcast!")
    );

    setTimeout(() => {
        base.terminateAll();
        console.assert(base.numWorkers() == 0);
    }, 2000);
}

test1();

test2();

test3();

test4();

test5();
