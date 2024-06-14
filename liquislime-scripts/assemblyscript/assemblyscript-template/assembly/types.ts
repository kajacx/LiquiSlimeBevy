export type FatPtr = u64;

export type FactionId = u32;
export type UnitId = i32;
export type InstanceId = i32;

export type PackedData = u64;

const ONE_SLIME_AMOUNT: i64 = u32.MAX_VALUE as i64;

export class SlimeAmount {
  private amount: i64;

  private constructor(amount: i64) {
    this.amount = amount;
  }

  static from(amount: number): SlimeAmount {
    return new SlimeAmount(Math.round(amount * ONE_SLIME_AMOUNT) as i64);
  }
}
