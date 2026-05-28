/**
 * Title: Lazy Initialization (Lazy<T> holder)
 * Problem: Defer an expensive computation until first access and memoize it;
 *   if the factory throws, the next access should retry.
 * Approach: generic Lazy<T> storing the factory and a tagged state (only set on success).
 * Complexity: O(cost of factory) once, then O(1).
 */
export class Lazy<T> {
  private state: { done: true; value: T } | { done: false } = { done: false };
  constructor(private readonly factory: () => T) {}
  get value(): T {
    if (!this.state.done) this.state = { done: true, value: this.factory() };
    return this.state.value;
  }
  get isCreated(): boolean { return this.state.done; }
}

let calls = 0;
const primes = new Lazy(() => {
  calls++;
  const sieve = new Array<boolean>(100).fill(true); const out: number[] = [];
  for (let i = 2; i < 100; i++) if (sieve[i]) { out.push(i); for (let j = i * i; j < 100; j += i) sieve[j] = false; }
  return out;
});
console.log(primes.isCreated, primes.value.length, primes.value.length, "calls", calls); // false 25 25 calls 1

let attempts = 0;
const flaky = new Lazy(() => { if (++attempts < 2) throw new Error("boom"); return "ok"; });
try { flaky.value; } catch { /* first attempt fails */ }
console.log(flaky.value, attempts); // ok 2
