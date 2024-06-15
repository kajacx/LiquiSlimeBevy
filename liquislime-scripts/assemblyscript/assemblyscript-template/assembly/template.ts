import { TimeInterval } from "./types";

export interface ScriptTemplate<T> {
  changeSettings(settings: T): void;
  update(timeElapsed: TimeInterval): void;
}
