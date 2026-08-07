/**
 * LeetCode 641 - Design Circular Deque
 * Fixed-capacity deque with insert/delete at both ends plus getFront/getRear.
 * Approach: ring buffer with head index and size.
 * Time: O(1). Space: O(k).
 */
export class MyCircularDeque {
  private buf: number[];
  private head = 0;
  private size = 0;
  constructor(private readonly k: number) {
    this.buf = new Array(k);
  }
  insertFront(v: number): boolean {
    if (this.isFull()) return false;
    this.head = (this.head - 1 + this.k) % this.k;
    this.buf[this.head] = v;
    this.size++;
    return true;
  }
  insertLast(v: number): boolean {
    if (this.isFull()) return false;
    this.buf[(this.head + this.size) % this.k] = v;
    this.size++;
    return true;
  }
  deleteFront(): boolean {
    if (this.isEmpty()) return false;
    this.head = (this.head + 1) % this.k;
    this.size--;
    return true;
  }
  deleteLast(): boolean {
    if (this.isEmpty()) return false;
    this.size--;
    return true;
  }
  getFront(): number {
    return this.isEmpty() ? -1 : this.buf[this.head];
  }
  getRear(): number {
    return this.isEmpty() ? -1 : this.buf[(this.head + this.size - 1) % this.k];
  }
  isEmpty(): boolean {
    return this.size === 0;
  }
  isFull(): boolean {
    return this.size === this.k;
  }
}

const dq = new MyCircularDeque(3);
dq.insertLast(1);
dq.insertLast(2);
dq.insertFront(3);
console.log(dq.insertFront(4), dq.getRear()); // false 2
