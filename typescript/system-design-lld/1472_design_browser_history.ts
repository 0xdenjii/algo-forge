/**
 * LeetCode 1472 - Design Browser History
 * visit(url) clears forward history; back(steps) / forward(steps) move and return the current url.
 * Approach: array with a current index and a logical end index (no truncation needed).
 * Time: O(1) per op. Space: O(n).
 */
export class BrowserHistory {
  private pages: string[];
  private cur = 0;
  private end = 0;
  constructor(home: string) {
    this.pages = [home];
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
console.log(bh.back(1), bh.back(1), bh.forward(1)); // facebook google facebook
bh.visit("linkedin.com");
console.log(bh.forward(2), bh.back(2), bh.back(7)); // linkedin google leetcode
