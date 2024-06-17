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
}

class NoneValue extends DynValue {
  encode(writer: Writer): void {
    writer.writeNil();
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
}

export function dynValueFromPtr(ptr: FatPtr): DynValue {
  const bytes = readArrayBuffer(ptr);
  const reader = new DataReader(bytes);
  return decodeDynValue(reader);
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
    throw new Error("what is the tag?" + tag);
    if (tag == "SlimeAmount") {
      return DynValue.slimeAmount(
        slimeAmountFromAbi(
          entryReader.nextEntry()!.tryReadInt().get("Get slime amount")
        )
      );
    }
  }

  // TODO: print entry
  throw new Error("Unknown entry: " + entry.toString());
}
