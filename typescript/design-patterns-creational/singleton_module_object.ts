/**
 * Title: Module-scoped Singleton (Idiomatic TS)
 * Problem: Implement a counter service as a singleton without class ceremony.
 * Approach: ES modules are cached, so a frozen object literal exported from a
 *   module is already a singleton; closure keeps state private.
 * Complexity: O(1) per op, O(1) space.
 */
let count = 0;
export const counter = Object.freeze({
  increment: (): number => ++count,
  reset: (): void => { count = 0; },
  get value(): number { return count; },
});

counter.increment(); counter.increment();
console.log(counter.value); // 2
