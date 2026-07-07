/**
 * LeetCode 1146 - Snapshot Array
 * Array of length n (initially zeros) with set(index, val), snap() returning a snapshot id,
 * and get(index, snapId) returning the value at that index when the snapshot was taken.
 * Approach: per-index history of (snapId, value); binary search for the last entry with id <= snapId.
 * Time: set O(1) amortized, snap O(1), get O(log s). Space: O(n + sets).
 */
export class SnapshotArray {
  private snapId = 0;
  private readonly history: Array<Array<[snap: number, val: number]>>;

  constructor(length: number) {
    this.history = Array.from({ length }, () => [[0, 0]]);
  }

  set(index: number, val: number): void {
    const h = this.history[index];
    const last = h[h.length - 1];
    if (last[0] === this.snapId) last[1] = val;
    else h.push([this.snapId, val]);
  }

  snap(): number {
    return this.snapId++;
  }

  get(index: number, snapId: number): number {
    const h = this.history[index];
    let lo = 0;
    let hi = h.length - 1;
    while (lo < hi) {
      const mid = (lo + hi + 1) >> 1;
      if (h[mid][0] <= snapId) lo = mid;
      else hi = mid - 1;
    }
    return h[lo][1];
  }
}

const sa = new SnapshotArray(3);
sa.set(0, 5);
const id0 = sa.snap();
sa.set(0, 6);
console.log(id0, sa.get(0, id0), sa.get(0, sa.snap())); // 0 5 6
