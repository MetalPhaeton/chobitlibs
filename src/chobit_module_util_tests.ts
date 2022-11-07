import {MessageEncoder} from "./chobit_module_util.ts";

function test1Core(encoder: MessageEncoder, msg: Uint8Array) {
    console.log("decodeMsgID(): " + encoder.decodeMsgID(msg));
    console.log("is***() --------------------------------")
    console.log("isInitMsg(): " + encoder.isInitMsg(msg));
    console.log("isRecvMsg(): " + encoder.isRecvMsg(msg));
    console.log(
        "isNotifyInputBufferMsg(): " + encoder.isNotifyInputBufferMsg(msg)
    );
    console.log(
        "isNotifyOutputBufferMsg(): " + encoder.isNotifyOutputBufferMsg(msg)
    );
    console.log("isSendMsg(): " + encoder.isSendMsg(msg));

    console.log("decode***() ----------------------------");
    console.log("decodeInitMsg(): " + encoder.decodeInitMsg(msg));
    console.log("decodeRecvMsg(): " + encoder.decodeRecvMsg(msg));
    console.log(
        "decodeNotifyInputBufferMsg(): "
            + encoder.decodeNotifyInputBufferMsg(msg)
    );
    console.log(
        "decodeNotifyOutputBufferMsg(): "
            + encoder.decodeNotifyOutputBufferMsg(msg)
    );
    console.log("decodeSendMsg(): " + encoder.decodeSendMsg(msg));
}

function test1() {
    const encoder = new MessageEncoder();
    console.log(encoder);
    console.log(encoder.toID("init"));
    console.log(encoder.initID);
    console.log(encoder.recvID);
    console.log(encoder.notifyInputBufferID);
    console.log(encoder.notifyOutputBufferID);
    console.log(encoder.sendID);

    console.log("----------------------------------------")

    const init = encoder.encodeInitMsg(100n);
    test1Core(encoder, init);

    const data = new Uint8Array([1, 2, 3, 4, 5]);

    console.log("----------------------------------------")

    const recv = encoder.encodeRecvMsg(100n, data);
    test1Core(encoder, recv);

    console.log("----------------------------------------")

    const inputMsg = encoder.encodeNotifyInputBufferMsg(123, 456);
    test1Core(encoder, inputMsg);

    console.log("----------------------------------------")

    const outputMsg = encoder.encodeNotifyOutputBufferMsg(123, 456);
    test1Core(encoder, outputMsg);

    console.log("----------------------------------------")

    const send = encoder.encodeSendMsg(100n, data);
    test1Core(encoder, send);
}

test1();
