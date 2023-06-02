import {
    NotAtomError,
    NotConsError,
    ReadError,
    WriteError,
    ValueType,
    ChobitSexpr,
    Iter
} from "../src/chobit_sexpr.ts";

function chobitSexprTest1() {
    const data = new Uint8Array([1, 2, 3, 4, 5]);

    const sexpr = ChobitSexpr.genAtom(data);
    console.assert(sexpr.isAtom());
    console.assert(!sexpr.isCons());

    const atom = sexpr.atom();
    console.assert(data.toString() == atom.toString());
}

function chobitSexprTest2() {
    const data1 = new Uint8Array([1, 2, 3, 4, 5]);
    const data2 = new Uint8Array([1, 2, 3, 4, 5, 6, 7]);

    const car = ChobitSexpr.genAtom(data1);
    const cdr = ChobitSexpr.genAtom(data2);

    const sexpr = ChobitSexpr.genCons(car, cdr);
    console.assert(!sexpr.isAtom());
    console.assert(sexpr.isCons());
    const car2 = sexpr.car();
    const cdr2 = sexpr.cdr();

    if (car2 && cdr2) {
        const carAtom = car2.atom();
        const cdrAtom = cdr2.atom();
        if (carAtom && cdrAtom) {
            console.assert(carAtom.toString() == data1.toString());
            console.assert(cdrAtom.toString() == data2.toString());
        }
    }
}

function chobitSexprTest3() {
    const data1 = new Uint8Array([1, 2, 3, 4, 5]);
    const data2 = new Uint8Array([1, 2, 3, 4, 5, 6, 7]);
    const data3 = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9]);

    const item1 = ChobitSexpr.genAtom(data1);
    const item2 = ChobitSexpr.genAtom(data2);
    const item3 = ChobitSexpr.genAtom(data3);
    const nil = ChobitSexpr.genAtom(new Uint8Array(0));

    const sexpr = ChobitSexpr.genCons(
        item1,
        ChobitSexpr.genCons(
            item2,
            ChobitSexpr.genCons(
                item3,
                nil
            )
        )
    );
    console.log(sexpr);

    let current: ChobitSexpr = sexpr;
    while (current) {
        try {
            const car = current.car();

            console.assert(!current.isAtom());
            console.assert(current.isCons());

            const data = car.atom();
            console.log(data);
        } catch (e) {
            console.assert(e instanceof NotConsError);
            break;
        }

        current = current.cdr();
    }

    console.log(current);
}

function chobitSexprTest4() {
    {
        const value = 0x11;
        const atom = ChobitSexpr.genI8(value);
        console.assert(atom.readI8() == value);
        atom.writeI8(value + 1);
        console.assert(atom.readI8() == (value + 1));
    }

    {
        const value = 0x11;
        const atom = ChobitSexpr.genU8(value);
        console.assert(atom.readU8() == value);
        atom.writeU8(value + 1);
        console.assert(atom.readU8() == (value + 1));
    }

    {
        const value = 0x1111;
        const atom = ChobitSexpr.genI16(value);
        console.assert(atom.readI16() == value);
        atom.writeI16(value + 1);
        console.assert(atom.readI16() == (value + 1));
    }

    {
        const value = 0x1111;
        const atom = ChobitSexpr.genU16(value);
        console.assert(atom.readU16() == value);
        atom.writeU16(value + 1);
        console.assert(atom.readU16() == (value + 1));
    }

    {
        const value = 0x11111111;
        const atom = ChobitSexpr.genI32(value);
        console.assert(atom.readI32() == value);
        atom.writeI32(value + 1);
        console.assert(atom.readI32() == (value + 1));
    }

    {
        const value = 0x11111111;
        const atom = ChobitSexpr.genU32(value);
        console.assert(atom.readU32() == value);
        atom.writeU32(value + 1);
        console.assert(atom.readU32() == (value + 1));
    }

    {
        const value = BigInt("0x1111111111111111");
        const atom = ChobitSexpr.genI64(value);
        console.assert(atom.readI64() == value);
        atom.writeI64(value + BigInt(1));
        console.assert(atom.readI64() == (value + BigInt(1)));
    }

    {
        const value = BigInt("0x1111111111111111");
        const atom = ChobitSexpr.genU64(value);
        console.assert(atom.readU64() == value);
        atom.writeU64(value + BigInt(1));
        console.assert(atom.readU64() == (value + BigInt(1)));
    }

    {
        const value = 0.1234;
        const tmp = new Float32Array([value]);
        const atom = ChobitSexpr.genF32(tmp[0]);
        console.assert(atom.readF32() == tmp[0]);

        const tmp2 = new Float32Array([1.0]);
        const tmp3 = new Float32Array([tmp[0] + tmp2[0]]);
        atom.writeF32(tmp3[0]);
        console.assert(atom.readF32() == tmp3[0]);
    }

    {
        const value = 0.1234;
        const atom = ChobitSexpr.genF64(value);
        console.assert(atom.readF64() == value);
        atom.writeF64(value + 1.0);
        console.assert(atom.readF64() == (value + 1.0));
    }

    {
        const value = "Hello World";
        const atom = ChobitSexpr.genString(value);
        console.assert(atom.readString() == value);
    }
}

