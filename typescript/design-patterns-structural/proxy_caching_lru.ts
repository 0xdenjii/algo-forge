/**
 * Title: Proxy - Caching Proxy with LRU Eviction and TTL (uses LeetCode 146 LRU Cache)
 * Problem: A slow WeatherService.forecast(city) should be fronted by a cache that holds at
 *   most `capacity` entries (least-recently-used eviction) and expires entries after a TTL.
 * Approach: Map preserves insertion order, so delete+set moves a key to the "recent" end;
 *   the first key is the LRU victim. The proxy implements the service interface.
 * Complexity: O(1) average per get/put; O(capacity) space.
 */
class LRUCache<K, V> {
  private map = new Map<K, V>();
  constructor(private capacity: number) {
    if (capacity <= 0) throw new RangeError("capacity must be positive");
  }
  get(key: K): V | undefined {
    if (!this.map.has(key)) return undefined;
    const v = this.map.get(key)!;
    this.map.delete(key);
    this.map.set(key, v);
    return v;
  }
  put(key: K, value: V): void {
    this.map.delete(key);
    this.map.set(key, value);
    if (this.map.size > this.capacity) this.map.delete(this.map.keys().next().value as K);
  }
  delete(key: K): void { this.map.delete(key); }
}

interface WeatherService { forecast(city: string): Promise<string> }

class SlowWeatherService implements WeatherService {
  calls = 0;
  async forecast(city: string): Promise<string> {
    this.calls++;
    await new Promise((r) => setTimeout(r, 10));
    return `${city}: ${(city.length * 7) % 30}C`;
  }
}

class CachingWeatherProxy implements WeatherService {
  private cache: LRUCache<string, { value: Promise<string>; expires: number }>;
  constructor(private inner: WeatherService, capacity: number, private ttlMs: number, private now = () => Date.now()) {
    this.cache = new LRUCache(capacity);
  }
  forecast(city: string): Promise<string> {
    const key = city.toLowerCase();
    const hit = this.cache.get(key);
    if (hit && hit.expires > this.now()) return hit.value;
    const value = this.inner.forecast(city);
    value.catch(() => this.cache.delete(key)); // don't cache failures
    this.cache.put(key, { value, expires: this.now() + this.ttlMs });
    return value;
  }
}

(async () => {
  let clock = 0;
  const real = new SlowWeatherService();
  const proxy = new CachingWeatherProxy(real, 2, 1000, () => clock);
  await proxy.forecast("Paris"); await proxy.forecast("paris"); // hit
  await proxy.forecast("Oslo"); await proxy.forecast("Rome");   // evicts Paris
  await proxy.forecast("Paris");                                // miss
  console.log("backend calls:", real.calls); // 4
  clock += 2000;
  await proxy.forecast("Paris");                                // expired -> miss
  console.log("backend calls:", real.calls); // 5

  const lru = new LRUCache<number, number>(2); // LeetCode 146 example
  lru.put(1, 1); lru.put(2, 2); console.log(lru.get(1)); lru.put(3, 3); console.log(lru.get(2));
  lru.put(4, 4); console.log(lru.get(1), lru.get(3), lru.get(4)); // 1, undefined, undefined 3 4
})();
