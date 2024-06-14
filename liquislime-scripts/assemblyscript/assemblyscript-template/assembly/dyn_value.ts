export abstract class DynValue {
  static none():  DynValue  {
    return new NoneValue();
  };

  static number: (value: number) => DynValue = (value) => {
    return new NumberValue(value);
  };

  abstract asNumber: () => number = () => {
    return 0;
  };
}

class NoneValue extends DynValue {}

class NumberValue extends DynValue {
  value: number;

  constructor(value: number) {
    super();
    this.value = value;
  }

  asNumber: () => number = () => {
    return this.value;
  };
}
