import { FactionId, Position, SlimeAmount, TilePosition } from "./types";
import * as bindings from "../internal/bindings";
import { packPosition, unpackPositionNullable } from "../internal/helpers";
import { writeString } from "../internal/memory";

export function getSlimeAmount(
  faction: FactionId,
  position: TilePosition
): SlimeAmount {
  return bindings.getSlimeAmount(faction, packPosition(position));
}

export function setSlimeAmount(
  faction: FactionId,
  position: TilePosition,
  amount: f64
): void {
  bindings.setSlimeAmount(faction, packPosition(position), amount);
}

export function addSlimeAmount(
  faction: FactionId,
  position: TilePosition,
  amount: SlimeAmount
): SlimeAmount {
  const newAmount = getSlimeAmount(faction, position) + amount;
  setSlimeAmount(faction, position, newAmount);
  return newAmount;
}

export function setSlimeAmountAtLeast(
  faction: FactionId,
  position: Position,
  amount: SlimeAmount
): SlimeAmount {
  const newAmount = Math.max(getSlimeAmount(faction, position), amount);
  setSlimeAmount(faction, position, newAmount);
  return newAmount;
}

export function isMousePressed(): bool {
  return bindings.isMousePressed();
}

export function getMousePosition(): Position | null {
  return unpackPositionNullable(bindings.getMousePosition());
}

export function getOwnFaction(): FactionId {
  return bindings.getOwnFaction();
}

export function log(message: string): void {
  bindings.log(writeString(message));
}
