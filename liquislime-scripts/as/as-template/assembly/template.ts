import { DynValue } from "./api/dyn_value";
import { TimeInterval } from "./api/types";

export interface ScriptTemplate {
  changeSettings(settings: DynValue): void;
  update(timeElapsed: TimeInterval): void;
}
