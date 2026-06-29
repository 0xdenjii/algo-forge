/**
 * LeetCode 284 - Peeking Iterator
 * Wrap an iterator exposing next()/hasNext() so it also supports peek() without advancing.
 * Approach: cache one look-ahead element; generic over T and built on the native Iterator protocol.
 * Time: O(1) per op. Space: O(1).
 */
export class PeekingIterator<T> implements IterableIterator<T> {
  private buffered: IteratorResult<T>;

  constructor(private readonly source: Iterator<T>) {
    this.buffered = source.next();
  }

  peek(): T {
    if (this.buffered.done) throw new Error("iterator exhausted");
    return this.buffered.value;
  }

  hasNext(): boolean {
    return !this.buffered.done;
  }

  next(): IteratorResult<T> {
    const current = this.buffered;
    if (!current.done) this.buffered = this.source.next();
    return current;
  }

  [Symbol.iterator](): IterableIterator<T> {
    return this;
  }
}

const it = new PeekingIterator([1, 2, 3][Symbol.iterator]());
console.log(it.next().value, it.peek(), it.next().value, it.next().value, it.hasNext()); // 1 2 2 3 false
console.log([...new PeekingIterator("abc"[Symbol.iterator]())]); // ['a','b','c']
