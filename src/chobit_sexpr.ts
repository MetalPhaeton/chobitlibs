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

const SEXPR_HEADER_LEN: number = 4 as const;

/**
 * Error of header of ChobitSexpr.
 */
export class SexprHeaderError extends Error {
    constructor() {
        super("SexprHeaderError");
    }
}

/**
 * It is not ChobitSexpr.
 */
export class NotSexprError extends Error {
    constructor() {
        super("NotSexprError");
    }
}

/**
 * It is not atom of ChobitSexpr.
 */
export class NotAtomError extends Error {
    constructor() {
        super("NotAtomError");
    }
}

/**
 * It is not cons of ChobitSexpr.
 */
export class NotConsError extends Error {
    constructor() {
        super("NotConsError");
    }
}

function getValueTypeStr(valueType: ValueType): String {
    switch (valueType) {
        case ValueType.I8:
            return "ValueType.I8";
        case ValueType.U8:
            return "ValueType.U8";
        case ValueType.I16:
            return "ValueType.I16";
        case ValueType.U16:
            return "ValueType.U16";
        case ValueType.I32:
            return "ValueType.I32";
        case ValueType.U32:
            return "ValueType.U32";
        case ValueType.I64:
            return "ValueType.I64";
        case ValueType.U64:
            return "ValueType.U64";
        case ValueType.F32:
            return "ValueType.F32";
        case ValueType.F64:
            return "ValueType.F64";
        case ValueType.Str:
            return "ValueType.Str";
        default:
            return "Others";
    }
}

/**
 * Error in reading value.
 */
export class ReadError extends Error {
    public valueType: ValueType;

    constructor(valueType: ValueType) {
        super("ReadError(" + getValueTypeStr(valueType) + ")");

        this.valueType = valueType;
    }
}

/**
 * Error in writing value.
 */
export class WriteError extends Error {
    public valueType: ValueType;

    constructor(valueType: ValueType) {
        super("WriteError(" + getValueTypeStr(valueType) + ")");

        this.valueType = valueType;
    }
}

/// Value for ReadError and WriteError.
export enum ValueType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    Str
}

/**
 * ChobitSexpr for Typescript.
 */
export class ChobitSexpr {
    #body: Uint8Array;

    /**
     * Constructor.
     *
     * @param body Body of ChobitSexpr.
     */
    constructor(body: Uint8Array) {
        this.#body = body;
    }

    /**
     * Gets body.
     *
     * @return Body.
     */
    get body(): Uint8Array {return this.#body;}

    /**
     * Generates Atom header.
     *
     * @param size A size of payload.
     * @return Header.
     */
    static genAtomHeader(size: number): number {
        return size & 0x7fffffff;
    }

    /**
     * Generates cons header.
     *
     * @param carSize A size of car.
     * @return Header.
     */
    static genConsHeader(carSize: number): number {
        return (carSize & 0x7fffffff) | 0x80000000;
    }

    /**
     * Generates Atom.
     *
     * @param data payload.
     * @return Atom.
     */
    static genAtom(data: Uint8Array): ChobitSexpr {
        const sexpr = new Uint8Array(data.length + SEXPR_HEADER_LEN);
        const header = ChobitSexpr.genAtomHeader(data.length);

        const view = new DataView(sexpr.buffer);
        view.setUint32(sexpr.byteOffset, header, true);

        sexpr.set(data, SEXPR_HEADER_LEN);

        return new ChobitSexpr(sexpr);
    }

    /**
     * Generates Cons.
     *
     * @param car Car.
     * @param cdr Cdr.
     * @return Cons.
     */
    static genCons(car: ChobitSexpr, cdr: ChobitSexpr): ChobitSexpr {
        const carLength = car.#body.length;
        const cdrLength = cdr.#body.length;

        const sexpr = new Uint8Array(carLength + cdrLength + SEXPR_HEADER_LEN);
        const header = ChobitSexpr.genConsHeader(carLength);

        const view = new DataView(sexpr.buffer);
        view.setUint32(sexpr.byteOffset, header, true);

        sexpr.set(car.#body, SEXPR_HEADER_LEN);
        sexpr.set(cdr.#body, carLength + SEXPR_HEADER_LEN);

        return new ChobitSexpr(sexpr);
    }

    #header(): number {
        if (this.#body.length < SEXPR_HEADER_LEN) {
            throw new SexprHeaderError();
        }

        return new DataView(this.#body.buffer).getUint32(
            this.#body.byteOffset,
            true
        );
    }

    static #flag(header: number): number {
        return header & 0x80000000;
    }

    static #size(header: number): number {
        return header & 0x7fffffff;
    }

