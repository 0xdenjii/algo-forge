/**
 * LeetCode 341: Flatten Nested List Iterator
 * Problem: Given a nested list of integers, implement an iterator yielding
 *   every integer in order. Elements are integers or lists nested
 *   arbitrarily deep (possibly empty).
 * Approach: Lazy stack of array iterators; hasNext advances until an integer
 *   is buffered. A generator version is shown as the idiomatic alternative.
 * Complexity: amortized O(1) per element; O(depth) space.
 */
type Nested = number | Nested[];

class NestedIterator {
  private stack: Iterator<Nested>[];
  private peeked: number | undefined;
  constructor(list: Nested[]) { this.stack = [list[Symbol.iterator]()]; }

  hasNext(): boolean {
    while (this.peeked === undefined && this.stack.length) {
      const r = this.stack[this.stack.length - 1].next();
      if (r.done) this.stack.pop();
      else if (typeof r.value === "number") this.peeked = r.value;
      else this.stack.push(r.value[Symbol.iterator]());
    }
    return this.peeked !== undefined;
  }

  next(): number {
    if (!this.hasNext()) throw new Error("no more elements");
    const v = this.peeked!;
    this.peeked = undefined;
    return v;
  }
}

function* flatten(list: Nested[]): Generator<number> {
  for (const x of list) {
    if (typeof x === "number") yield x;
    else yield* flatten(x);
  }
}

const input: Nested[] = [[1, 1], 2, [1, [], [[4]]]];
const it = new NestedIterator(input);
const out: number[] = [];
while (it.hasNext()) out.push(it.next());
console.log(out, [...flatten(input)]); // [1,1,2,1,4] twice
