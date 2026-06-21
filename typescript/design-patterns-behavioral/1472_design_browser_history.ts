/**
 * LeetCode 1472: Design Browser History
 * Problem: Start at a homepage. visit(url) clears forward history;
 *   back(steps) and forward(steps) move at most as far as possible and
 *   return the current url.
 * Approach: Memento-like history array with a cursor and a logical end
 *   index, so visit is O(1) without truncating the array.
 * Complexity: O(1) per operation; O(n) space for n visits.
 */
class BrowserHistory {
  private pages: string[];
  private cur = 0;
  private end = 0; // last valid index

  constructor(homepage: string) { this.pages = [homepage]; }

  visit(url: string): void {
    this.cur++;
    this.pages[this.cur] = url;
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
bh.visit("google.com"); bh.visit("facebook.com"); bh.visit("youtube.com");
console.log(bh.back(1));    // facebook.com
console.log(bh.back(1));    // google.com
console.log(bh.forward(1)); // facebook.com
bh.visit("linkedin.com");
console.log(bh.forward(2)); // linkedin.com
console.log(bh.back(2));    // google.com
console.log(bh.back(7));    // leetcode.com
