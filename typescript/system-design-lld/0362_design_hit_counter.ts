/**
 * LeetCode 362 - Design Hit Counter
 * Count hits received in the past 300 seconds. Timestamps arrive in non-decreasing order.
 * Approach: 300 circular buckets holding (timestamp, count); stale buckets are reset lazily.
 * Time: hit O(1), getHits O(300). Space: O(300).
 */
export class HitCounter {
  private times = new Array<number>(300).fill(0);
  private counts = new Array<number>(300).fill(0);

  hit(ts: number): void {
    const i = ts % 300;
    if (this.times[i] !== ts) {
      this.times[i] = ts;
      this.counts[i] = 0;
    }
    this.counts[i]++;
  }

  getHits(ts: number): number {
    let total = 0;
    for (let i = 0; i < 300; i++) if (ts - this.times[i] < 300) total += this.counts[i];
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
