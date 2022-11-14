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
 * ChobitSexpr for Typescript.
 */
export class ChobitSexpr {
    private _body: Uint8Array;

    /**
     * Constructor.
     *
     * @param body Body of ChobitSexpr.
     */
    constructor(body: Uint8Array) {
        this._body = body;
    }

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
        const carLength = car._body.length;
        const cdrLength = cdr._body.length;

        const sexpr = new Uint8Array(carLength + cdrLength + SEXPR_HEADER_LEN);
        const header = ChobitSexpr.genConsHeader(carLength);

        const view = new DataView(sexpr.buffer);
        view.setUint32(sexpr.byteOffset, header, true);

        sexpr.set(car._body, SEXPR_HEADER_LEN);
        sexpr.set(cdr._body, carLength + SEXPR_HEADER_LEN);

        return new ChobitSexpr(sexpr);
    }

    private _header(): number | null {
        if (this._body.length < SEXPR_HEADER_LEN) {return null;}

        return new DataView(this._body.buffer).getUint32(
            this._body.byteOffset,
            true
        );
    }

    private static _flag(header: number): number {
        return header & 0x80000000;
    }

    private static _size(header: number): number {
        return header & 0x7fffffff;
    }

    /**
     * Whether this sexpr is atom or not.
     *
     * @return If this sexpr is atom, true.
     */
    isAtom(): boolean {
        const header = this._header();
        if (header) {
            return ChobitSexpr._flag(header) == 0;
        }

        return false;
    }

    /**
     * Whether this sexpr is cons or not.
     *
     * @return If this sexpr is cons, true.
     */
    isCons(): boolean {
        const header = this._header();
        if (header) {
            return ChobitSexpr._flag(header) != 0;
        }

        return false;
    }

    /**
     * Gets atom.
     *
     * @return If this sexpr is atom, returns payload. Otherwise, null.
     */
    atom(): Uint8Array | null {
        const header = this._header();

        if (header) {
            if (ChobitSexpr._flag(header) != 0) {return null;}

            const size = ChobitSexpr._size(header);
            if ((SEXPR_HEADER_LEN + size) > this._body.length) {return null;}

            return new Uint8Array(
                this._body.buffer,
                this._body.byteOffset + SEXPR_HEADER_LEN,
                size
            );
        } else {
            return null;
        }
    }

    /**
     * Gets car.
     *
     * @return If this sexpr is cons, returns car. Otherwise, null.
     */
    car(): ChobitSexpr | null {
        const header = this._header();

        if (header) {
            if (ChobitSexpr._flag(header) == 0) {return null;}

            const size = ChobitSexpr._size(header);
            if ((SEXPR_HEADER_LEN + size) > this._body.length) {return null;}

            return new ChobitSexpr(new Uint8Array(
                this._body.buffer,
                this._body.byteOffset + SEXPR_HEADER_LEN,
                size
            ));
        } else {
            return null;
        }
    }

    /**
     * Gets car.
     *
     * @return If this sexpr is cons, returns cdr. Otherwise, returns null.
     */
    cdr(): ChobitSexpr | null {
        const header = this._header();

        if (header) {
            if (ChobitSexpr._flag(header) == 0) {return null;}

            const size = ChobitSexpr._size(header);
            if ((SEXPR_HEADER_LEN + size) > this._body.length) {return null;}

            return new ChobitSexpr(new Uint8Array(
                this._body.buffer,
                this._body.byteOffset + SEXPR_HEADER_LEN + size,
            ));
        } else {
            return null;
        }
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
     * @return If this sexpr is atom and payload is 1 byte, returns value. Otherwise, returns null.
     */
    readI8(): number | null {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 1) {
                return new DataView(atom.buffer).getInt8(atom.byteOffset);
            }
        }

        return null;
    }

    /**
     * Writes Int8 value.
     *
     * @param value A value.
     */
    writeI8(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 1) {
                return new DataView(atom.buffer)
                    .setInt8(atom.byteOffset, value);
            }
        }
    }

    /**
     * Reads Uint8 value.
     *
     * @return If this sexpr is atom and payload is 1 byte, returns value. Otherwise, returns null.
     */
    readU8(): number | null {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 1) {
                return new DataView(atom.buffer).getUint8(atom.byteOffset);
            }
        }

        return null;
    }

    /**
     * Writes Uint8 value.
     *
     * @param value A value.
     */
    writeU8(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 1) {
                return new DataView(atom.buffer)
                    .setUint8(atom.byteOffset, value);
            }
        }
    }

    /**
     * Reads Int16 value.
     *
     * @return If this sexpr is atom and payload is 2 byte, returns value. Otherwise, returns null.
     */
    readI16(): number | null {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 2) {
                return new DataView(atom.buffer)
                    .getInt16(atom.byteOffset, true);
            }
        }

        return null;
    }

    /**
     * Writes Int16 value.
     *
     * @param value A value.
     */
    writeI16(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 2) {
                return new DataView(atom.buffer)
                    .setInt16(atom.byteOffset, value, true);
            }
        }
    }

    /**
     * Reads Uint16 value.
     *
     * @return If this sexpr is atom and payload is 2 byte, returns value. Otherwise, returns null.
     */
    readU16(): number | null {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 2) {
                return new DataView(atom.buffer)
                    .getUint16(atom.byteOffset, true);
            }
        }

        return null;
    }

    /**
     * Writes Uint16 value.
     *
     * @param value A value.
     */
    writeU16(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 2) {
                return new DataView(atom.buffer)
                    .setUint16(atom.byteOffset, value, true);
            }
        }
    }

    /**
     * Reads Int32 value.
     *
     * @return If this sexpr is atom and payload is 4 byte, returns value. Otherwise, returns null.
     */
    readI32(): number | null {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 4) {
                return new DataView(atom.buffer)
                    .getInt32(atom.byteOffset, true);
            }
        }

        return null;
    }

    /**
     * Writes Int32 value.
     *
     * @param value A value.
     */
    writeI32(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 4) {
                return new DataView(atom.buffer)
                    .setInt32(atom.byteOffset, value, true);
            }
        }
    }

    /**
     * Reads Uint32 value.
     *
     * @return If this sexpr is atom and payload is 4 byte, returns value. Otherwise, returns null.
     */
    readU32(): number | null {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 4) {
                return new DataView(atom.buffer)
                    .getUint32(atom.byteOffset, true);
            }
        }

        return null;
    }

    /**
     * Writes Uint32 value.
     *
     * @param value A value.
     */
    writeU32(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 4) {
                return new DataView(atom.buffer)
                    .setUint32(atom.byteOffset, value, true);
            }
        }
    }

    /**
     * Reads Int64 value.
     *
     * @return If this sexpr is atom and payload is 8 byte, returns value. Otherwise, returns null.
     */
    readI64(): bigint | null {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 8) {
                return new DataView(atom.buffer)
                    .getBigInt64(atom.byteOffset, true);
            }
        }

        return null;
    }

    /**
     * Writes Int64 value.
     *
     * @param value A value.
     */
    writeI64(value: bigint) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 8) {
                return new DataView(atom.buffer)
                    .setBigInt64(atom.byteOffset, value, true);
            }
        }
    }

    /**
     * Reads Uint64 value.
     *
     * @return If this sexpr is atom and payload is 8 byte, returns value. Otherwise, returns null.
     */
    readU64(): bigint | null {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 8) {
                return new DataView(atom.buffer)
                    .getBigUint64(atom.byteOffset, true);
            }
        }

        return null;
    }

    /**
     * Writes Uint64 value.
     *
     * @param value A value.
     */
    writeU64(value: bigint) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 8) {
                return new DataView(atom.buffer)
                    .setBigUint64(atom.byteOffset, value, true);
            }
        }
    }

    /**
     * Reads Float32 value.
     *
     * @return If this sexpr is atom and payload is 4 byte, returns value. Otherwise, returns null.
     */
    readF32(): number | null {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 4) {
                return new DataView(atom.buffer)
                    .getFloat32(atom.byteOffset, true);
            }
        }

        return null;
    }

    /**
     * Writes Float32 value.
     *
     * @param value A value.
     */
    writeF32(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 4) {
                return new DataView(atom.buffer)
                    .setFloat32(atom.byteOffset, value, true);
            }
        }
    }

    /**
     * Reads Float64 value.
     *
     * @return If this sexpr is atom and payload is 8 byte, returns value. Otherwise, returns null.
     */
    readF64(): number | null {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 8) {
                return new DataView(atom.buffer)
                    .getFloat64(atom.byteOffset, true);
            }
        }

        return null;
    }

    /**
     * Writes Float64 value.
     *
     * @param value A value.
     */
    writeF64(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 8) {
                return new DataView(atom.buffer)
                    .setFloat64(atom.byteOffset, value, true);
            }
        }
    }

    /**
     * Reads String.
     *
     * @return If this sexpr is atom, returns value. Otherwise, returns null.
     */
    readString(): string | null {
        const atom = this.atom();
        if (atom) {
            return new TextDecoder().decode(atom);
        }

        return null;
    }

    private static _genNumberSexpr(length: number): ChobitSexpr {
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
        const ret = ChobitSexpr._genNumberSexpr(1);
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
        const ret = ChobitSexpr._genNumberSexpr(1);
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
        const ret = ChobitSexpr._genNumberSexpr(2);
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
        const ret = ChobitSexpr._genNumberSexpr(2);
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
        const ret = ChobitSexpr._genNumberSexpr(4);
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
        const ret = ChobitSexpr._genNumberSexpr(4);
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
        const ret = ChobitSexpr._genNumberSexpr(8);
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
        const ret = ChobitSexpr._genNumberSexpr(8);
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
        const ret = ChobitSexpr._genNumberSexpr(4);
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
        const ret = ChobitSexpr._genNumberSexpr(8);
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

        const ret = ChobitSexpr._genNumberSexpr(bytes.length);
        ret._body.set(bytes, SEXPR_HEADER_LEN);
        return ret;
    }
}
