/**
 * LeetCode 706 - Design HashMap
 * Implement put/get/remove for integer keys and values without built-in hash tables.
 * Approach: separate chaining with [key, value] pairs per bucket.
 * Time: O(n / B) average. Space: O(n + B).
 */
export class MyHashMap {
  private static readonly B = 2069;
  private buckets: [number, number][][] = Array.from({ length: MyHashMap.B }, () => []);

  put(k: number, v: number): void {
    const b = this.buckets[k % MyHashMap.B];
    const e = b.find(p => p[0] === k);
    if (e) e[1] = v;
    else b.push([k, v]);
  }
  get(k: number): number {
    return this.buckets[k % MyHashMap.B].find(p => p[0] === k)?.[1] ?? -1;
  }
  remove(k: number): void {
    const b = this.buckets[k % MyHashMap.B];
    const i = b.findIndex(p => p[0] === k);
    if (i >= 0) b.splice(i, 1);
  }
}

const hm = new MyHashMap();
hm.put(1, 1);
hm.put(2, 2);
hm.remove(2);
console.log(hm.get(1), hm.get(2)); // 1 -1
