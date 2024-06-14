export * from "./bindings";

import { DynValue } from "./dyn_value";

export function add(a: i32, b: i32): i32 {
  return a + b + (DynValue.none().asNumber().getOr(5) as i32);
}
