import { Option } from "./option";
import { SlimeAmount, slimeAmountToAbi } from "./types";
// import { Packer } from "msgpack-as";
import { Writer } from "@wapc/as-msgpack";

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
