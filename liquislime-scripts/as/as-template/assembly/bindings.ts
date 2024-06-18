import { allocateBytes, encodeToMemory, freeBytes, writeBytes } from "./memory";
import { SETTINGS_DEFINITION, UserScript } from "./script";
import {
  FactionId,
  FatPtr,
  InstanceId,
  PackedData,
  UnitId,
  timeIntervalFromAbi,
} from "./types";
import { DynValue, decodeDynValue, dynValueFromPtr } from "./dyn_value";
import { Encoder } from "@wapc/as-msgpack";

export function init(): void {}

export function describe_settings(): FatPtr {
  return encodeToMemory((writer) =>
    SETTINGS_DEFINITION.describeSettings(writer)
  );
}

export function default_settings(): FatPtr {
  return encodeToMemory((writer) =>
    SETTINGS_DEFINITION.defaultValue().encode(writer)
  );
}

const INSTANCES = new Map<InstanceId, UserScript>();

export function new_instance(instance: InstanceId, settings: FatPtr): void {
  INSTANCES.set(instance, new UserScript(dynValueFromPtr(settings)));
}

export function change_settings(instance: InstanceId, settings: FatPtr): void {
  INSTANCES.get(instance).changeSettings(dynValueFromPtr(settings));
}

export function update(instance: InstanceId, timeElapsed: i64): void {
  INSTANCES.get(instance).update(timeIntervalFromAbi(timeElapsed));
}

declare namespace liquislime_api {
  export function level_width(): i32;
  export function level_height(): i32;

  export function get_current_unit(): UnitId;
  export function get_current_instance(): InstanceId;

  // export function get_own_faction(): FactionId;
  // export function get_own_position(): PackedData;

  // export function get_slime_amount(
  //   faction: FactionId,
  //   position: PackedData
  // ): i64;
  // export function set_slime_amount(
  //   faction: FactionId,
  //   position: PackedData,
  //   amount: i64
  // ): void;

  export function get_mouse_position(): PackedData;
  // export function is_mouse_pressed(): bool;

  export function log(message: FatPtr): void;
}

@external("liquislime_api", "get_own_faction")
declare function getOwnFaction(): FactionId;

@external("liquislime_api", "get_own_position")
declare function getOwnPosition(): PackedData;

@external("liquislime_api", "get_slime_amount")
declare function getSlimeAmount(faction: FactionId, position: PackedData): i64;

@external("liquislime_api", "set_slime_amount")
declare function setSlimeAmount(faction: FactionId, position: PackedData, amount: i64): void;

@external("liquislime_api", "is_mouse_pressed")
declare function isMousePressed(): bool;

export { liquislime_api, getOwnFaction, getOwnPosition, getSlimeAmount, setSlimeAmount, isMousePressed };

export function allocate_bytes(len: u32): FatPtr {
  return allocateBytes(len);
}

export function free_bytes(bytes: FatPtr): void {
  freeBytes(bytes);
}
