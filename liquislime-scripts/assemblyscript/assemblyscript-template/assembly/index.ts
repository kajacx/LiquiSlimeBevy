import { TimeInterval } from "./types";

export * from "./bindings";

interface ScriptTemplate<T> {
  changeSettings(settings: T): void;
  update(timeElapsed: TimeInterval): void;
}
