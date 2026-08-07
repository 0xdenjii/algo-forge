/**
 * LeetCode 705 - Design HashSet
 * Implement add/remove/contains for integer keys without built-in hash tables.
 * Approach: separate chaining over a prime number of buckets.
 * Time: O(n / B) average. Space: O(n + B).
 */
export class MyHashSet {
  private static readonly B = 769;
  private buckets: number[][] = Array.from({ length: MyHashSet.B }, () => []);
  private bucket(k: number): number[] {
    return this.buckets[k % MyHashSet.B];
  }
  add(k: number): void {
    const b = this.bucket(k);
    if (!b.includes(k)) b.push(k);
  }
  remove(k: number): void {
    const b = this.bucket(k);
    const i = b.indexOf(k);
    if (i >= 0) b.splice(i, 1);
  }
  contains(k: number): boolean {
    return this.bucket(k).includes(k);
  }
}

const hs = new MyHashSet();
hs.add(1);
hs.add(2);
hs.remove(2);
console.log(hs.contains(1), hs.contains(2)); // true false
