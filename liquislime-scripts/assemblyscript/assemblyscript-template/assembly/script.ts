import { ScriptTemplate } from "./template";
import { DynValue } from "./dyn_value";
import { SlimeAmountSettings } from "./settings";
import {
  SlimeAmount,
  TimeInterval,
  slimeAmountFromAbi,
  slimeAmountToAbi,
} from "./types";
import {
  getOwnFaction,
  getSlimeAmount,
  isMousePressed,
  liquislime_api,
  setSlimeAmount,
} from "./bindings";
import { packU32s } from "./helpers";

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
    // throw new Error("Setitngs ARE: " + settings.toString());
  }

  changeSettings(settings: DynValue): void {
    this.settings = Settings.fromDynValue(settings);
  }

  update(timeElapsed: TimeInterval): void {
    // if (liquislime_api.is_mouse_pressed()) {
    if (isMousePressed()) {
      // const faction = liquislime_api.get_own_faction();
      const faction = getOwnFaction();
      const position = packU32s(5, 7);

      // throw new Error(
      //   "settings: " +
      //     this.settings.amount.toString() +
      //     " elapsed: " +
      //     timeElapsed.toString()
      // );

      const amount = slimeAmountFromAbi(getSlimeAmount(faction, position));
      setSlimeAmount(
        faction,
        position,
        slimeAmountToAbi(amount + this.settings.amount * timeElapsed)
      );

      // const amount = slimeAmountFromAbi(
      //   liquislime_api.get_slime_amount(faction, positionAbi)
      // );
      // liquislime_api.set_slime_amount(
      //   faction,
      //   positionAbi,
      //   slimeAmountToAbi(amount + this.settings.amount * timeElapsed)
      // );
    }
  }
}
