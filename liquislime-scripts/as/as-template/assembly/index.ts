import { TimeInterval } from "./api/types";

import {
  allocate_bytes,
  change_settings,
  describe_settings,
  free_bytes,
  init,
  new_instance,
  update,
} from "./internal/bindings";

export {
  allocate_bytes,
  change_settings,
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
