import { FatPtr } from "./types";

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
