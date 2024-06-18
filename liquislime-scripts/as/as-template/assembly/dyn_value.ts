import { liquislime_api } from "./bindings";
import { readArrayBuffer } from "./memory";
import { Option } from "./option";
import {
  FatPtr,
  SlimeAmount,
  slimeAmountFromAbi,
  slimeAmountToAbi,
} from "./types";
import { DataReader, Encoder, Writer, EntryReader } from "@wapc/as-msgpack";

export abstract class DynValue {
  static none(): DynValue {
    return new NoneValue();
  }

  static number(value: number): DynValue {
    return new NumberValue(value);
  }

  static slimeAmount(amount: SlimeAmount): DynValue {
    return new SlimeAmountValue(amount);
  }

  asNumber(): Option<number> {
    return Option.None<number>();
  }

  asSlimeAmount(): Option<SlimeAmount> {
    return Option.None<SlimeAmount>();
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

class NumberValue extends DynValue {
  value: number;

  constructor(value: number) {
    super();
    this.value = value;
  }

  asNumber(): Option<number> {
    return Option.Some(this.value);
  }

  encode(writer: Writer): void {
    writer.writeFloat64(this.value);
  }

  toString(): string {
    return "Number: " + this.value.toString();
  }
}

class SlimeAmountValue extends DynValue {
  amount: SlimeAmount;

  constructor(amount: SlimeAmount) {
    super();
    this.amount = amount;
  }

  asSlimeAmount(): Option<SlimeAmount> {
    return Option.Some(this.amount);
  }

  encode(writer: Writer): void {
    writer.writeMapSize(1);
    writer.writeString("SlimeAmount");
    writer.writeInt64(slimeAmountToAbi(this.amount));
  }

  toString(): string {
    return (
      "SlimeAmount: " +
      this.amount.toString() +
      ", raw: " +
      slimeAmountToAbi(this.amount).toString()
    );
  }
}

export function dynValueFromPtr(ptr: FatPtr): DynValue {
  const bytes = readArrayBuffer(ptr);
  // throw new Error("WTF? " + debugPrintBytes(bytes, 2));
  const reader = new DataReader(bytes);
  const value = decodeDynValue(reader);

  // throw new Error(
  //   "WHUT?" + debugPrintBytes(bytes, 2) + " IS " + value.toString()
  // );
  return value;
}

export function decodeDynValue(data: DataReader): DynValue {
  const entryReader = new EntryReader(data);
  const entry = entryReader.nextEntry()!;

  if (entry.isInt(false)) {
    return DynValue.number(entry.readInt() as f64);
  }
  if (entry.isUint(false)) {
    return DynValue.number(entry.readUint() as f64);
  }
  if (entry.isFloat()) {
    return DynValue.number(entry.readFloat());
  }
  if (entry.isMapLength() && entry.readMapLength() == 1) {
    const tag = entryReader.nextEntry()!.tryReadString().get("Get map tag");
    // throw new Error("what is the tag?" + tag);
    if (tag == "SlimeAmount") {
      const entry = entryReader.nextEntry()!;
      const abi = entry.tryReadInt().get("Get slime amount");
      const result = DynValue.slimeAmount(slimeAmountFromAbi(abi));
      // throw new Error(
      //   "ENTRY: " +
      //     entry.toString() +
      //     ", GOT DYN VALUE: " +
      //     result.toString() +
      //     ", RAW SLIME ABI: " +
      //     abi.toString()
      // );
      return result;
    }

    throw new Error("Unknown map tag: " + tag);
  }

  // TODO: print entry
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
