/**
 * LeetCode 981 - Time Based Key-Value Store
 * set(key, value, ts) and get(key, ts) returning the value with the largest timestamp <= ts.
 * Timestamps for a key are strictly increasing.
 * Approach: per-key parallel arrays; binary search on get.
 * Time: set O(1), get O(log n). Space: O(n).
 */
export class TimeMap {
  private m = new Map<string, { ts: number[]; vals: string[] }>();

  set(key: string, value: string, ts: number): void {
    if (!this.m.has(key)) this.m.set(key, { ts: [], vals: [] });
    const e = this.m.get(key)!;
    e.ts.push(ts);
    e.vals.push(value);
  }

  get(key: string, ts: number): string {
    const e = this.m.get(key);
    if (!e) return "";
    let lo = 0;
    let hi = e.ts.length - 1;
    let ans = -1;
    while (lo <= hi) {
      const mid = (lo + hi) >> 1;
      if (e.ts[mid] <= ts) {
        ans = mid;
        lo = mid + 1;
      } else hi = mid - 1;
    }
    return ans < 0 ? "" : e.vals[ans];
  }
}

const tm = new TimeMap();
tm.set("foo", "bar", 1);
tm.set("foo", "bar2", 4);
console.log(tm.get("foo", 3), tm.get("foo", 5), JSON.stringify(tm.get("foo", 0))); // bar bar2 ""
