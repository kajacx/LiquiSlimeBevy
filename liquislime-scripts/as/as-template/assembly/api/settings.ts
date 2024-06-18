import { DynValue } from "./dyn_value";
import { log } from "./imports";
import { SlimeAmount } from "./types";
import { Writer } from "@wapc/as-msgpack";

interface Settings {
  describeSettings(packer: Writer): void;
  defaultValue(): DynValue;
}

export class NoneSettings implements Settings {
  describeSettings(writer: Writer): void {
    // writer.string("None"); // TODO: check
    writer.writeString("None");
  }

  defaultValue(): DynValue {
    return DynValue.none();
  }

  parse(_value: DynValue): null {
    return null;
  }
}

export class FloatSettings implements Settings {
  default_value: f64;

  constructor(default_value: f64) {
    this.default_value = default_value;
  }

  describeSettings(writer: Writer): void {
    writer.writeString("f64");
  }

  defaultValue(): DynValue {
    return DynValue.float(this.default_value);
  }

  parse(value: DynValue): SlimeAmount {
    if (value.isFloat()) {
      return value.getFloat();
    } else {
      log(`Warning: value '${value.toString()}' is not a float`);
      return 0;
    }
  }
}
