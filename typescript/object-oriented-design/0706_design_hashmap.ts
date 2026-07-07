/**
 * LeetCode 706 - Design HashMap
 * Implement put, get (-1 if absent) and remove for integer keys without built-in hash tables.
 * Approach: separate chaining with [key, value] pairs; double the table when load factor exceeds 0.75.
 * Time: O(1) average. Space: O(n).
 */
export class MyHashMap {
  private buckets: Array<Array<[number, number]>> = Array.from({ length: 16 }, () => []);
  private count = 0;

  private idx(key: number): number {
    return (Math.imul(key, 0x9e3779b1) >>> 0) % this.buckets.length;
  }
  private grow(): void {
    const old = this.buckets;
    this.buckets = Array.from({ length: old.length * 2 }, () => []);
    for (const b of old) for (const [k, v] of b) this.buckets[this.idx(k)].push([k, v]);
  }

  put(key: number, value: number): void {
    const b = this.buckets[this.idx(key)];
    const e = b.find((p) => p[0] === key);
    if (e) {
      e[1] = value;
      return;
    }
    b.push([key, value]);
    if (++this.count > this.buckets.length * 0.75) this.grow();
  }
  get(key: number): number {
    return this.buckets[this.idx(key)].find((p) => p[0] === key)?.[1] ?? -1;
  }
  remove(key: number): void {
    const b = this.buckets[this.idx(key)];
    const i = b.findIndex((p) => p[0] === key);
    if (i >= 0) {
      b.splice(i, 1);
      this.count--;
    }
  }
}

const hm = new MyHashMap();
hm.put(1, 1);
hm.put(2, 2);
console.log(hm.get(1), hm.get(3)); // 1 -1
hm.put(2, 1);
console.log(hm.get(2)); // 1
hm.remove(2);
console.log(hm.get(2)); // -1
