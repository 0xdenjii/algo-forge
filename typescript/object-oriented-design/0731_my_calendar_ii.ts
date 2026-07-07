/**
 * LeetCode 731 - My Calendar II
 * book(start, end) adds [start, end) unless it would cause a triple booking
 * (some instant covered by three events). Return whether the event was added.
 * Approach: keep all bookings and the list of pairwise overlaps; reject if the new event hits an overlap.
 * Time: O(n) per booking. Space: O(n).
 */
type Interval = [start: number, end: number];

export class MyCalendarTwo {
  private readonly booked: Interval[] = [];
  private readonly overlaps: Interval[] = [];

  book(start: number, end: number): boolean {
    for (const [s, e] of this.overlaps) if (start < e && s < end) return false;
    for (const [s, e] of this.booked) {
      if (start < e && s < end) this.overlaps.push([Math.max(s, start), Math.min(e, end)]);
    }
    this.booked.push([start, end]);
    return true;
  }
}

const c2 = new MyCalendarTwo();
console.log(
  c2.book(10, 20), // true
  c2.book(50, 60), // true
  c2.book(10, 40), // true
  c2.book(5, 15), // false
  c2.book(5, 10), // true
  c2.book(25, 55), // true
);
