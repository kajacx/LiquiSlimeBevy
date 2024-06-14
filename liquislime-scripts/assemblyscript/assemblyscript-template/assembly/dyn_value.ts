import { Option } from "./option";

export abstract class DynValue {
  static none(): DynValue {
    return new NoneValue();
  }

  static number(value: number): DynValue {
    return new NumberValue(value);
  }

  asNumber(): Option<number> {
    return Option.None<number>();
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
