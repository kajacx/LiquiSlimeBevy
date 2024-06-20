export type FatPtr = u64;

export type FactionId = u32;
export type UnitId = u32;
export type InstanceId = u32;

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

  withX(x: i32): TilePosition {
    return new TilePosition(x, this.y);
  }

  withY(y: i32): TilePosition {
    return new TilePosition(this.x, y);
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
