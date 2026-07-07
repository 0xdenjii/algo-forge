/**
 * LeetCode 460 - LFU Cache
 * Design a cache evicting the least frequently used key; ties broken by least recently used.
 * get and put must run in O(1) average.
 * Approach: key -> {value, freq}; freq -> insertion-ordered Set of keys (order = recency); track minFreq.
 * Time: O(1) per op. Space: O(capacity).
 */
export class LFUCache {
  private readonly vals = new Map<number, { value: number; freq: number }>();
  private readonly buckets = new Map<number, Set<number>>();
  private minFreq = 0;
  constructor(private readonly capacity: number) {}

  private addToBucket(freq: number, key: number): void {
    let b = this.buckets.get(freq);
    if (!b) {
      b = new Set();
      this.buckets.set(freq, b);
    }
    b.add(key);
  }

  private touch(key: number): void {
    const node = this.vals.get(key)!;
    const bucket = this.buckets.get(node.freq)!;
    bucket.delete(key);
    if (bucket.size === 0) {
      this.buckets.delete(node.freq);
      if (this.minFreq === node.freq) this.minFreq++;
    }
    node.freq++;
    this.addToBucket(node.freq, key);
  }

  get(key: number): number {
    if (!this.vals.has(key)) return -1;
    this.touch(key);
    return this.vals.get(key)!.value;
  }

  put(key: number, value: number): void {
    if (this.capacity <= 0) return;
    if (this.vals.has(key)) {
      this.vals.get(key)!.value = value;
      this.touch(key);
      return;
    }
    if (this.vals.size >= this.capacity) {
      const bucket = this.buckets.get(this.minFreq)!;
      const evict = bucket.values().next().value as number;
      bucket.delete(evict);
      if (bucket.size === 0) this.buckets.delete(this.minFreq);
      this.vals.delete(evict);
    }
    this.vals.set(key, { value, freq: 1 });
    this.addToBucket(1, key);
    this.minFreq = 1;
  }
}

const lfu = new LFUCache(2);
lfu.put(1, 1);
lfu.put(2, 2);
console.log(lfu.get(1)); // 1
lfu.put(3, 3); // evicts 2 (freq 1)
console.log(lfu.get(2), lfu.get(3)); // -1 3
lfu.put(4, 4); // 1 and 3 both freq 2; 1 is least recent -> evicted
console.log(lfu.get(1), lfu.get(3), lfu.get(4)); // -1 3 4
