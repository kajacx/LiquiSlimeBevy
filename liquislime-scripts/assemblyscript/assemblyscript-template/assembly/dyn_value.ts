import { Option } from "./option";
import { SlimeAmount } from "./types";

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
}

class NoneValue extends DynValue {}

class NumberValue extends DynValue {
  value: number;

  constructor(value: number) {
    super();
    this.value = value;
  }

  asNumber(): Option<number> {
    return Option.Some(this.value);
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
}
