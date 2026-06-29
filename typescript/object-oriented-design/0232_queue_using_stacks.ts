/**
 * LeetCode 232 - Implement Queue using Stacks
 * Implement a FIFO queue (push, pop, peek, empty) using only two stacks.
 * Approach: push onto `inbox`; when `outbox` is empty, pour inbox into it, reversing order.
 * Time: amortized O(1) per op. Space: O(n).
 */
export class MyQueue {
  private readonly inbox: number[] = [];
  private readonly outbox: number[] = [];

  push(x: number): void {
    this.inbox.push(x);
  }
  private refill(): void {
    if (this.outbox.length === 0) {
      while (this.inbox.length) this.outbox.push(this.inbox.pop()!);
    }
  }
  pop(): number {
    this.refill();
    return this.outbox.pop()!;
  }
  peek(): number {
    this.refill();
    return this.outbox[this.outbox.length - 1];
  }
  empty(): boolean {
    return this.inbox.length === 0 && this.outbox.length === 0;
  }
}

const q = new MyQueue();
q.push(1);
q.push(2);
console.log(q.peek(), q.pop(), q.empty()); // 1 1 false
