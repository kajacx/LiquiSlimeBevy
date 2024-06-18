import { DynValue } from "./dyn_value";
import { SlimeAmount } from "./types";
import { Writer } from "@wapc/as-msgpack";

interface Settings<T> {
  describeSettings(packer: Writer): void;
  defaultValue(): DynValue;
  parse(value: DynValue): T;
}

export class NoneSettings implements Settings<null> {
  describeSettings(writer: Writer): void {
    // writer.string("None"); // TODO: check
    writer.writeString("None");
  }

  defaultValue(): DynValue {
    return DynValue.none();
  }

  parse(value: DynValue): null {
    return null;
  }
}

export class SlimeAmountSettings implements Settings<SlimeAmount> {
  default_amount: SlimeAmount;

  constructor(default_amount: SlimeAmount) {
    this.default_amount = default_amount;
  }

  describeSettings(writer: Writer): void {
    writer.writeString("SlimeAmount");
  }

  defaultValue(): DynValue {
    return DynValue.slimeAmount(this.default_amount);
  }

  parse(value: DynValue): SlimeAmount {
    // TODO: log error
    return value.asSlimeAmount().getOr(0);
  }
}
