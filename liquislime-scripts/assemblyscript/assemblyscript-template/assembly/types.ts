export type FatPtr = u64;

export type FactionId = u32;
export type UnitId = i32;
export type InstanceId = i32;

export type PackedData = u64;

const ONE_SLIME_AMOUNT: i64 = u32.MAX_VALUE as i64;

export type SlimeAmount = f64;

export function slimeAmountToAbi(amount: SlimeAmount): i64 {
  return Math.round(amount * (ONE_SLIME_AMOUNT as f64)) as i64;
}

export function slimeAmountFromAbi(abi: i64): SlimeAmount {
  return (abi as f64) / (ONE_SLIME_AMOUNT as f64);
}

const FRAGMENTS_IN_SECOND: i64 = 18_000;

export class TimeInterval {
  private fragments: i64;

  private constructor(fragments: i64) {
    this.fragments = fragments;
  }

  static fromSeconds(seconds: f64): TimeInterval {
    return new TimeInterval(Math.round(seconds * (FRAGMENTS_IN_SECOND as f64)));
  }
}
