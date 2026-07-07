/**
 * LeetCode 705 - Design HashSet
 * Implement add, remove and contains for non-negative integer keys without built-in hash tables.
 * Approach: separate chaining over a fixed prime number of buckets; swap-remove inside a bucket.
 * Time: O(n / buckets) average. Space: O(buckets + n).
 */
export class MyHashSet {
  private static readonly SIZE = 769;
  private readonly buckets: number[][] = Array.from({ length: MyHashSet.SIZE }, () => []);

  private bucket(key: number): number[] {
    return this.buckets[key % MyHashSet.SIZE];
  }
  add(key: number): void {
    const b = this.bucket(key);
    if (!b.includes(key)) b.push(key);
  }
  remove(key: number): void {
    const b = this.bucket(key);
    const i = b.indexOf(key);
    if (i >= 0) {
      b[i] = b[b.length - 1];
      b.pop();
    }
  }
  contains(key: number): boolean {
    return this.bucket(key).includes(key);
  }
}

const hs = new MyHashSet();
hs.add(1);
hs.add(2);
console.log(hs.contains(1), hs.contains(3)); // true false
hs.remove(2);
console.log(hs.contains(2)); // false
