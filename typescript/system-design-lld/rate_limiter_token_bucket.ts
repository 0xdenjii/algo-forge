/**
 * LLD - Rate Limiter (token bucket + sliding window log)
 * Limit each client to a configured request rate. Token bucket allows bursts up to capacity and
 * refills continuously; sliding window log allows at most N requests in any window of W ms.
 * Approach: Strategy interface with two implementations, keyed per client.
 * Time: token bucket O(1); sliding log O(1) amortized. Space: O(clients) / O(clients * N).
 */
export interface RateLimiter {
  allow(client: string, now?: number): boolean;
}

export class TokenBucketLimiter implements RateLimiter {
  private buckets = new Map<string, { tokens: number; last: number }>();
  constructor(private capacity: number, private refillPerSec: number) {}
  allow(client: string, now = Date.now()): boolean {
    const b = this.buckets.get(client) ?? { tokens: this.capacity, last: now };
    b.tokens = Math.min(this.capacity, b.tokens + ((now - b.last) / 1000) * this.refillPerSec);
    b.last = now;
    this.buckets.set(client, b);
    if (b.tokens < 1) return false;
    b.tokens -= 1;
    return true;
  }
}

export class SlidingWindowLogLimiter implements RateLimiter {
  private logs = new Map<string, number[]>();
  constructor(private limit: number, private windowMs: number) {}
  allow(client: string, now = Date.now()): boolean {
    const log = this.logs.get(client) ?? [];
    let i = 0;
    while (i < log.length && log[i] <= now - this.windowMs) i++;
    if (i) log.splice(0, i);
    this.logs.set(client, log);
    if (log.length >= this.limit) return false;
    log.push(now);
    return true;
  }
}

const tb = new TokenBucketLimiter(3, 1);
console.log([0, 0, 0, 0, 1000, 1500].map(t => tb.allow("u", t))); // [true,true,true,false,true,false]
const sw = new SlidingWindowLogLimiter(2, 1000);
console.log([0, 100, 200, 1000, 1101].map(t => sw.allow("u", t))); // [true,true,false,true,true]
