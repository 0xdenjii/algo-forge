/**
 * LeetCode 362 - Design Hit Counter
 * hit(timestamp) records a hit; getHits(timestamp) returns hits in the past 300 seconds
 * (timestamps are non-decreasing, in seconds).
 * Approach: 300-slot circular buffer keyed by timestamp % 300, storing (time, count) per slot.
 * Time: hit O(1), getHits O(300). Space: O(300), independent of hit volume.
 */
export class HitCounter {
  private static readonly WINDOW = 300;
  private readonly times = new Array<number>(HitCounter.WINDOW).fill(0);
  private readonly counts = new Array<number>(HitCounter.WINDOW).fill(0);

  hit(timestamp: number): void {
    const i = timestamp % HitCounter.WINDOW;
    if (this.times[i] !== timestamp) {
      this.times[i] = timestamp;
      this.counts[i] = 0;
    }
    this.counts[i]++;
  }

  getHits(timestamp: number): number {
    let total = 0;
    for (let i = 0; i < HitCounter.WINDOW; i++) {
      if (timestamp - this.times[i] < HitCounter.WINDOW) total += this.counts[i];
    }
    return total;
  }
}

const hc = new HitCounter();
hc.hit(1);
hc.hit(2);
hc.hit(3);
console.log(hc.getHits(4)); // 3
hc.hit(300);
console.log(hc.getHits(300), hc.getHits(301)); // 4 3
