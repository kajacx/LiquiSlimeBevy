import { Encoder, Sizer, Writer } from "@wapc/as-msgpack";
import { createFatPtr, fatPtrAddr, fatPtrLen } from "./helpers";
import { FatPtr } from "../api/types";

export function allocateBytes(len: u32): FatPtr {
  const ptr = heap.alloc(len);
  return createFatPtr(ptr as u32, len);
}

export function freeBytes(ptr: FatPtr): void {
  let addr = fatPtrAddr(ptr);
  heap.free(addr);
}

export function writeBytes(bytes: ArrayBuffer): FatPtr {
  const allocated = allocateBytes(bytes.byteLength);
  memory.copy(
    fatPtrAddr(allocated),
    changetype<usize>(bytes),
    bytes.byteLength
  );
  return allocated;
}

export function readArrayBuffer(ptr: FatPtr): ArrayBuffer {
  const addr = fatPtrAddr(ptr);
  const len = fatPtrLen(ptr);
  const buffer = new ArrayBuffer(len);
  memory.copy(changetype<usize>(buffer), addr, len);
  freeBytes(ptr);
  return buffer;
}

export function encodeToMemory(callback: (writer: Writer) => void): FatPtr {
  const sizer = new Sizer();
  callback(sizer);
  const buffer = new ArrayBuffer(sizer.length);
  callback(new Encoder(buffer));
  return writeBytes(buffer);
}

// const ENCODER = new TextEncoder("UTF-8");
export function writeString(text: string): FatPtr {
  // text.
  // const bytes = ENCODER.encode(text);
  // // const bytes = new Uint8Array(Buffer.from(text, "utf-8"));
  // return writeBytes(bytes.buffer);
  throw "TODO: string to bytes";
}
