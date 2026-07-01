/**
 * LeetCode 307 - Range Sum Query - Mutable
 * Given an integer array, support update(index, val) and sumRange(left, right) (inclusive).
 * Approach: Fenwick (binary indexed) tree storing deltas; keep a copy of values to compute deltas.
 * Time: O(log n) per op, O(n log n) build. Space: O(n).
 */
export class NumArray {
  private readonly tree: number[];
  private readonly vals: number[];

  constructor(nums: number[]) {
    this.vals = new Array<number>(nums.length).fill(0);
    this.tree = new Array<number>(nums.length + 1).fill(0);
    nums.forEach((v, i) => this.update(i, v));
  }

  update(index: number, val: number): void {
    const delta = val - this.vals[index];
    this.vals[index] = val;
    for (let i = index + 1; i < this.tree.length; i += i & -i) this.tree[i] += delta;
  }

  private prefix(count: number): number {
    let s = 0;
    for (let i = count; i > 0; i -= i & -i) s += this.tree[i];
    return s;
  }

  sumRange(left: number, right: number): number {
    return this.prefix(right + 1) - this.prefix(left);
  }
}

const na = new NumArray([1, 3, 5]);
console.log(na.sumRange(0, 2)); // 9
na.update(1, 2);
console.log(na.sumRange(0, 2), na.sumRange(1, 1)); // 8 2
