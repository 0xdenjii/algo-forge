/**
 * LeetCode 729 - My Calendar I
 * book(start, end) adds the half-open event [start, end) if it does not overlap any
 * existing booking, returning true; otherwise returns false and books nothing.
 * Approach: keep bookings sorted by start; binary search the insertion point and check both neighbours.
 * Time: O(log n) search + O(n) array insert. Space: O(n).
 */
export class MyCalendar {
  private readonly events: Array<[start: number, end: number]> = [];

  book(start: number, end: number): boolean {
    let lo = 0;
    let hi = this.events.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (this.events[mid][0] < start) lo = mid + 1;
      else hi = mid;
    }
    const prev = this.events[lo - 1];
    const next = this.events[lo];
    if (prev && prev[1] > start) return false;
    if (next && next[0] < end) return false;
    this.events.splice(lo, 0, [start, end]);
    return true;
  }
}

const cal = new MyCalendar();
console.log(cal.book(10, 20), cal.book(15, 25), cal.book(20, 30)); // true false true
