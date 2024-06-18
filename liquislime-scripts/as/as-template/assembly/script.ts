import { ScriptTemplate } from "./template";
import { DynValue } from "./api/dyn_value";
import { SlimeAmount, TimeInterval } from "./api/types";
import { FloatSettings } from "./api/settings";
import {
  addSlimeAmount,
  getOwnFaction,
  isMousePressed,
  getMousePosition,
} from "./api/imports";

export const SETTINGS_DEFINITION = new FloatSettings(500);

class Settings {
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
    // throw new Error("Setitngs ARE: " + settings.toString());
  }

  changeSettings(settings: DynValue): void {
    this.settings = Settings.fromDynValue(settings);
  }

  update(timeElapsed: TimeInterval): void {
    const position = getMousePosition();
    if (isMousePressed() && position != null) {
      const faction = getOwnFaction();
      addSlimeAmount(
        faction,
        position.toTilePosition(),
        this.settings.amount * timeElapsed
      );
    }
  }
}
