import { DynValue } from "./dyn_value";
import { TimeInterval } from "./types";

export interface ScriptTemplate {
  changeSettings(settings: DynValue): void;
  update(timeElapsed: TimeInterval): void;
}
