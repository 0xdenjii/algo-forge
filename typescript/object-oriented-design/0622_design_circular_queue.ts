/**
 * LeetCode 622 - Design Circular Queue
 * Fixed-capacity ring buffer: enQueue, deQueue, Front, Rear, isEmpty, isFull.
 * Approach: array + head index + size; tail = (head + size - 1) mod k.
 * Time: O(1) per op. Space: O(k).
 */
export class MyCircularQueue {
  private readonly buf: number[];
  private head = 0;
  private size = 0;
  constructor(private readonly k: number) {
    this.buf = new Array<number>(k);
  }

  enQueue(v: number): boolean {
    if (this.isFull()) return false;
    this.buf[(this.head + this.size++) % this.k] = v;
    return true;
  }
  deQueue(): boolean {
    if (this.isEmpty()) return false;
    this.head = (this.head + 1) % this.k;
    this.size--;
    return true;
  }
  Front(): number {
    return this.isEmpty() ? -1 : this.buf[this.head];
  }
  Rear(): number {
    return this.isEmpty() ? -1 : this.buf[(this.head + this.size - 1) % this.k];
  }
  isEmpty(): boolean {
    return this.size === 0;
  }
  isFull(): boolean {
    return this.size === this.k;
  }
}

const cq = new MyCircularQueue(3);
console.log(cq.enQueue(1), cq.enQueue(2), cq.enQueue(3), cq.enQueue(4)); // true true true false
console.log(cq.Rear(), cq.isFull(), cq.deQueue(), cq.enQueue(4), cq.Rear()); // 3 true true true 4