    /**
     * Whether this sexpr is atom or not.
     *
     * @return If this sexpr is atom, true.
     */
    isAtom(): boolean {
        const header = this.#header();
        if (header != null) {
            return ChobitSexpr.#flag(header) == 0;
        }

        return false;
    }

    /**
     * Whether this sexpr is cons or not.
     *
     * @return If this sexpr is cons, true.
     */
    isCons(): boolean {
        const header = this.#header();
        if (header != null) {
            return ChobitSexpr.#flag(header) != 0;
        }

        return false;
    }

    /**
     * Gets atom.
     *
     * @return Payload of atom.
     * @throws {SexprHeaderError}.
     * @throws {NotAtomError}.
     * @throws {NotSexprError}.
     */
    atom(): Uint8Array {
        const header = this.#header();

        if (ChobitSexpr.#flag(header) != 0) {
            throw new NotAtomError();
        }

        const size = ChobitSexpr.#size(header);
        if ((SEXPR_HEADER_LEN + size) > this.#body.length) {
            throw new NotSexprError();
        }

        return new Uint8Array(
            this.#body.buffer,
            this.#body.byteOffset + SEXPR_HEADER_LEN,
            size
        );
    }

    /**
     * Gets car.
     *
     * @return Sexpr of car.
     * @throws {SexprHeaderError}.
     * @throws {NotConsError}.
     * @throws {NotSexprError}.
     */
    car(): ChobitSexpr {
        const header = this.#header();

        if (ChobitSexpr.#flag(header) == 0) {
            throw new NotConsError();
        }

        const size = ChobitSexpr.#size(header);
        if ((SEXPR_HEADER_LEN + size) > this.#body.length) {
            throw new NotSexprError();
        }

        return new ChobitSexpr(new Uint8Array(
            this.#body.buffer,
            this.#body.byteOffset + SEXPR_HEADER_LEN,
            size
        ));
    }

    /**
     * Gets car.
     *
     * @return Sexpr of cdr.
     * @throws {SexprHeaderError}.
     * @throws {NotConsError}.
     * @throws {NotSexprError}.
     */
    cdr(): ChobitSexpr {
        const header = this.#header();

        if (ChobitSexpr.#flag(header) == 0) {
            throw new NotConsError();
        }

        const size = ChobitSexpr.#size(header);
        if ((SEXPR_HEADER_LEN + size) > this.#body.length) {
            throw new NotSexprError();
        }

        return new ChobitSexpr(new Uint8Array(
            this.#body.buffer,
            this.#body.byteOffset + SEXPR_HEADER_LEN + size,
        ));
    }

    /**
     * Gets car and cdr.
     *
     * @return Sexpr of [car, cdr].
     * @throws {SexprHeaderError}.
     * @throws {NotConsError}.
     * @throws {NotSexprError}.
     */
    carCdr(): [ChobitSexpr, ChobitSexpr] {
        const header = this.#header();

        if (ChobitSexpr.#flag(header) == 0) {
            throw new NotConsError();
        }

        const size = ChobitSexpr.#size(header);
        if ((SEXPR_HEADER_LEN + size) > this.#body.length) {
            throw new NotSexprError();
        }

        return [
            new ChobitSexpr(new Uint8Array(
                this.#body.buffer,
                this.#body.byteOffset + SEXPR_HEADER_LEN,
                size
            )),
            new ChobitSexpr(new Uint8Array(
                this.#body.buffer,
                this.#body.byteOffset + SEXPR_HEADER_LEN + size,
            ))
        ];
    }

