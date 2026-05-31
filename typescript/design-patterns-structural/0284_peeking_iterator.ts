/**
 * Problem 284: Peeking Iterator
 * Problem: Given an iterator with next()/hasNext(), design a PeekingIterator that also
 *   supports peek() - returning the next element without advancing. A classic Decorator:
 *   it wraps an existing iterator and adds behaviour while keeping its interface.
 * Approach: Cache one look-ahead element.
 * Complexity: O(1) per operation, O(1) space.
 */
interface BasicIterator<T> { hasNext(): boolean; next(): T }

class ArrayIterator<T> implements BasicIterator<T> {
  private i = 0;
  constructor(private arr: T[]) {}
  hasNext(): boolean { return this.i < this.arr.length; }
  next(): T { return this.arr[this.i++]; }
}

class PeekingIterator<T> implements BasicIterator<T> {
  private peeked = false;
  private cached!: T;
  constructor(private inner: BasicIterator<T>) {}
  peek(): T {
    if (!this.peeked) {
      if (!this.inner.hasNext()) throw new Error("exhausted");
      this.cached = this.inner.next();
      this.peeked = true;
    }
    return this.cached;
  }
  next(): T {
    const v = this.peek();
    this.peeked = false;
    return v;
  }
  hasNext(): boolean { return this.peeked || this.inner.hasNext(); }
}

const p = new PeekingIterator(new ArrayIterator([1, 2, 3]));
console.log(p.next(), p.peek(), p.next(), p.next(), p.hasNext()); // 1 2 2 3 false
