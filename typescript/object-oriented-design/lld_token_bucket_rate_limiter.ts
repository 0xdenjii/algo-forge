/**
 * LLD - Rate Limiter (Token Bucket + Sliding Window Log, Strategy pattern)
 * Limit each client to a configured request rate. Callers ask allow(clientId) and get a boolean.
 * Algorithms must be swappable behind one interface and testable with an injected clock.
 * Approach: RateLimiter interface; TokenBucket refills lazily from elapsed time; SlidingWindowLog keeps timestamps.
 * Time: token bucket O(1); sliding log O(1) amortized. Space: O(clients) / O(clients * limit).
 */
export type Clock = () => number; // milliseconds

export interface RateLimiter {
  allow(clientId: string): boolean;
}

export class TokenBucketLimiter implements RateLimiter {
  private readonly buckets = new Map<string, { tokens: number; last: number }>();
  constructor(
    private readonly capacity: number,
    private readonly refillPerSec: number,
    private readonly clock: Clock = Date.now,
  ) {}

  allow(clientId: string): boolean {
    const now = this.clock();
    const b = this.buckets.get(clientId) ?? { tokens: this.capacity, last: now };
    b.tokens = Math.min(this.capacity, b.tokens + ((now - b.last) / 1000) * this.refillPerSec);
    b.last = now;
    this.buckets.set(clientId, b);
    if (b.tokens < 1) return false;
    b.tokens -= 1;
    return true;
  }
}

export class SlidingWindowLogLimiter implements RateLimiter {
  private readonly logs = new Map<string, number[]>();
  constructor(
    private readonly limit: number,
    private readonly windowMs: number,
    private readonly clock: Clock = Date.now,
  ) {}

  allow(clientId: string): boolean {
    const now = this.clock();
    const log = this.logs.get(clientId) ?? [];
    while (log.length && now - log[0] >= this.windowMs) log.shift();
    this.logs.set(clientId, log);
    if (log.length >= this.limit) return false;
    log.push(now);
    return true;
  }
}

let fakeNow = 0;
const clock: Clock = () => fakeNow;
const limiters: Record<string, RateLimiter> = {
  bucket: new TokenBucketLimiter(3, 1, clock),
  window: new SlidingWindowLogLimiter(3, 1000, clock),
};
for (const [name, rl] of Object.entries(limiters)) {
  fakeNow = 0;
  const burst = [1, 2, 3, 4].map(() => rl.allow("u1"));
  fakeNow = 1000;
  console.log(name, burst, rl.allow("u1"), rl.allow("u2"));
}
// bucket [true,true,true,false] true true
// window [true,true,true,false] true true
