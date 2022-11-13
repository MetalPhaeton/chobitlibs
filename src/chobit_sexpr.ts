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

export class ChobitSexpr {
    private _body: Uint8Array;

    constructor(body: Uint8Array) {
        this._body = body;
    }

    static genConsHeader(carSize: number): number {
        return (carSize & 0x7fffffff) | 0x80000000;
    }

    static genAtomHeader(size: number): number {
        return size & 0x7fffffff;
    }

    static genAtom(data: Uint8Array): ChobitSexpr {
        const sexpr = new Uint8Array(data.length + SEXPR_HEADER_LEN);
        const header = ChobitSexpr.genAtomHeader(data.length);

        const view = new DataView(sexpr.buffer);
        view.setUint32(sexpr.byteOffset, header, true);

        sexpr.set(data, SEXPR_HEADER_LEN);

        return new ChobitSexpr(sexpr);
    }

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

    static genNil(): ChobitSexpr {
        return ChobitSexpr.genAtom(new Uint8Array(0));
    }

    readI8(): number | null {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 1) {
                return new DataView(atom.buffer).getInt8(atom.byteOffset);
            }
        }

        return null;
    }

    writeI8(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 1) {
                return new DataView(atom.buffer)
                    .setInt8(atom.byteOffset, value);
            }
        }
    }

    readU8(): number | null {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 1) {
                return new DataView(atom.buffer).getUint8(atom.byteOffset);
            }
        }

        return null;
    }

    writeU8(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 1) {
                return new DataView(atom.buffer)
                    .setUint8(atom.byteOffset, value);
            }
        }
    }

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

    writeI16(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 2) {
                return new DataView(atom.buffer)
                    .setInt16(atom.byteOffset, value, true);
            }
        }
    }

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

    writeU16(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 2) {
                return new DataView(atom.buffer)
                    .setUint16(atom.byteOffset, value, true);
            }
        }
    }

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

    writeI32(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 4) {
                return new DataView(atom.buffer)
                    .setInt32(atom.byteOffset, value, true);
            }
        }
    }

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

    writeU32(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 4) {
                return new DataView(atom.buffer)
                    .setUint32(atom.byteOffset, value, true);
            }
        }
    }

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

    writeI64(value: bigint) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 8) {
                return new DataView(atom.buffer)
                    .setBigInt64(atom.byteOffset, value, true);
            }
        }
    }

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

    writeU64(value: bigint) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 8) {
                return new DataView(atom.buffer)
                    .setBigUint64(atom.byteOffset, value, true);
            }
        }
    }

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

    writeF32(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 4) {
                return new DataView(atom.buffer)
                    .setFloat32(atom.byteOffset, value, true);
            }
        }
    }

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

    writeF64(value: number) {
        const atom = this.atom();
        if (atom) {
            if (atom.length == 8) {
                return new DataView(atom.buffer)
                    .setFloat64(atom.byteOffset, value, true);
            }
        }
    }

    private static _genNumberSexpr(length: number): ChobitSexpr {
        const body = new Uint8Array(length + SEXPR_HEADER_LEN);
        new DataView(body.buffer).setUint32(0, length, true);
        return new ChobitSexpr(body);
    }

    static genI8(value: number): ChobitSexpr {
        const ret = ChobitSexpr._genNumberSexpr(1);
        ret.writeI8(value);
        return ret;
    }

    static genU8(value: number): ChobitSexpr {
        const ret = ChobitSexpr._genNumberSexpr(1);
        ret.writeU8(value);
        return ret;
    }

    static genI16(value: number): ChobitSexpr {
        const ret = ChobitSexpr._genNumberSexpr(2);
        ret.writeI16(value);
        return ret;
    }

    static genU16(value: number): ChobitSexpr {
        const ret = ChobitSexpr._genNumberSexpr(2);
        ret.writeU16(value);
        return ret;
    }

    static genI32(value: number): ChobitSexpr {
        const ret = ChobitSexpr._genNumberSexpr(4);
        ret.writeI32(value);
        return ret;
    }

    static genU32(value: number): ChobitSexpr {
        const ret = ChobitSexpr._genNumberSexpr(4);
        ret.writeU32(value);
        return ret;
    }

    static genI64(value: bigint): ChobitSexpr {
        const ret = ChobitSexpr._genNumberSexpr(8);
        ret.writeI64(value);
        return ret;
    }

    static genU64(value: bigint): ChobitSexpr {
        const ret = ChobitSexpr._genNumberSexpr(8);
        ret.writeU64(value);
        return ret;
    }

    static genF32(value: number): ChobitSexpr {
        const ret = ChobitSexpr._genNumberSexpr(4);
        ret.writeF32(value);
        return ret;
    }

    static genF64(value: number): ChobitSexpr {
        const ret = ChobitSexpr._genNumberSexpr(8);
        ret.writeF64(value);
        return ret;
    }
}
