import { allocateBytes, encodeToMemory, freeBytes, writeBytes } from "./memory";
import { SETTINGS_DEFINITION } from "./script";
import { FactionId, FatPtr, InstanceId, PackedData, UnitId } from "./types";
import { DynValue } from "./dyn_value";
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

export function new_instance(instance: InstanceId, settings: FatPtr): void {}

export function change_settings(instance: InstanceId, settings: FatPtr): void {}

export function update(instance: InstanceId, time_elapsed: i64): void {}

export declare namespace liquislime_api {
  export function level_width(): i32;
  export function level_height(): i32;

  export function get_current_unit(): UnitId;
  export function get_current_instance(): InstanceId;

  export function get_own_faction(): FactionId;
  export function get_own_position(): PackedData;

  export function get_slime_amount(
    faction: FactionId,
    position: PackedData
  ): i64;
  export function set_slime_amount(
    faction: FactionId,
    position: PackedData,
    amount: i64
  ): void;

  export function get_mouse_position(): PackedData;
  export function is_mouse_pressed(): bool;

  export function log(message: FatPtr): void;
}

export function allocate_bytes(len: u32): FatPtr {
  return allocateBytes(len);
}

export function free_bytes(bytes: FatPtr): void {
  freeBytes(bytes);
}