    /**
     * Generates Nil.
     *
     * @return Nil.
     */
    static genNil(): ChobitSexpr {
        return ChobitSexpr.genAtom(new Uint8Array(0));
    }

    /**
     * Reads Int8 value.
     *
     * @return Atom as Int8.
     * @throws {ReadError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    readI8(): number {
        const atom = this.atom();

        if (atom.length == 1) {
            return new DataView(atom.buffer).getInt8(atom.byteOffset);
        }

        throw new ReadError(ValueType.I8);
    }

    /**
     * Writes Int8 value.
     *
     * @param value A value.
     * @throws {WriteError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    writeI8(value: number) {
        const atom = this.atom();

        if (atom.length == 1) {
            new DataView(atom.buffer)
                .setInt8(atom.byteOffset, value);
        } else {
            throw new WriteError(ValueType.I8)
        }
    }

    /**
     * Reads Uint8 value.
     *
     * @return Atom Uint8.
     * @throws {ReadError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    readU8(): number {
        const atom = this.atom();

        if (atom.length == 1) {
            return new DataView(atom.buffer).getUint8(atom.byteOffset);
        }

        throw new ReadError(ValueType.U8);
    }

    /**
     * Writes Uint8 value.
     *
     * @param value A value.
     * @throws {WriteError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    writeU8(value: number) {
        const atom = this.atom();

        if (atom.length == 1) {
            new DataView(atom.buffer)
                .setUint8(atom.byteOffset, value);
        } else {
            throw new WriteError(ValueType.U8)
        }
    }

    /**
     * Reads Int16 value.
     *
     * @return Atom as Int16.
     * @throws {ReadError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    readI16(): number {
        const atom = this.atom();

        if (atom.length == 2) {
            return new DataView(atom.buffer)
                .getInt16(atom.byteOffset, true);
        }

        throw new ReadError(ValueType.I16);
    }

    /**
     * Writes Int16 value.
     *
     * @param value A value.
     * @throws {WriteError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    writeI16(value: number) {
        const atom = this.atom();

        if (atom.length == 2) {
            new DataView(atom.buffer)
                .setInt16(atom.byteOffset, value, true);
        } else {
            throw new WriteError(ValueType.I16)
        }
    }

    /**
     * Reads Uint16 value.
     *
     * @return Atom as Uint16.
     * @throws {ReadError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    readU16(): number {
        const atom = this.atom();

        if (atom.length == 2) {
            return new DataView(atom.buffer)
                .getUint16(atom.byteOffset, true);
        }

        throw new ReadError(ValueType.U16);
    }

    /**
     * Writes Uint16 value.
     *
     * @param value A value.
     * @throws {WriteError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    writeU16(value: number) {
        const atom = this.atom();

        if (atom.length == 2) {
            new DataView(atom.buffer)
                .setUint16(atom.byteOffset, value, true);
        } else {
            throw new WriteError(ValueType.U16)
        }
    }

    /**
     * Reads Int32 value.
     *
     * @return Atom as Int32.
     * @throws {ReadError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    readI32(): number {
        const atom = this.atom();

        if (atom.length == 4) {
            return new DataView(atom.buffer)
                .getInt32(atom.byteOffset, true);
        }

        throw new ReadError(ValueType.I32);
    }

    /**
     * Writes Int32 value.
     *
     * @param value A value.
     * @throws {WriteError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    writeI32(value: number) {
        const atom = this.atom();

        if (atom.length == 4) {
            new DataView(atom.buffer)
                .setInt32(atom.byteOffset, value, true);
        } else {
            throw new WriteError(ValueType.I32);
        }
    }

    /**
     * Reads Uint32 value.
     *
     * @return Atom as Uint32.
     * @throws {ReadError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    readU32(): number {
        const atom = this.atom();

        if (atom.length == 4) {
            return new DataView(atom.buffer)
                .getUint32(atom.byteOffset, true);
        }

        throw new ReadError(ValueType.U32);
    }

    /**
     * Writes Uint32 value.
     *
     * @param value A value.
     * @throws {WriteError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    writeU32(value: number) {
        const atom = this.atom();

        if (atom.length == 4) {
            new DataView(atom.buffer)
                .setUint32(atom.byteOffset, value, true);
        } else {
            throw new WriteError(ValueType.U32);
        }
    }

    /**
     * Reads Int64 value.
     *
     * @return Atom as Int64.
     * @throws {ReadError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    readI64(): bigint {
        const atom = this.atom();

        if (atom.length == 8) {
            return new DataView(atom.buffer)
                .getBigInt64(atom.byteOffset, true);
        }

        throw new ReadError(ValueType.I64);
    }

    /**
     * Writes Int64 value.
     *
     * @param value A value.
     * @throws {WriteError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    writeI64(value: bigint) {
        const atom = this.atom();

        if (atom.length == 8) {
            new DataView(atom.buffer)
                .setBigInt64(atom.byteOffset, value, true);
        } else {
            throw new WriteError(ValueType.I64);
        }
    }

    /**
     * Reads Uint64 value.
     *
     * @return Atom as Uint64.
     * @throws {ReadError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    readU64(): bigint {
        const atom = this.atom();

        if (atom.length == 8) {
            return new DataView(atom.buffer)
                .getBigUint64(atom.byteOffset, true);
        }

        throw new ReadError(ValueType.U64);
    }

    /**
     * Writes Uint64 value.
     *
     * @param value A value.
     * @throws {WriteError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    writeU64(value: bigint) {
        const atom = this.atom();

        if (atom.length == 8) {
            new DataView(atom.buffer)
                .setBigUint64(atom.byteOffset, value, true);
        } else {
            throw new WriteError(ValueType.U64);
        }
    }

    /**
     * Reads Float32 value.
     *
     * @return Atom as Float32.
     * @throws {ReadError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    readF32(): number {
        const atom = this.atom();

        if (atom.length == 4) {
            return new DataView(atom.buffer)
                .getFloat32(atom.byteOffset, true);
        }

        throw new ReadError(ValueType.F32);
    }

    /**
     * Writes Float32 value.
     *
     * @param value A value.
     * @throws {WriteError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    writeF32(value: number) {
        const atom = this.atom();

        if (atom.length == 4) {
            new DataView(atom.buffer)
                .setFloat32(atom.byteOffset, value, true);
        } else {
            throw new WriteError(ValueType.F32);
        }
    }

    /**
     * Reads Float64 value.
     *
     * @return Atom as Float64.
     * @throws {ReadError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    readF64(): number {
        const atom = this.atom();

        if (atom.length == 8) {
            return new DataView(atom.buffer)
                .getFloat64(atom.byteOffset, true);
        }

        throw new ReadError(ValueType.F64);
    }

    /**
     * Writes Float64 value.
     *
     * @param value A value.
     * @throws {WriteError}
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    writeF64(value: number) {
        const atom = this.atom();

        if (atom.length == 8) {
            new DataView(atom.buffer)
                .setFloat64(atom.byteOffset, value, true);
        } else {
            throw new WriteError(ValueType.F64);
        }
    }

    /**
     * Reads String.
     *
     * @return Atom as string.
     * @throws {NotAtomError}
     * @throws {NotSexprError}
     */
    readString(): string {
        return new TextDecoder().decode(this.atom());
    }

