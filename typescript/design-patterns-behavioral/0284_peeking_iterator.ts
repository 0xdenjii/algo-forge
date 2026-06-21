/**
 * LeetCode 284: Peeking Iterator
 * Problem: Wrap an existing iterator (with next/hasNext) so it also supports
 *   peek(), returning the next element without advancing.
 * Approach: Decorate the iterator, caching one look-ahead element. Generic
 *   over T and adaptable from any JS Iterable.
 * Complexity: O(1) per operation, O(1) extra space.
 */
interface SimpleIterator<T> { hasNext(): boolean; next(): T }

function fromIterable<T>(src: Iterable<T>): SimpleIterator<T> {
  const it = src[Symbol.iterator]();
  let cur = it.next();
  return {
    hasNext: () => !cur.done,
    next: () => {
      if (cur.done) throw new Error("exhausted");
      const v = cur.value;
      cur = it.next();
      return v;
    },
  };
}

class PeekingIterator<T> implements SimpleIterator<T> {
  private buffered = false;
  private head!: T;
  constructor(private inner: SimpleIterator<T>) {}

  peek(): T {
    if (!this.buffered) { this.head = this.inner.next(); this.buffered = true; }
    return this.head;
  }
  next(): T {
    if (this.buffered) { this.buffered = false; return this.head; }
    return this.inner.next();
  }
  hasNext(): boolean { return this.buffered || this.inner.hasNext(); }
}

const p = new PeekingIterator(fromIterable([1, 2, 3]));
console.log(p.next(), p.peek(), p.next(), p.next(), p.hasNext()); // 1 2 2 3 false

// Classic use: run-length grouping requires look-ahead.
const q = new PeekingIterator(fromIterable("aaabccdd"));
const groups: string[] = [];
while (q.hasNext()) {
  const c = q.next(); let n = 1;
  while (q.hasNext() && q.peek() === c) { q.next(); n++; }
  groups.push(`${c}${n}`);
}
console.log(groups.join("")); // a3b1c2d2
