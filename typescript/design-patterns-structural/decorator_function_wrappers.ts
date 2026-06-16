/**
 * Title: Decorator - Higher-order Function Wrappers (memoize, timing, logging)
 * Problem: Add cross-cutting behaviour to plain functions without changing them: logging of
 *   calls, timing, and memoization, composable in any order and type-preserving.
 * Approach: Generic higher-order functions `(fn) => fn'` with the same signature; a
 *   `compose` helper stacks them.
 * Complexity: memoize O(1) average lookup with O(distinct args) space; others O(1) overhead.
 */
type Fn<A extends unknown[], R> = (...args: A) => R;
type Wrapper = <A extends unknown[], R>(fn: Fn<A, R>) => Fn<A, R>;

const withLogging = (name: string, log: string[]): Wrapper => (fn) => (...args) => {
  const r = fn(...args);
  log.push(`${name}(${args.join(", ")}) = ${String(r)}`);
  return r;
};

const withTiming = (sink: number[]): Wrapper => (fn) => (...args) => {
  const t0 = performance.now();
  try { return fn(...args); } finally { sink.push(performance.now() - t0); }
};

const memoize: Wrapper = (fn) => {
  const cache = new Map<string, ReturnType<typeof fn>>();
  return (...args) => {
    const key = JSON.stringify(args);
    if (!cache.has(key)) cache.set(key, fn(...args));
    return cache.get(key)!;
  };
};

const compose = (...ws: Wrapper[]): Wrapper => (fn) => ws.reduceRight((acc, w) => w(acc), fn);

// Memoized recursive Fibonacci: the recursion goes through the decorated binding.
const log: string[] = [];
const times: number[] = [];
const fib: (n: number) => number = compose(withTiming(times), memoize)((n: number): number => (n < 2 ? n : fib(n - 1) + fib(n - 2)));
console.log(fib(80)); // 23416728348467685
const add = withLogging("add", log)((a: number, b: number) => a + b);
add(2, 3); add(10, -4);
console.log(log);
console.log(`timed ${times.length} calls`);
