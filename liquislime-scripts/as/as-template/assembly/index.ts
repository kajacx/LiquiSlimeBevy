import { TimeInterval } from "./types";

import {
  allocate_bytes,
  change_settings,
  default_settings,
  describe_settings,
  free_bytes,
  init,
  new_instance,
  update,
} from "./bindings";

export {
  allocate_bytes,
  change_settings,
  default_settings,
  describe_settings,
  free_bytes,
  init,
  new_instance,
  update,
};

interface ScriptTemplate<T> {
  changeSettings(settings: T): void;
  update(timeElapsed: TimeInterval): void;
}
