/**
 * LeetCode 901 - Online Stock Span
 * next(price) returns the span: the number of consecutive days up to today (inclusive)
 * whose price was <= today's price.
 * Approach: monotonic decreasing stack of (price, span); absorb spans of smaller-or-equal prices.
 * Time: amortized O(1) per call. Space: O(n).
 */
export class StockSpanner {
  private readonly stack: Array<[price: number, span: number]> = [];

  next(price: number): number {
    let span = 1;
    while (this.stack.length && this.stack[this.stack.length - 1][0] <= price) {
      span += this.stack.pop()![1];
    }
    this.stack.push([price, span]);
    return span;
  }
}

const sp = new StockSpanner();
console.log([100, 80, 60, 70, 60, 75, 85].map((p) => sp.next(p))); // [1, 1, 1, 2, 1, 4, 6]
