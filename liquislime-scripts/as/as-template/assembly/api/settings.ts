import { DynValue } from "./dyn_value";

export interface SettingsDescription {
  describeSettings(): DynValue;
}

export class NoneSettings implements SettingsDescription {
  describeSettings(): DynValue {
    return DynValue.object(
      new Map<string, DynValue>().set("type", DynValue.string("None"))
    );
  }
}

export class Float64Settings implements SettingsDescription {
  defaultValue: f64;

  constructor(defaultValue: f64) {
    this.defaultValue = defaultValue;
  }

  describeSettings(): DynValue {
    return DynValue.object(
      new Map<string, DynValue>()
        .set("type", DynValue.string("Float64"))
        .set("default_value", DynValue.float(this.defaultValue))
    );
  }
}

export class StringSettings implements SettingsDescription {
  defaultValue: string;

  constructor(defaultValue: string) {
    this.defaultValue = defaultValue;
  }

  describeSettings(): DynValue {
    return DynValue.object(
      new Map<string, DynValue>()
        .set("type", DynValue.string("String"))
        .set("default_value", DynValue.string(this.defaultValue))
    );
  }
}

export class ObjectSettings implements SettingsDescription {
  values: Map<string, SettingsDescription>;

  constructor(values: Map<string, SettingsDescription>) {
    this.values = values;
  }

  describeSettings(): DynValue {
    const values = new Map<string, DynValue>();
    const keys = this.values.keys();
    for (let i = 0; i < keys.length; i++) {
      const key = keys[i];
      values.set(key, this.values.get(key).describeSettings());
    }

    return DynValue.object(
      new Map<string, DynValue>()
        .set("type", DynValue.string("Object"))
        .set("values", DynValue.object(values))
    );
  }
}
