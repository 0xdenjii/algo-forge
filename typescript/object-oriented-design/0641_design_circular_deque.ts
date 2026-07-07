/**
 * LeetCode 641 - Design Circular Deque
 * Fixed-capacity double-ended queue with insert/delete at both ends and front/rear peeks.
 * Approach: ring buffer with head index and size; inserting at the front moves head backward mod k.
 * Time: O(1) per op. Space: O(k).
 */
export class MyCircularDeque {
  private readonly buf: number[];
  private head = 0;
  private size = 0;
  constructor(private readonly k: number) {
    this.buf = new Array<number>(k);
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
    this.buf[(this.head + this.size++) % this.k] = v;
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
console.log(dq.insertLast(1), dq.insertLast(2), dq.insertFront(3), dq.insertFront(4)); // true true true false
console.log(dq.getRear(), dq.isFull(), dq.deleteLast(), dq.insertFront(4), dq.getFront()); // 2 true true true 4
