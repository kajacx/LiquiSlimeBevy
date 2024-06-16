import { ScriptTemplate } from "./template";
import { DynValue } from "./dyn_value";
import { SlimeAmountSettings } from "./settings";
import {
  SlimeAmount,
  TimeInterval,
  slimeAmountFromAbi,
  slimeAmountToAbi,
} from "./types";
import { liquislime_api } from "./bindings";

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

export class UserScript implements ScriptTemplate {
  settings: Settings;

  constructor(settings: DynValue) {
    this.settings = Settings.fromDynValue(settings);
  }

  changeSettings(settings: DynValue): void {
    this.settings = Settings.fromDynValue(settings);
  }

  update(timeElapsed: TimeInterval): void {
    if (liquislime_api.is_mouse_pressed()) {
      const faction = liquislime_api.get_own_faction();
      const positionAbi = liquislime_api.get_own_position();
      const amount = slimeAmountFromAbi(
        liquislime_api.get_slime_amount(faction, positionAbi)
      );
      liquislime_api.set_slime_amount(
        faction,
        positionAbi,
        slimeAmountToAbi(amount + this.settings.amount * timeElapsed)
      );
    }
  }
}
