import { readArrayBuffer } from "../internal/memory";
import { FatPtr } from "./types";
import { DataReader, Writer, EntryReader } from "@wapc/as-msgpack";

export abstract class DynValue {
  static none(): DynValue {
    return new NoneValue();
  }

  static integer(value: i64): DynValue {
    return new IntegerValue(value);
  }

  static float(value: f64): DynValue {
    return new FloatValue(value);
  }

  isNone(): bool {
    return false;
  }

  isInteger(): bool {
    return false;
  }

  getInteger(): i64 {
    throw new Error(`DynValue '${this.toString()}' is not an integer`);
  }

  isFloat(): bool {
    return false;
  }

  getFloat(): f64 {
    throw new Error(`DynValue '${this.toString()}' is not a float`);
  }

  abstract encode(writer: Writer): void;

  abstract toString(): string;
}

class NoneValue extends DynValue {
  encode(writer: Writer): void {
    writer.writeNil();
  }

  toString(): string {
    return "None";
  }
}

class IntegerValue extends DynValue {
  value: i64;

  constructor(value: i64) {
    super();
    this.value = value;
  }

  isInteger(): bool {
    return true;
  }

  asInteger(): i64 {
    return this.value;
  }

  encode(writer: Writer): void {
    writer.writeInt64(this.value);
  }

  toString(): string {
    return "Integer: " + this.value.toString();
  }
}

class FloatValue extends DynValue {
  value: f64;

  constructor(value: f64) {
    super();
    this.value = value;
  }

  isFloat(): bool {
    return true;
  }

  asFloat(): number {
    return this.value;
  }

  encode(writer: Writer): void {
    writer.writeFloat64(this.value);
  }

  toString(): string {
    return "Float: " + this.value.toString();
  }
}

export function dynValueFromPtr(ptr: FatPtr): DynValue {
  const bytes = readArrayBuffer(ptr);
  const reader = new DataReader(bytes);
  const value = decodeDynValue(reader);
  return value;
}

export function decodeDynValue(data: DataReader): DynValue {
  const entryReader = new EntryReader(data);
  const entry = entryReader.nextEntry()!;

  if (entry.isInt(false)) {
    return DynValue.integer(entry.readInt());
  }
  if (entry.isUint(false)) {
    return DynValue.integer(entry.readUint() as i64);
  }
  if (entry.isFloat()) {
    return DynValue.float(entry.readFloat());
  }

  throw new Error("Unknown entry: " + entry.toString());
}

function debugPrintBytes(bytes: ArrayBuffer, radix: i32 = 10): string {
  let result = "ArrayBuffer[";
  let first = true;
  for (let i = 0; i < bytes.byteLength; i++) {
    if (first) {
      first = false;
    } else {
      result += ", ";
    }
    let byte = load<u8>(changetype<usize>(bytes) + i);
    result += byte.toString(radix);
  }
  return result + "]";
}
