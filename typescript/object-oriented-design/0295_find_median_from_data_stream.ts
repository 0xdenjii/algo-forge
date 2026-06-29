/**
 * LeetCode 295 - Find Median from Data Stream
 * Support addNum(num) and findMedian() over a growing stream of integers.
 * Approach: max-heap for the lower half, min-heap for the upper half, sizes differ by at most 1.
 * Time: addNum O(log n), findMedian O(1). Space: O(n).
 */
class Heap<T> {
  private readonly a: T[] = [];
  constructor(private readonly less: (x: T, y: T) => boolean) {}
  get size(): number {
    return this.a.length;
  }
  peek(): T | undefined {
    return this.a[0];
  }
  push(v: T): void {
    const a = this.a;
    a.push(v);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!this.less(a[i], a[p])) break;
      [a[i], a[p]] = [a[p], a[i]];
      i = p;
    }
  }
  pop(): T | undefined {
    const a = this.a;
    if (a.length === 0) return undefined;
    const top = a[0];
    const last = a.pop()!;
    if (a.length) {
      a[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < a.length && this.less(a[l], a[m])) m = l;
        if (r < a.length && this.less(a[r], a[m])) m = r;
        if (m === i) break;
        [a[i], a[m]] = [a[m], a[i]];
        i = m;
      }
    }
    return top;
  }
}

export class MedianFinder {
  private readonly lo = new Heap<number>((x, y) => x > y);
  private readonly hi = new Heap<number>((x, y) => x < y);

  addNum(num: number): void {
    this.lo.push(num);
    this.hi.push(this.lo.pop()!);
    if (this.hi.size > this.lo.size) this.lo.push(this.hi.pop()!);
  }
  findMedian(): number {
    return this.lo.size > this.hi.size ? this.lo.peek()! : (this.lo.peek()! + this.hi.peek()!) / 2;
  }
}

const mf = new MedianFinder();
mf.addNum(1);
mf.addNum(2);
console.log(mf.findMedian()); // 1.5
mf.addNum(3);
console.log(mf.findMedian()); // 2
