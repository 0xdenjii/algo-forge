/**
 * LeetCode 146 - LRU Cache
 * Design a cache with capacity C supporting get(key) and put(key, value) in O(1).
 * When full, evict the least recently used key.
 * Approach: JS Map preserves insertion order; re-insert on access, evict the first key.
 * Time: O(1) per op. Space: O(C).
 */
export class LRUCache<K, V> {
  private map = new Map<K, V>();
  constructor(private readonly capacity: number) {}

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
    if (this.map.size > this.capacity) {
      const oldest = this.map.keys().next().value as K;
      this.map.delete(oldest);
    }
  }
}

const c = new LRUCache<number, number>(2);
c.put(1, 1);
c.put(2, 2);
console.log(c.get(1)); // 1
c.put(3, 3);
console.log(c.get(2)); // undefined
