import { Encoder, Sizer, Writer } from "@wapc/as-msgpack";
import { createFatPtr, fatPtrAddr } from "./helpers";
import { FatPtr } from "./types";
// import { memory, heap } from "bindings/memory";

// const OBJECTS = new Map<u32, Uint8Array>();
// let nextFreeId = 0;

// export function allocateBytes(len: u32) {
//   nextFreeId = nextFreeId + 1; // Start from 1
//   const bytes = new Uint8Array(len);
//   OBJECTS.set(nextFreeId, bytes);
//   return createFatPtr(nextFreeId, len);
// }

// export function freeBytes(ptr: FatPtr): void {
//   OBJECTS.delete(fatPtrAddr(ptr));
// }

// export function writeArrayBuffer(buffer: ArrayBuffer): FatPtr {
//   let ptr = allocateBytes(buffer.byteLength);
// }

export function allocateBytes(len: u32): FatPtr {
  const ptr = heap.alloc(len);
  // const ptr = __new(len, idof<ArrayBuffer>());
  return createFatPtr(ptr as u32, len);
}

export function freeBytes(ptr: FatPtr): void {
  let addr = fatPtrAddr(ptr);
  heap.free(addr);
  // __free(addr as usize);
}

export function writeBytes(bytes: Uint8Array): FatPtr {
  const allocated = allocateBytes(bytes.length);
  // @ts-ignore
  memory.store(fatPtrAddr(allocated), 0, bytes);
  return allocated;
}

export function writeArrayBuffer(bytes: ArrayBuffer): FatPtr {
  const allocated = allocateBytes(bytes.byteLength);
  // @ts-ignore
  // memory.store(fatPtrAddr(allocated), 0, bytes);
  // memory.copy(fatPtrAddr(allocated), bytes., bytes);
  // memory.
  // heap.
  // memory.fill(fatPtrAddr(allocated), 0, bytes);
  memory.copy(
    fatPtrAddr(allocated),
    changetype<usize>(bytes),
    bytes.byteLength
  );
  return allocated;
}

export function encodeToMemory(callback: (writer: Writer) => void): FatPtr {
  const sizer = new Sizer();
  callback(sizer);
  const buffer = new ArrayBuffer(sizer.length);
  callback(new Encoder(buffer));
  return writeArrayBuffer(buffer);
}
