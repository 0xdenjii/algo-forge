/**
 * LeetCode 146 - LRU Cache
 * Design a cache with fixed capacity supporting get(key) and put(key, value) in O(1).
 * When capacity is exceeded, evict the least recently used key.
 * Approach: JS Map preserves insertion order; delete + reinsert on access moves a key to the newest end.
 * Time: O(1) per op. Space: O(capacity).
 */
export class LRUCache {
  private readonly map = new Map<number, number>();
  constructor(private readonly capacity: number) {}

  get(key: number): number {
    const v = this.map.get(key);
    if (v === undefined) return -1;
    this.map.delete(key);
    this.map.set(key, v);
    return v;
  }

  put(key: number, value: number): void {
    if (this.map.has(key)) this.map.delete(key);
    this.map.set(key, value);
    if (this.map.size > this.capacity) {
      const oldest = this.map.keys().next().value as number;
      this.map.delete(oldest);
    }
  }
}

const c = new LRUCache(2);
c.put(1, 1);
c.put(2, 2);
console.log(c.get(1)); // 1
c.put(3, 3);
console.log(c.get(2)); // -1
c.put(4, 4);
console.log(c.get(1), c.get(3), c.get(4)); // -1 3 4