function chobitSexprTest5() {
    const data1 = new Uint8Array([1, 2, 3, 4, 5]);
    const data2 = new Uint8Array([1, 2, 3, 4, 5, 6, 7]);
    const data3 = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9]);

    const item1 = ChobitSexpr.genAtom(data1);
    const item2 = ChobitSexpr.genAtom(data2);
    const item3 = ChobitSexpr.genAtom(data3);
    const nil = ChobitSexpr.genAtom(new Uint8Array(0));

    const sexpr = ChobitSexpr.genCons(
        item1,
        ChobitSexpr.genCons(
            item2,
            ChobitSexpr.genCons(
                item3,
                nil
            )
        )
    );

    for (const item of sexpr.iter()) {
        console.log(item.atom());
    }
}

function chobitSexprTest6() {
    const data1 = new Uint8Array([1, 2, 3, 4, 5]);
    const data2 = new Uint8Array([1, 2, 3, 4, 5, 6, 7]);
    const data3 = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9]);

    const item1 = ChobitSexpr.genAtom(data1);
    const item2 = ChobitSexpr.genAtom(data2);
    const item3 = ChobitSexpr.genAtom(data3);
    const nil = ChobitSexpr.genAtom(new Uint8Array([]));

    const sexpr = ChobitSexpr.genCons(
        item1,
        ChobitSexpr.genCons(
            item2,
            ChobitSexpr.genCons(
                item3,
                nil
            )
        )
    );

    const ret1 = sexpr.carCdr();
    const [car1, cdr1] = ret1;
    const atom1 = car1.atom();
    console.assert(atom1.length == data1.length);
    for (let i in atom1) {
        console.assert(atom1[i] == data1[i]);
    }

    const ret2 = cdr1.carCdr();
    const [car2, cdr2] = ret2;
    const atom2 = car2.atom();
    console.assert(atom2.length == data2.length);
    for (let i in atom2) {
        console.assert(atom2[i] == data2[i]);
    }

    const ret3 = cdr2.carCdr();
    const [car3, cdr3] = ret3;
    const atom3 = car3.atom();
    console.assert(atom3.length == data3.length);
    for (let i in atom3) {
        console.assert(atom3[i] == data3[i]);
    }

    const atom4 = cdr3.atom();
    console.assert(atom4.length == 0);
}