    static #genNumberSexpr(length: number): ChobitSexpr {
        const body = new Uint8Array(length + SEXPR_HEADER_LEN);
        new DataView(body.buffer).setUint32(0, length, true);
        return new ChobitSexpr(body);
    }

    /**
     * Generates I8 atom.
     *
     * @param value A value.
     * @return Atom.
     */
    static genI8(value: number): ChobitSexpr {
        const ret = ChobitSexpr.#genNumberSexpr(1);
        ret.writeI8(value);
        return ret;
    }

    /**
     * Generates U8 atom.
     *
     * @param value A value.
     * @return Atom.
     */
    static genU8(value: number): ChobitSexpr {
        const ret = ChobitSexpr.#genNumberSexpr(1);
        ret.writeU8(value);
        return ret;
    }

    /**
     * Generates I16 atom.
     *
     * @param value A value.
     * @return Atom.
     */
    static genI16(value: number): ChobitSexpr {
        const ret = ChobitSexpr.#genNumberSexpr(2);
        ret.writeI16(value);
        return ret;
    }

    /**
     * Generates U16 atom.
     *
     * @param value A value.
     * @return Atom.
     */
    static genU16(value: number): ChobitSexpr {
        const ret = ChobitSexpr.#genNumberSexpr(2);
        ret.writeU16(value);
        return ret;
    }

    /**
     * Generates I32 atom.
     *
     * @param value A value.
     * @return Atom.
     */
    static genI32(value: number): ChobitSexpr {
        const ret = ChobitSexpr.#genNumberSexpr(4);
        ret.writeI32(value);
        return ret;
    }

    /**
     * Generates U32 atom.
     *
     * @param value A value.
     * @return Atom.
     */
    static genU32(value: number): ChobitSexpr {
        const ret = ChobitSexpr.#genNumberSexpr(4);
        ret.writeU32(value);
        return ret;
    }

    /**
     * Generates I64 atom.
     *
     * @param value A value.
     * @return Atom.
     */
    static genI64(value: bigint): ChobitSexpr {
        const ret = ChobitSexpr.#genNumberSexpr(8);
        ret.writeI64(value);
        return ret;
    }

    /**
     * Generates U64 atom.
     *
     * @param value A value.
     * @return Atom.
     */
    static genU64(value: bigint): ChobitSexpr {
        const ret = ChobitSexpr.#genNumberSexpr(8);
        ret.writeU64(value);
        return ret;
    }

    /**
     * Generates F32 atom.
     *
     * @param value A value.
     * @return Atom.
     */
    static genF32(value: number): ChobitSexpr {
        const ret = ChobitSexpr.#genNumberSexpr(4);
        ret.writeF32(value);
        return ret;
    }

    /**
     * Generates F64 atom.
     *
     * @param value A value.
     * @return Atom.
     */
    static genF64(value: number): ChobitSexpr {
        const ret = ChobitSexpr.#genNumberSexpr(8);
        ret.writeF64(value);
        return ret;
    }

    /**
     * Generates String atom.
     *
     * @param value A value.
     * @return Atom.
     */
    static genString(value: string): ChobitSexpr {
        const bytes = new TextEncoder().encode(value);

        const ret = ChobitSexpr.#genNumberSexpr(bytes.length);
        ret.#body.set(bytes, SEXPR_HEADER_LEN);
        return ret;
    }

    /**
     * Generates an iterator for this sexpr.
     *
     * @return iterator.
     */
    iter(): Iter {
        return new Iter(this);
    }
}

export class Iter implements IterableIterator<ChobitSexpr> {
    #sexpr: ChobitSexpr;

    constructor(sexpr: ChobitSexpr) {
        this.#sexpr = sexpr;
    }

    next(): IteratorResult<ChobitSexpr> {
        try {
            const carCdr = this.#sexpr.carCdr();

            const [car, cdr] = carCdr;

            this.#sexpr = cdr;

            return {
                done: false,
                value: car
            };
        } catch (e) {
            if (e instanceof NotConsError) {
                return {
                    done: true,
                    value: null
                };
            }

            throw e
        }
    }

    [Symbol.iterator](): IterableIterator<ChobitSexpr> {
        return this;
    }
}
