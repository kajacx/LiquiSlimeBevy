export type FatPtr = u64;

export type FactionId = u32;
export type UnitId = i32;
export type InstanceId = i32;

export type SlimeAmount = f64;
export type TimeInterval = f64;

export type PackedData = u64;

export class TilePosition {
  x: i32;
  y: i32;

  constructor(x: i32, y: i32) {
    this.x = x;
    this.y = y;
  }
}

export class Position {
  x: f32;
  y: f32;

  constructor(x: f32, y: f32) {
    this.x = x;
    this.y = y;
  }

  toTilePosition(): TilePosition {
    return new TilePosition(
      Math.floor(this.x) as i32,
      Math.floor(this.y) as i32
    );
  }
}
