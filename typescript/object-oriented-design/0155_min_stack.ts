/**
 * LeetCode 155 - Min Stack
 * Design a stack supporting push, pop, top and getMin, all in O(1).
 * Approach: store pairs (value, min-so-far) so the minimum travels with each frame.
 * Time: O(1) per op. Space: O(n).
 */
export class MinStack {
  private readonly data: Array<[value: number, min: number]> = [];

  push(val: number): void {
    const min = this.data.length ? Math.min(val, this.data[this.data.length - 1][1]) : val;
    this.data.push([val, min]);
  }
  pop(): void {
    this.data.pop();
  }
  top(): number {
    return this.data[this.data.length - 1][0];
  }
  getMin(): number {
    return this.data[this.data.length - 1][1];
  }
}

const s = new MinStack();
s.push(-2);
s.push(0);
s.push(-3);
console.log(s.getMin()); // -3
s.pop();
console.log(s.top(), s.getMin()); // 0 -2
