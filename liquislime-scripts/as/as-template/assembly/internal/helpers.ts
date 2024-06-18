import { Position, TilePosition } from "../api/types";

export type FatPtr = u64;
export type PackedData = u64;

export function packU32s(num1: u32, num2: u32): u64 {
  return (num1 as u64) | ((num2 as u64) << 32);
}

export function unpackU32s(packed: u64): u32[] {
  const num1 = packed as u32;
  const num2 = (packed >> 32) as u32;
  return [num1, num2];
}

export function createFatPtr(addr: u32, len: u32): FatPtr {
  return packU32s(addr, len);
}

export function fatPtrAddr(ptr: FatPtr): u32 {
  const unpacked = unpackU32s(ptr);
  return unpacked[0];
}

export function fatPtrLen(ptr: FatPtr): u32 {
  const unpacked = unpackU32s(ptr);
  return unpacked[1];
}

export function packTilePosition(position: TilePosition): PackedData {
  return packU32s(position.x as u32, position.y as u32);
}

export function unpackTilePosition(packed: PackedData): TilePosition {
  const unpacked = unpackU32s(packed);
  return new TilePosition(unpacked[0] as i32, unpacked[1] as i32);
}

export function packPosition(position: TilePosition | null): PackedData {
  if (position == null) {
    return packU32s(reinterpret<u32>(f32.NaN), reinterpret<u32>(f32.NaN));
  } else {
    return packU32s(changetype<u32>(position.x), changetype<u32>(position.y));
  }
}

export function unpackPosition(packed: PackedData): Position {
  const unpacked = unpackU32s(packed);
  return new Position(
    reinterpret<f32>(unpacked[0]),
    reinterpret<f32>(unpacked[1])
  );
}

export function unpackPositionNullable(packed: PackedData): Position | null {
  const unpacked = unpackU32s(packed);
  const x = reinterpret<f32>(unpacked[0]);
  const y = reinterpret<f32>(unpacked[1]);
  if (isNaN(x) && isNaN(y)) {
    return null;
  } else {
    return new Position(x, y);
  }
}
