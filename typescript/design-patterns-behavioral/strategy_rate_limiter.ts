/**
 * Title: Strategy - Pluggable Rate Limiter (Token Bucket / Sliding Window / Fixed Window)
 * Problem: An API gateway limits requests per client. The algorithm must be
 *   configurable per route without changing gateway code. Time is injected
 *   so behaviour is deterministic and testable.
 * Approach: RateLimitStrategy interface with three implementations; the
 *   gateway keeps one strategy instance per client key.
 * Complexity: token bucket and fixed window O(1); sliding log O(limit) amortized.
 */
interface RateLimitStrategy { allow(nowMs: number): boolean }

class TokenBucket implements RateLimitStrategy {
  private tokens: number; private last = 0;
  constructor(private capacity: number, private refillPerSec: number) { this.tokens = capacity; }
  allow(now: number) {
    this.tokens = Math.min(this.capacity, this.tokens + ((now - this.last) / 1000) * this.refillPerSec);
    this.last = now;
    if (this.tokens >= 1) { this.tokens -= 1; return true; }
    return false;
  }
}

class SlidingWindowLog implements RateLimitStrategy {
  private log: number[] = []; private head = 0;
  constructor(private limit: number, private windowMs: number) {}
  allow(now: number) {
    while (this.head < this.log.length && this.log[this.head] <= now - this.windowMs) this.head++;
    if (this.head > 1024) { this.log = this.log.slice(this.head); this.head = 0; }
    if (this.log.length - this.head >= this.limit) return false;
    this.log.push(now); return true;
  }
}

class FixedWindow implements RateLimitStrategy {
  private window = -1; private count = 0;
  constructor(private limit: number, private windowMs: number) {}
  allow(now: number) {
    const w = Math.floor(now / this.windowMs);
    if (w !== this.window) { this.window = w; this.count = 0; }
    return this.count < this.limit ? (this.count++, true) : false;
  }
}

class Gateway {
  private perClient = new Map<string, RateLimitStrategy>();
  constructor(private factory: () => RateLimitStrategy) {}
  handle(client: string, now: number): 200 | 429 {
    let s = this.perClient.get(client);
    if (!s) this.perClient.set(client, (s = this.factory()));
    return s.allow(now) ? 200 : 429;
  }
}

const times = [0, 100, 200, 300, 900, 1000, 1100, 1500];
for (const [name, f] of [
  ["token", () => new TokenBucket(3, 2)],
  ["sliding", () => new SlidingWindowLog(3, 1000)],
  ["fixed", () => new FixedWindow(3, 1000)],
] as const) {
  const g = new Gateway(f);
  console.log(name.padEnd(8), times.map((t) => g.handle("ip1", t)).join(" "));
}
