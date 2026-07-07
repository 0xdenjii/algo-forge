/**
 * LeetCode 380 - Insert Delete GetRandom O(1)
 * Implement a set with insert, remove and getRandom, each in average O(1);
 * getRandom returns every present element with equal probability.
 * Approach: dense array of values + Map value -> index; remove by swapping with the last element.
 * Time: O(1) average per op. Space: O(n).
 */
export class RandomizedSet {
  private readonly vals: number[] = [];
  private readonly pos = new Map<number, number>();

  insert(val: number): boolean {
    if (this.pos.has(val)) return false;
    this.pos.set(val, this.vals.length);
    this.vals.push(val);
    return true;
  }
  remove(val: number): boolean {
    const i = this.pos.get(val);
    if (i === undefined) return false;
    const last = this.vals[this.vals.length - 1];
    this.vals[i] = last;
    this.pos.set(last, i);
    this.vals.pop();
    this.pos.delete(val);
    return true;
  }
  getRandom(): number {
    return this.vals[Math.floor(Math.random() * this.vals.length)];
  }
}

const rs = new RandomizedSet();
console.log(rs.insert(1), rs.remove(2), rs.insert(2)); // true false true
console.log([1, 2].includes(rs.getRandom())); // true
console.log(rs.remove(1), rs.insert(2), rs.getRandom()); // true false 2
