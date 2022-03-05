import { increment } from "./imports";

export function add(a: i64, b: i64): i64 {
  return increment(a + b);
}
