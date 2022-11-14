import {ChobitSexpr} from "./chobit_sexpr.ts";

function chobitSexprTest1() {
    const data = new Uint8Array([1, 2, 3, 4, 5]);

    const sexpr = ChobitSexpr.genAtom(data);
    console.assert(sexpr.isAtom());
    console.assert(!sexpr.isCons());

    const atom = sexpr.atom();
    if (atom) {
        console.assert(data.toString() == atom.toString());
    }
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

    let current: ChobitSexpr | null = sexpr;
    while (current) {
        const car = current.car();
        if (car) {
            console.assert(!current.isAtom());
            console.assert(current.isCons());

            const data = car.atom();
            if (data) {
                console.log(data);
                current = current.cdr();
            } else {
                console.error("Error 1");
                break;
            }
        } else {
            break;
        }
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
        const value = 0x1111111111111111n;
        const atom = ChobitSexpr.genI64(value);
        console.assert(atom.readI64() == value);
        atom.writeI64(value + 1n);
        console.assert(atom.readI64() == (value + 1n));
    }

    {
        const value = 0x1111111111111111n;
        const atom = ChobitSexpr.genU64(value);
        console.assert(atom.readU64() == value);
        atom.writeU64(value + 1n);
        console.assert(atom.readU64() == (value + 1n));
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

console.log("chobitSexprTest1 ===========================================")
chobitSexprTest1();

console.log("chobitSexprTest2 ===========================================")
chobitSexprTest2();

console.log("chobitSexprTest3 ===========================================")
chobitSexprTest3();

console.log("chobitSexprTest4 ===========================================")
chobitSexprTest4();
console.log("============================================================")
