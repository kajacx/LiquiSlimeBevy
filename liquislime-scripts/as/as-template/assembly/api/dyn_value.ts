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

  static string(value: string): DynValue {
    return new StringValue(value);
  }

  static object(values: Map<string, DynValue>): DynValue {
    return new ObjectValue(values);
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

  isString(): bool {
    return false;
  }

  getString(): string | null {
    return null;
  }

  isObject(): bool {
    return false;
  }

  getObjectValues(): Map<string, DynValue> | null {
    return null;
  }

  getObjectValue(_key: string): DynValue | null {
    return null;
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

  getFloat(): number {
    return this.value;
  }

  encode(writer: Writer): void {
    writer.writeFloat64(this.value);
  }

  toString(): string {
    return "Float: " + this.value.toString();
  }
}

class StringValue extends DynValue {
  value: string;

  constructor(value: string) {
    super();
    this.value = value;
  }

  isString(): bool {
    return true;
  }

  getString(): string | null {
    return this.value;
  }

  encode(writer: Writer): void {
    writer.writeString(this.value);
  }

  toString(): string {
    return 'String: "' + this.value.toString() + '"';
  }
}

class ObjectValue extends DynValue {
  // TODO: preserve order
  values: Map<string, DynValue>;

  constructor(values: Map<string, DynValue>) {
    super();
    this.values = values;
  }

  isObject(): bool {
    return true;
  }

  getObjectValues(): Map<string, DynValue> | null {
    return this.values;
  }

  getObjectValue(key: string): DynValue | null {
    if (this.values.has(key)) {
      return this.values.get(key);
    } else {
      return null;
    }
  }

  encode(writer: Writer): void {
    writer.writeMapSize(this.values.size);
    const keys = this.values.keys();
    for (let i = 0; i < keys.length; i++) {
      const key = keys[i];
      writer.writeString(key);
      this.values.get(key).encode(writer);
    }
  }

  toString(): string {
    let text = "Object: {";
    let first = true;
    const keys = this.values.keys();
    for (let i = 0; i < keys.length; i++) {
      const key = keys[i];
      if (first) {
        first = false;
      } else {
        text += ",";
      }
      text += ` "${key}": `;
      text += this.values.get(key).toString();
    }
    return text + " }";
  }
}

export function dynValueFromPtr(ptr: FatPtr): DynValue {
  const bytes = readArrayBuffer(ptr);
  const reader = new DataReader(bytes);
  const entryReader = new EntryReader(reader);
  const value = decodeDynValue(entryReader);
  return value;
}

export function decodeDynValue(entryReader: EntryReader): DynValue {
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
  if (entry.isString()) {
    return DynValue.string(entry.readString());
  }
  if (entry.isMapLength()) {
    const values = new Map<string, DynValue>();
    const len = entry.readMapLength();
    for (let i = 0 as usize; i < len; i++) {
      const name = entryReader.nextEntry()!.readString();
      const value = decodeDynValue(entryReader);
      values.set(name, value);
    }
    return DynValue.object(values);
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
