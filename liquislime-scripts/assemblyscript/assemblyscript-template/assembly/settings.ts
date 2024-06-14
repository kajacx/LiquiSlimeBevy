import { DynValue } from "./dyn_value";
import { Writer } from "as-proto";
import { SlimeAmount } from "./types";

export const settings = {
  none: () => NoneSettings,
  slimeAmount: (default_amount: SlimeAmount) =>
    new SlimeAmountSettings(default_amount),
};

interface Settings {
  describeSettings(writer: Writer): void;
  defaultValue(): DynValue;
}

class NoneSettings implements Settings {
  describeSettings(writer: Writer): void {
    writer.string("None");
  }

  defaultValue(): DynValue {
    return DynValue.none();
  }
}

class SlimeAmountSettings implements Settings {
  default_amount: SlimeAmount;

  constructor(default_amount: SlimeAmount) {
    this.default_amount = default_amount;
  }

  describeSettings(writer: Writer): void {
    writer.string("SlimeAmount");
  }

  defaultValue(): DynValue {
    return DynValue.slimeAmount(this.default_amount);
  }
}
