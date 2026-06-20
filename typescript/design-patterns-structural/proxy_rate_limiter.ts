/**
 * Title: Proxy - Rate-limiting Proxy (token bucket; cf. LeetCode 359 Logger Rate Limiter)
 * Problem: Protect an external SearchApi from abuse: each client may make at most `capacity`
 *   requests in a burst, refilling at `ratePerSec` tokens per second. Excess calls fail fast.
 * Approach: Per-client token bucket stored in a Map; the proxy implements the API interface
 *   and refills lazily based on elapsed time (no timers needed).
 * Complexity: O(1) per request; O(clients) space.
 */
interface SearchApi { search(clientId: string, q: string): string }

class RealSearchApi implements SearchApi {
  search(_clientId: string, q: string): string { return `results for "${q}"`; }
}

class RateLimitExceeded extends Error {
  constructor(readonly retryAfterMs: number) { super(`rate limited, retry in ${Math.ceil(retryAfterMs)}ms`); }
}

class RateLimitingProxy implements SearchApi {
  private buckets = new Map<string, { tokens: number; last: number }>();
  constructor(private inner: SearchApi, private capacity: number, private ratePerSec: number, private now = () => Date.now()) {}
  search(clientId: string, q: string): string {
    const t = this.now();
    const b = this.buckets.get(clientId) ?? { tokens: this.capacity, last: t };
    b.tokens = Math.min(this.capacity, b.tokens + ((t - b.last) / 1000) * this.ratePerSec);
    b.last = t;
    this.buckets.set(clientId, b);
    if (b.tokens < 1) throw new RateLimitExceeded(((1 - b.tokens) / this.ratePerSec) * 1000);
    b.tokens -= 1;
    return this.inner.search(clientId, q);
  }
}

let clock = 0;
const api: SearchApi = new RateLimitingProxy(new RealSearchApi(), 3, 2, () => clock);
const attempt = (who: string, q: string) => {
  try { console.log(`t=${clock} ${who}:`, api.search(who, q)); }
  catch (e) { console.log(`t=${clock} ${who}:`, (e as Error).message); }
};
for (let i = 0; i < 4; i++) attempt("alice", `q${i}`); // 3 ok, 4th limited
attempt("bob", "hello");                               // separate bucket
clock += 500;                                           // +1 token
attempt("alice", "again");
attempt("alice", "too soon");
