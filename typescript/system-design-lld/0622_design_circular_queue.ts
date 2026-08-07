/**
 * LeetCode 622 - Design Circular Queue
 * Fixed-size FIFO queue with enQueue, deQueue, Front, Rear, isEmpty, isFull.
 * Approach: ring buffer with head index and element count.
 * Time: O(1). Space: O(k).
 */
export class MyCircularQueue {
  private buf: number[];
  private head = 0;
  private n = 0;
  constructor(private readonly k: number) {
    this.buf = new Array(k);
  }
  enQueue(v: number): boolean {
    if (this.isFull()) return false;
    this.buf[(this.head + this.n++) % this.k] = v;
    return true;
  }
  deQueue(): boolean {
    if (this.isEmpty()) return false;
    this.head = (this.head + 1) % this.k;
    this.n--;
    return true;
  }
  Front(): number {
    return this.isEmpty() ? -1 : this.buf[this.head];
  }
  Rear(): number {
    return this.isEmpty() ? -1 : this.buf[(this.head + this.n - 1) % this.k];
  }
  isEmpty(): boolean {
    return this.n === 0;
  }
  isFull(): boolean {
    return this.n === this.k;
  }
}

const q = new MyCircularQueue(3);
[1, 2, 3, 4].forEach(x => q.enQueue(x));
console.log(q.Rear()); // 3
q.deQueue();
q.enQueue(4);
console.log(q.Rear()); // 4
