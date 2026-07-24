/**
 * LeetCode 155 - Min Stack
 * Stack supporting push, pop, top and getMin, all in O(1).
 * Approach: store pairs [value, minSoFar].
 * Time: O(1). Space: O(n).
 */
export class MinStack {
  private s: [number, number][] = [];
  push(x: number): void {
    const m = this.s.length ? Math.min(x, this.getMin()) : x;
    this.s.push([x, m]);
  }
  pop(): void {
    this.s.pop();
  }
  top(): number {
    return this.s[this.s.length - 1][0];
  }
  getMin(): number {
    return this.s[this.s.length - 1][1];
  }
}

const ms = new MinStack();
ms.push(-2);
ms.push(0);
ms.push(-3);
console.log(ms.getMin()); // -3
ms.pop();
console.log(ms.top(), ms.getMin()); // 0 -2
