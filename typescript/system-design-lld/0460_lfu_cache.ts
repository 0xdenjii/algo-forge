/**
 * LeetCode 460 - LFU Cache
 * Cache that evicts the least frequently used key; ties broken by least recently used.
 * get/put must run in O(1).
 * Approach: key -> {value, freq}; freq -> insertion-ordered Set of keys; track minFreq.
 * Time: O(1) per op. Space: O(C).
 */
export class LFUCache {
  private vals = new Map<number, { v: number; f: number }>();
  private buckets = new Map<number, Set<number>>();
  private minFreq = 0;
  constructor(private readonly cap: number) {}

  private bucket(f: number): Set<number> {
    let b = this.buckets.get(f);
    if (!b) this.buckets.set(f, (b = new Set()));
    return b;
  }

  private touch(key: number): void {
    const e = this.vals.get(key)!;
    const b = this.buckets.get(e.f)!;
    b.delete(key);
    if (b.size === 0) {
      this.buckets.delete(e.f);
      if (this.minFreq === e.f) this.minFreq++;
    }
    e.f++;
    this.bucket(e.f).add(key);
  }

  get(key: number): number {
    if (!this.vals.has(key)) return -1;
    this.touch(key);
    return this.vals.get(key)!.v;
  }

  put(key: number, value: number): void {
    if (this.cap <= 0) return;
    if (this.vals.has(key)) {
      this.vals.get(key)!.v = value;
      this.touch(key);
      return;
    }
    if (this.vals.size >= this.cap) {
      const b = this.buckets.get(this.minFreq)!;
      const victim = b.values().next().value as number;
      b.delete(victim);
      if (b.size === 0) this.buckets.delete(this.minFreq);
      this.vals.delete(victim);
    }
    this.vals.set(key, { v: value, f: 1 });
    this.bucket(1).add(key);
    this.minFreq = 1;
  }
}

const l = new LFUCache(2);
l.put(1, 1);
l.put(2, 2);
l.get(1);
l.put(3, 3); // evicts 2
console.log(l.get(2), l.get(3)); // -1 3
