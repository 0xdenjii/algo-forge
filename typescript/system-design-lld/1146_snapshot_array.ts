/**
 * LeetCode 1146 - Snapshot Array
 * Array supporting set(index, val), snap() -> snapId, and get(index, snapId).
 * Approach: per-index history of [snapId, val]; binary search for the last entry <= snapId.
 * Time: set O(1), snap O(1), get O(log h). Space: O(n + number of sets).
 */
export class SnapshotArray {
  private hist: [number, number][][];
  private snapId = 0;

  constructor(length: number) {
    this.hist = Array.from({ length }, () => [[0, 0]] as [number, number][]);
  }

  set(i: number, v: number): void {
    const h = this.hist[i];
    const last = h[h.length - 1];
    if (last[0] === this.snapId) last[1] = v;
    else h.push([this.snapId, v]);
  }

  snap(): number {
    return this.snapId++;
  }

  get(i: number, sid: number): number {
    const h = this.hist[i];
    let lo = 0;
    let hi = h.length - 1;
    while (lo < hi) {
      const mid = (lo + hi + 1) >> 1;
      if (h[mid][0] <= sid) lo = mid;
      else hi = mid - 1;
    }
    return h[lo][1];
  }
}

const sa = new SnapshotArray(3);
sa.set(0, 5);
const sid = sa.snap();
sa.set(0, 6);
console.log(sa.get(0, sid)); // 5
