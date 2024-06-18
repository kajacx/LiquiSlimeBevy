import { allocateBytes,  freeBytes } from "./memory";
import { FactionId, FatPtr, InstanceId, PackedData, SlimeAmount, TimeInterval, UnitId } from "../api/types";
import * as exportsImpl from "./exports";

export function init(): void {
  exportsImpl.init();
}

export function describe_settings(): FatPtr {
  return exportsImpl.describeSettings();
}

export function default_settings(): FatPtr {
  return exportsImpl.defaultSettings();
}

export function new_instance(instance: InstanceId, settings: FatPtr): void {
  exportsImpl.newInstance(instance, settings);
}

export function change_settings(instance: InstanceId, settings: FatPtr): void {
  exportsImpl.changeSettings(instance, settings);
}

export function update(instance: InstanceId, timeElapsed: TimeInterval): void {
  exportsImpl.update(instance, timeElapsed);
}

@external("liquislime_api", "level_width")
declare function levelWidth(): i32;
@external("liquislime_api", "level_height")
declare function levelHeight(): i32;

@external("liquislime_api", "get_current_unit")
declare function getCurrentUnit(): UnitId;
@external("liquislime_api", "get_current_instance")
declare function getCurrentInstance(): InstanceId;

@external("liquislime_api", "get_own_faction")
declare function getOwnFaction(): FactionId;
@external("liquislime_api", "get_own_position")
declare function getOwnPosition(): PackedData;

@external("liquislime_api", "get_slime_amount")
declare function getSlimeAmount(faction: FactionId, position: PackedData): SlimeAmount;
@external("liquislime_api", "set_slime_amount")
declare function setSlimeAmount(faction: FactionId, position: PackedData, amount: SlimeAmount): void;

@external("liquislime_api", "is_mouse_pressed")
declare function isMousePressed(): bool;
@external("liquislime_api", "get_mouse_position")
declare function getMousePosition(): PackedData;

@external("liquislime_api", "log")
declare function log(message: FatPtr): void;

export {
  levelWidth,
  levelHeight,
  getCurrentUnit,
  getCurrentInstance,
  getOwnFaction,
  getOwnPosition,
  getSlimeAmount,
  setSlimeAmount,
  isMousePressed,
  getMousePosition,
  log,
};

export function allocate_bytes(len: u32): FatPtr {
  return allocateBytes(len);
}

export function free_bytes(bytes: FatPtr): void {
  freeBytes(bytes);
}
