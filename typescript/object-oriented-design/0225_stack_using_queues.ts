/**
 * LeetCode 225 - Implement Stack using Queues
 * Implement a LIFO stack using only queue operations (enqueue at back, dequeue from front).
 * Approach: single queue; after each push rotate the previous n-1 elements behind the new one.
 * Time: push O(n), others O(1). Space: O(n).
 */
class Queue<T> {
  private items: T[] = [];
  private head = 0;
  enqueue(x: T): void {
    this.items.push(x);
  }
  dequeue(): T {
    const v = this.items[this.head++];
    if (this.head > 32 && this.head * 2 > this.items.length) {
      this.items = this.items.slice(this.head);
      this.head = 0;
    }
    return v;
  }
  front(): T {
    return this.items[this.head];
  }
  get size(): number {
    return this.items.length - this.head;
  }
}

export class MyStack {
  private readonly q = new Queue<number>();
  push(x: number): void {
    this.q.enqueue(x);
    for (let i = 1; i < this.q.size; i++) this.q.enqueue(this.q.dequeue());
  }
  pop(): number {
    return this.q.dequeue();
  }
  top(): number {
    return this.q.front();
  }
  empty(): boolean {
    return this.q.size === 0;
  }
}

const st = new MyStack();
st.push(1);
st.push(2);
console.log(st.top(), st.pop(), st.empty()); // 2 2 false
