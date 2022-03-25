export { abort } from "./abort"

export function allocate(size: u32): u32 {
  let ptr = __alloc(size)
  __pin(ptr);
  store<ArrayBuffer>(ptr, changetype<ArrayBuffer>(ptr))
  return ptr;
}

export function read(ptr: u32): u32 {
  return changetype<u32>(load<ArrayBuffer>(ptr));
}