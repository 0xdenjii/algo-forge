/**
 * LeetCode 1472 - Design Browser History
 * Start on a homepage; visit(url) clears forward history; back(steps)/forward(steps)
 * move at most that many pages and return the current url.
 * Approach: array of pages + current index + logical end (forward history is overwritten lazily).
 * Time: O(1) per op. Space: O(visits).
 */
export class BrowserHistory {
  private readonly pages: string[];
  private cur = 0;
  private end = 0;

  constructor(homepage: string) {
    this.pages = [homepage];
  }

  visit(url: string): void {
    this.pages[++this.cur] = url;
    this.end = this.cur;
  }
  back(steps: number): string {
    this.cur = Math.max(0, this.cur - steps);
    return this.pages[this.cur];
  }
  forward(steps: number): string {
    this.cur = Math.min(this.end, this.cur + steps);
    return this.pages[this.cur];
  }
}

const bh = new BrowserHistory("leetcode.com");
bh.visit("google.com");
bh.visit("facebook.com");
bh.visit("youtube.com");
console.log(bh.back(1), bh.back(1), bh.forward(1)); // facebook.com google.com facebook.com
bh.visit("linkedin.com");
console.log(bh.forward(2), bh.back(2), bh.back(7)); // linkedin.com google.com leetcode.com