function chobitSexprTest7() {
    {
        const value = 11;
        const sexpr = ChobitSexpr.genI8(value);
        const atom = sexpr.readI8();
        console.assert(atom == value);
    }

    {
        const value = 12;
        const sexpr = ChobitSexpr.genU8(value);
        const atom = sexpr.readU8();
        console.assert(atom == value);
    }

    {
        const value = 13;
        const sexpr = ChobitSexpr.genI16(value);
        const atom = sexpr.readI16();
        console.assert(atom == value);
    }

    {
        const value = 14;
        const sexpr = ChobitSexpr.genU16(value);
        const atom = sexpr.readU16();
        console.assert(atom == value);
    }

    {
        const value = 15;
        const sexpr = ChobitSexpr.genI32(value);
        const atom = sexpr.readI32();
        console.assert(atom == value);
    }

    {
        const value = 16;
        const sexpr = ChobitSexpr.genU32(value);
        const atom = sexpr.readU32();
        console.assert(atom == value);
    }

    {
        const value = BigInt(17);
        const sexpr = ChobitSexpr.genI64(value);
        const atom = sexpr.readI64();
        console.assert(atom == value);
    }

    {
        const value = BigInt(18);
        const sexpr = ChobitSexpr.genU64(value);
        const atom = sexpr.readU64();
        console.assert(atom == value);
    }

    {
        const value = new Float32Array(1);
        value[0] = 19.1;
        const sexpr = ChobitSexpr.genF32(value[0]);
        const atom = sexpr.readF32();
        console.assert(atom == value[0]);
    }

    {
        const value = new Float64Array(1);
        value[0] = 20.2;
        const sexpr = ChobitSexpr.genF64(value[0]);
        const atom = sexpr.readF64();
        console.assert(atom == value[0]);
    }
}

function chobitSexprTest8() {
    const data = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    const sexpr = ChobitSexpr.genAtom(data);

    try {
        const atom = sexpr.readI8();

        console.assert(false);
    } catch (e) {
        if (e instanceof ReadError) {
            console.assert(e.valueType == ValueType.I8);
        } else {
            console.assert(false);
        }
    }

    try {
        const atom = sexpr.readU8();

        console.assert(false);
    } catch (e) {
        if (e instanceof ReadError) {
            console.assert(e.valueType == ValueType.U8);
        } else {
            console.assert(false);
        }
    }

    try {
        const atom = sexpr.readI16();

        console.assert(false);
    } catch (e) {
        if (e instanceof ReadError) {
            console.assert(e.valueType == ValueType.I16);
        } else {
            console.assert(false);
        }
    }

    try {
        const atom = sexpr.readU16();

        console.assert(false);
    } catch (e) {
        if (e instanceof ReadError) {
            console.assert(e.valueType == ValueType.U16);
        } else {
            console.assert(false);
        }
    }

    try {
        const atom = sexpr.readI32();

        console.assert(false);
    } catch (e) {
        if (e instanceof ReadError) {
            console.assert(e.valueType == ValueType.I32);
        } else {
            console.assert(false);
        }
    }

    try {
        const atom = sexpr.readU32();

        console.assert(false);
    } catch (e) {
        if (e instanceof ReadError) {
            console.assert(e.valueType == ValueType.U32);
        } else {
            console.assert(false);
        }
    }

    try {
        const atom = sexpr.readI64();

        console.assert(false);
    } catch (e) {
        if (e instanceof ReadError) {
            console.assert(e.valueType == ValueType.I64);
        } else {
            console.assert(false);
        }
    }

    try {
        const atom = sexpr.readU64();

        console.assert(false);
    } catch (e) {
        if (e instanceof ReadError) {
            console.assert(e.valueType == ValueType.U64);
        } else {
            console.assert(false);
        }
    }

    try {
        const atom = sexpr.readF32();

        console.assert(false);
    } catch (e) {
        if (e instanceof ReadError) {
            console.assert(e.valueType == ValueType.F32);
        } else {
            console.assert(false);
        }
    }

    try {
        const atom = sexpr.readF64();

        console.assert(false);
    } catch (e) {
        if (e instanceof ReadError) {
            console.assert(e.valueType == ValueType.F64);
        } else {
            console.assert(false);
        }
    }
}

console.log("chobitSexprTest1 ===========================================")
chobitSexprTest1();

console.log("chobitSexprTest2 ===========================================")
chobitSexprTest2();

console.log("chobitSexprTest3 ===========================================")
chobitSexprTest3();

console.log("chobitSexprTest4 ===========================================")
chobitSexprTest4();

console.log("chobitSexprTest5 ===========================================")
chobitSexprTest5();

console.log("chobitSexprTest6 ===========================================")
chobitSexprTest6();

console.log("chobitSexprTest7 ===========================================")
chobitSexprTest7();

console.log("chobitSexprTest8 ===========================================")
chobitSexprTest8();
console.log("============================================================")
