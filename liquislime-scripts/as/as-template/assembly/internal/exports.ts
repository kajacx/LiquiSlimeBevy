import { encodeToMemory } from "./memory";
import { SETTINGS_DEFINITION, UserScript } from "../script";
import { FatPtr, InstanceId } from "../api/types";
import { dynValueFromPtr } from "../api/dyn_value";

export function init(): void {}

export function describeSettings(): FatPtr {
  return encodeToMemory((writer) =>
    SETTINGS_DEFINITION.describeSettings().encode(writer)
  );
}

const INSTANCES = new Map<InstanceId, UserScript>();

export function newInstance(instance: InstanceId, settings: FatPtr): void {
  INSTANCES.set(instance, new UserScript(dynValueFromPtr(settings)));
}

export function changeSettings(instance: InstanceId, settings: FatPtr): void {
  INSTANCES.get(instance).changeSettings(dynValueFromPtr(settings));
}

export function update(instance: InstanceId, timeElapsed: f64): void {
  INSTANCES.get(instance).update(timeElapsed);
}
