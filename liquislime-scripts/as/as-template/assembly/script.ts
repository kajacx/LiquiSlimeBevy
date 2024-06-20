import { ScriptTemplate } from "./template";
import { DynValue } from "./api/dyn_value";
import { SlimeAmount, TimeInterval } from "./api/types";
import {
  SettingsDescription,
  ObjectSettings,
  StringSettings,
  Float64Settings,
} from "./api/settings";
import {
  addSlimeAmount,
  getOwnFaction,
  isMousePressed,
  getMousePosition,
  log,
} from "./api/imports";
import { levelHeight, levelWidth } from "./internal/bindings";

export const SETTINGS_DEFINITION = new ObjectSettings(
  new Map<string, SettingsDescription>()
    .set("name", new StringSettings("AS template"))
    .set("amount", new Float64Settings(500))
);

class Settings {
  name: string;
  amount: f64;

  constructor(name: string, amount: SlimeAmount) {
    this.name = name;
    this.amount = amount;
  }

  static fromDynValue(value: DynValue): Settings {
    return new Settings(
      value.getObjectValue("name")!.getString()!,
      value.getObjectValue("amount")!.getFloat()
    );
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
    log(`Running "${this.settings.name}" script`);
    const position = getMousePosition();
    if (isMousePressed() && position != null) {
      const faction = getOwnFaction();
      const tile = position.toTilePosition();
      const amount = this.settings.amount * timeElapsed;

      addSlimeAmount(faction, tile.withX(0), amount);
      addSlimeAmount(faction, tile.withY(0), amount);
      addSlimeAmount(faction, tile.withX(levelWidth() - 1), amount);
      addSlimeAmount(faction, tile.withY(levelHeight() - 1), amount);
    }
  }
}
