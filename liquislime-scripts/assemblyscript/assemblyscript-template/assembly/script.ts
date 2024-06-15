import { ScriptTemplate } from "./template";
import { DynValue } from "./dyn_value";
import { SlimeAmountSettings } from "./settings";
import { SlimeAmount, TimeInterval } from "./types";

export const SETTINGS_DEFINITION = new SlimeAmountSettings(500);

export class Settings {
  amount: SlimeAmount;

  constructor(amount: SlimeAmount) {
    this.amount = amount;
  }

  static fromDynValue(value: DynValue): Settings {
    return new Settings(SETTINGS_DEFINITION.parse(value));
  }
}

export class UserScript implements ScriptTemplate<Settings> {
  settings: Settings;

  constructor(settings: Settings) {
    this.settings = settings;
  }

  changeSettings(settings: Settings): void {
    this.settings = settings;
  }

  update(timeElapsed: TimeInterval): void {
    // TODO: update
  }
}
