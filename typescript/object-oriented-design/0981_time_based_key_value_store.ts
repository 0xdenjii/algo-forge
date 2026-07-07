/**
 * LeetCode 981 - Time Based Key-Value Store
 * set(key, value, timestamp) stores a value; get(key, timestamp) returns the value with the
 * largest stored timestamp <= the query, or "" if none. Timestamps per key arrive increasing.
 * Approach: per-key append-only array of (timestamp, value); binary search on get.
 * Time: set O(1), get O(log n). Space: O(n).
 */
export class TimeMap {
  private readonly store = new Map<string, Array<[ts: number, value: string]>>();

  set(key: string, value: string, timestamp: number): void {
    if (!this.store.has(key)) this.store.set(key, []);
    this.store.get(key)!.push([timestamp, value]);
  }

  get(key: string, timestamp: number): string {
    const arr = this.store.get(key);
    if (!arr) return "";
    let lo = 0;
    let hi = arr.length - 1;
    let ans = "";
    while (lo <= hi) {
      const mid = (lo + hi) >> 1;
      if (arr[mid][0] <= timestamp) {
        ans = arr[mid][1];
        lo = mid + 1;
      } else hi = mid - 1;
    }
    return ans;
  }
}

const tm = new TimeMap();
tm.set("foo", "bar", 1);
console.log(tm.get("foo", 1), tm.get("foo", 3)); // bar bar
tm.set("foo", "bar2", 4);
console.log(tm.get("foo", 4), tm.get("foo", 5), JSON.stringify(tm.get("foo", 0))); // bar2 bar2 ""
